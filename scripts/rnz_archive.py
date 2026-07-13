#!/usr/bin/env python3
"""Zero-cost, resumable RNZ audio archive control plane."""

from __future__ import annotations

import argparse
import hashlib
import html
import html.parser
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


FORBIDDEN_WORKFLOW_PATTERNS = {
    "paid GitHub runner": re.compile(r"runs-on:\s*[^\n]*(?:gpu|larger|4-core|8-core|16-core|32-core|64-core)", re.I),
    "Hugging Face Jobs": re.compile(r"(?:hf\s+jobs|run_job|run_uv_job|create_scheduled_job)", re.I),
    "Hugging Face endpoint": re.compile(r"(?:hf\s+endpoints|InferenceEndpoint|create_inference_endpoint)", re.I),
    "Hugging Face bucket": re.compile(r"(?:hf\s+buckets|storage[_ -]?bucket)", re.I),
    "paid cloud credential": re.compile(r"(?:AWS_ACCESS_KEY_ID|GOOGLE_APPLICATION_CREDENTIALS|AZURE_CLIENT_SECRET)"),
}
ALLOWED_RUNNER = re.compile(r"^\s*runs-on:\s*ubuntu-latest\s*(?:#.*)?$", re.M)
MEDIA_EXTENSIONS = (".mp3", ".m4a", ".mp4", ".ogg", ".oga", ".wav", ".flac")


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def canonical_host(url: str) -> str:
    parsed = urllib.parse.urlparse(url)
    if parsed.scheme.lower() != "https" or not parsed.hostname:
        raise ValueError("URL must use HTTPS and include a host")
    return parsed.hostname.rstrip(".").lower()


def host_allowed(url: str, allowed_domains: Iterable[str]) -> bool:
    host = canonical_host(url)
    domains = {domain.rstrip(".").lower() for domain in allowed_domains}
    return any(host == domain or host.endswith(f".{domain}") for domain in domains)


def append_event(path: Path, event: dict[str, Any]) -> dict[str, Any]:
    payload = dict(event)
    payload["timestamp"] = utc_now()
    payload["event_id"] = uuid.uuid4().hex
    required = ("record_id", "event", "rights_basis")
    missing = [key for key in required if not payload.get(key)]
    if missing:
        raise ValueError(f"manifest event missing: {', '.join(missing)}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(payload, sort_keys=True, ensure_ascii=False) + "\n")
    return payload


def read_events(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    events = []
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as exc:
            raise ValueError(f"invalid manifest JSON on line {number}") from exc
    return events


def latest_by_record(events: Iterable[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    latest: dict[str, dict[str, Any]] = {}
    for event in events:
        latest[str(event["record_id"])] = event
    return latest


def validate_zero_cost(workflows: Path, expected_hf_repo: str) -> list[str]:
    errors: list[str] = []
    for path in sorted(workflows.glob("*.y*ml")):
        text = path.read_text(encoding="utf-8")
        if "rnz" in path.name.lower():
            runners = re.findall(r"^\s*runs-on:\s*([^#\n]+)", text, re.M)
            if not runners or any(value.strip() != "ubuntu-latest" for value in runners):
                errors.append(f"{path}: RNZ jobs must use ubuntu-latest")
        for label, pattern in FORBIDDEN_WORKFLOW_PATTERNS.items():
            if pattern.search(text):
                errors.append(f"{path}: forbidden {label}")
        if re.search(r"HF_REPO_ID:.*\$\{\{", text) and expected_hf_repo not in text:
            errors.append(f"{path}: Hugging Face destination must default to {expected_hf_repo}")
    return errors


class MediaHTMLParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.urls: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if tag in {"audio", "source", "a"} and values.get("src"):
            self.urls.append(str(values["src"]))
        if tag == "a" and values.get("href"):
            self.urls.append(str(values["href"]))
        if tag == "meta" and values.get("property") in {"og:audio", "og:audio:url", "twitter:player:stream"}:
            if values.get("content"):
                self.urls.append(str(values["content"]))


def media_candidates(record: dict[str, Any]) -> list[str]:
    candidates: list[str] = []
    preferred_keys = ("object_url", "source_url", "landing_url", "url")
    for key in preferred_keys:
        value = record.get(key)
        if isinstance(value, str) and value.startswith("https://"):
            candidates.append(value)
        elif isinstance(value, list):
            candidates.extend(item for item in value if isinstance(item, str) and item.startswith("https://"))
    return list(dict.fromkeys(candidates))


def extract_matching_audio_url(source_url: str, body: str) -> str | None:
    match = re.search(r"/audio/(\d+)(?:/|$)", urllib.parse.urlparse(source_url).path)
    if not match:
        return None
    audio_id = re.escape(match.group(1))
    decoded = html.unescape(body)
    media = re.search(
        rf'["\\]id["\\]?\s*:\s*{audio_id}.*?["\\]audioSrc["\\]?\s*:\s*["\\](https:[^"\\]+)',
        decoded,
        re.DOTALL,
    )
    return media.group(1).replace("\\/", "/") if media else None


def fetch_json(
    url: str,
    params: dict[str, Any],
    timeout: int = 60,
    headers: dict[str, str] | None = None,
) -> dict[str, Any]:
    encoded = urllib.parse.urlencode(params, doseq=True)
    request_headers = {"User-Agent": "dnz-rnz-archive/1", **(headers or {})}
    request = urllib.request.Request(f"{url}?{encoded}", headers=request_headers)
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return json.load(response)


def discover(args: argparse.Namespace) -> int:
    policy = load_json(args.policy)
    if not policy.get("rights_authorized"):
        raise RuntimeError("rights_authorized must be true before discovery")
    api_key = args.api_key or os.environ.get("DIGITALNZ_API_KEY")
    existing = latest_by_record(read_events(args.manifest))
    remaining = args.limit
    collections = policy["collections"]
    for collection in collections:
        if remaining <= 0:
            break
        params: dict[str, Any] = {
            "text": args.query or "*",
            "and[primary_collection][]": collection,
            "and[category][]": "Audio",
            "per_page": min(100, remaining),
            "page": 1,
        }
        headers = {"Authentication-Token": api_key} if api_key else None
        payload = fetch_json(
            "https://api.digitalnz.org/v3/records.json",
            params,
            headers=headers,
        )
        records = payload.get("search", {}).get("results", [])
        for record in records:
            record_id = str(record.get("id", "")).strip()
            if not record_id or record_id in existing:
                continue
            candidates = media_candidates(record)
            source_url = next((url for url in candidates if host_allowed(url, policy["allowed_media_domains"])), None)
            if not source_url:
                continue
            append_event(
                args.manifest,
                {
                    "record_id": record_id,
                    "event": "discovered",
                    "rights_basis": policy["rights_basis"],
                    "source_url": source_url,
                    "collection": collection,
                    "title": record.get("title"),
                    "retry_count": 0,
                },
            )
            remaining -= 1
            if remaining <= 0:
                break
    return 0


def resolve_media_url(source_url: str, policy: dict[str, Any]) -> str:
    if not host_allowed(source_url, policy["allowed_media_domains"]):
        raise ValueError("source host is not allowlisted")
    if urllib.parse.urlparse(source_url).path.lower().endswith(MEDIA_EXTENSIONS):
        return source_url
    request = urllib.request.Request(source_url, headers={"User-Agent": "dnz-rnz-archive/1"})
    with urllib.request.urlopen(request, timeout=60) as response:
        final_url = response.geturl()
        if not host_allowed(final_url, policy["allowed_media_domains"]):
            raise ValueError("landing-page redirect left the allowlist")
        content_type = response.headers.get_content_type()
        if content_type in policy["allowed_content_types"]:
            return final_url
        body = response.read(5_000_000).decode("utf-8", errors="ignore")
    matching = extract_matching_audio_url(source_url, body)
    if matching and host_allowed(matching, policy["allowed_media_domains"]):
        return matching
    if re.search(r"/audio/\d+(?:/|$)", urllib.parse.urlparse(source_url).path):
        raise ValueError("landing page does not contain media for the requested audio ID")
    parser = MediaHTMLParser()
    parser.feed(body)
    for candidate in parser.urls:
        candidate = urllib.parse.urljoin(final_url, candidate)
        if candidate.lower().split("?", 1)[0].endswith(MEDIA_EXTENSIONS) and host_allowed(
            candidate, policy["allowed_media_domains"]
        ):
            return candidate
    raise ValueError("no allowlisted media URL found")


def download_media(url: str, destination: Path, policy: dict[str, Any]) -> tuple[str, int, str]:
    if not host_allowed(url, policy["allowed_media_domains"]):
        raise ValueError("media host is not allowlisted")
    request = urllib.request.Request(url, headers={"User-Agent": "dnz-rnz-archive/1"})
    digest = hashlib.sha256()
    size = 0
    destination.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(request, timeout=120) as response, destination.open("wb") as handle:
        final_url = response.geturl()
        if not host_allowed(final_url, policy["allowed_media_domains"]):
            raise ValueError("media redirect left the allowlist")
        content_type = response.headers.get_content_type()
        if content_type not in policy["allowed_content_types"]:
            raise ValueError(f"unsupported content type: {content_type}")
        for chunk in iter(lambda: response.read(1024 * 1024), b""):
            size += len(chunk)
            if size > int(policy["max_download_bytes"]):
                raise ValueError("media exceeds maximum download size")
            digest.update(chunk)
            handle.write(chunk)
    return digest.hexdigest(), size, content_type


def ffprobe_duration(path: Path) -> float:
    completed = subprocess.run(
        ["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "json", str(path)],
        check=True,
        capture_output=True,
        text=True,
    )
    return float(json.loads(completed.stdout)["format"]["duration"])


def normalize_audio(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["ffmpeg", "-nostdin", "-y", "-i", str(source), "-ac", "1", "-ar", "16000", "-c:a", "flac", str(destination)],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def seconds_timestamp(value: float, separator: str = ",") -> str:
    millis = max(0, round(value * 1000))
    hours, remainder = divmod(millis, 3_600_000)
    minutes, remainder = divmod(remainder, 60_000)
    seconds, millis = divmod(remainder, 1000)
    return f"{hours:02d}:{minutes:02d}:{seconds:02d}{separator}{millis:03d}"


def write_caption_files(segments: list[dict[str, Any]], output_dir: Path) -> dict[str, str]:
    srt: list[str] = []
    vtt = ["WEBVTT", ""]
    plain: list[str] = []
    for index, segment in enumerate(segments, 1):
        start, end = float(segment["start"]), float(segment["end"])
        speaker = segment.get("speaker", "SPEAKER_UNKNOWN")
        text = str(segment.get("text", "")).strip()
        line = f"[{speaker}] {text}"
        srt.extend([str(index), f"{seconds_timestamp(start)} --> {seconds_timestamp(end)}", line, ""])
        vtt.extend([f"{seconds_timestamp(start, '.')} --> {seconds_timestamp(end, '.')}", line, ""])
        plain.append(line)
    paths = {"srt": output_dir / "transcript.srt", "vtt": output_dir / "transcript.vtt", "text": output_dir / "transcript.txt"}
    paths["srt"].write_text("\n".join(srt), encoding="utf-8")
    paths["vtt"].write_text("\n".join(vtt), encoding="utf-8")
    paths["text"].write_text("\n".join(plain) + "\n", encoding="utf-8")
    return {key: value.name for key, value in paths.items()}


def rttm_lines(recording_id: str, rows: Iterable[dict[str, Any]]) -> list[str]:
    lines = []
    for row in rows:
        start = float(row["start"])
        duration = float(row["end"]) - start
        speaker = str(row["speaker"])
        lines.append(
            f"SPEAKER {recording_id} 1 {start:.3f} {duration:.3f} <NA> <NA> {speaker} <NA> <NA>"
        )
    return lines


def transcribe(audio: Path, output_dir: Path, policy: dict[str, Any], hf_token: str) -> dict[str, Any]:
    import whisperx
    from whisperx.diarize import DiarizationPipeline

    models = policy["models"]
    device = "cpu"
    model = whisperx.load_model(models["transcription_primary"], device, compute_type="int8", language=None)
    raw = model.transcribe(str(audio), batch_size=1)
    language = raw.get("language", "unknown")
    quality_flags: list[str] = []
    try:
        align_model, metadata = whisperx.load_align_model(language_code=language, device=device)
        aligned = whisperx.align(raw["segments"], align_model, metadata, str(audio), device, return_char_alignments=False)
    except Exception:
        aligned = raw
        quality_flags.append("alignment_unavailable")
    if not hf_token:
        raise RuntimeError("HF_TOKEN with gated pyannote model access is required")
    try:
        diarizer = DiarizationPipeline(
            model_name=models["diarization"], token=hf_token, device=device
        )
        diarization = diarizer(str(audio))
        aligned = whisperx.assign_word_speakers(diarization, aligned)
        rows = diarization[["start", "end", "speaker"]].to_dict(orient="records")
        (output_dir / "diarization.rttm").write_text(
            "\n".join(rttm_lines(audio.stem, rows)) + "\n", encoding="utf-8"
        )
    except Exception as exc:
        raise RuntimeError(f"speaker diarization failed: {exc}") from exc
    result = {
        "language": language,
        "segments": aligned.get("segments", []),
        "word_segments": aligned.get("word_segments", []),
        "models": models,
        "quality_flags": quality_flags,
        "generated_at": utc_now(),
    }
    (output_dir / "transcript.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    result["outputs"] = write_caption_files(result["segments"], output_dir)
    return result


def process(args: argparse.Namespace) -> int:
    policy = load_json(args.policy)
    events = read_events(args.manifest)
    latest = latest_by_record(events)
    pending = [event for event in latest.values() if event["event"] in {"discovered", "retry"}]
    deadline = time.monotonic() + min(args.deadline_minutes, int(policy["deadline_minutes"])) * 60
    selected = pending[: min(args.max_items, int(policy["max_items_per_run"]))]
    hf_token = os.environ.get("HF_TOKEN", "")
    for item in selected:
        if time.monotonic() >= deadline:
            break
        record_id = str(item["record_id"])
        output_dir = args.output / datetime.now(timezone.utc).strftime("%Y-%m") / "items" / record_id
        output_dir.mkdir(parents=True, exist_ok=True)
        source_path = output_dir / "source.media"
        normalized = output_dir / "audio.flac"
        try:
            media_url = resolve_media_url(item["source_url"], policy)
            sha256, size, content_type = download_media(media_url, source_path, policy)
            duration = ffprobe_duration(source_path)
            if duration > float(policy["max_duration_seconds"]):
                raise ValueError("media exceeds maximum duration")
            normalize_audio(source_path, normalized)
            append_event(args.manifest, {**item, "event": "downloaded", "media_url": media_url, "sha256": sha256, "size_bytes": size, "content_type": content_type, "duration_seconds": duration})
            result = transcribe(normalized, output_dir, policy, hf_token)
            provenance = {"record_id": record_id, "source_url": item["source_url"], "media_url": media_url, "sha256": sha256, "rights_basis": policy["rights_basis"], "models": policy["models"], "generated_at": utc_now()}
            (output_dir / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
            append_event(args.manifest, {**item, "event": "processed", "media_url": media_url, "sha256": sha256, "size_bytes": size, "duration_seconds": duration, "models": policy["models"], "outputs": result["outputs"], "quality_flags": result["quality_flags"]})
        except Exception as exc:
            source_path.unlink(missing_ok=True)
            count = int(item.get("retry_count", 0)) + 1
            event = "retry" if count <= args.max_retries else "rejected"
            reason = str(exc)[:500]
            print(f"record {record_id} {event}: {reason}", file=sys.stderr, flush=True)
            append_event(args.manifest, {**item, "event": event, "retry_count": count, "reason": reason})
    return 0


def package(args: argparse.Namespace) -> int:
    events = read_events(args.manifest)
    packaged_ids = {path.parent.name for path in args.items.rglob("provenance.json")}
    if not packaged_ids:
        raise RuntimeError("refusing to publish an empty archive shard")

    import pyarrow as pa
    import pyarrow.parquet as pq

    processed = [
        event
        for event in latest_by_record(events).values()
        if event.get("event") == "processed" and event.get("record_id") in packaged_ids
    ]
    release_events = [event for event in events if event.get("record_id") in packaged_ids]
    args.output.mkdir(parents=True, exist_ok=True)
    shard_id = re.sub(r"[^A-Za-z0-9_.-]", "-", args.shard_id)
    pq.write_table(pa.Table.from_pylist(processed), args.output / f"manifest-{shard_id}.parquet")
    (args.output / f"manifest-{shard_id}.jsonl").write_text("".join(json.dumps(event, sort_keys=True, ensure_ascii=False) + "\n" for event in release_events), encoding="utf-8")
    shard = args.output / f"audio-{shard_id}.tar"
    with tarfile.open(shard, "w") as archive:
        for path in sorted(args.items.rglob("*")):
            if path.is_file():
                archive.add(path, arcname=path.relative_to(args.items))
    checksums = []
    for path in sorted(args.output.iterdir()):
        if path.is_file():
            checksums.append(f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}")
    (args.output / f"SHA256SUMS-{shard_id}").write_text("\n".join(checksums) + "\n", encoding="utf-8")
    return 0


def summary(args: argparse.Namespace) -> int:
    events = read_events(args.manifest)
    latest = latest_by_record(events)
    counts: dict[str, int] = {}
    for event in latest.values():
        counts[event["event"]] = counts.get(event["event"], 0) + 1
    payload = {"timestamp": utc_now(), "records": len(latest), "states": counts}
    print(json.dumps(payload, indent=2, sort_keys=True))
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a", encoding="utf-8") as handle:
            handle.write("## RNZ archive summary\n\n")
            handle.write(f"- Records: {len(latest)}\n")
            for state, count in sorted(counts.items()):
                handle.write(f"- {state}: {count}\n")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--policy", type=Path, default=Path("rnz/archive-policy.json"))
    subparsers = parser.add_subparsers(dest="command", required=True)

    policy_parser = subparsers.add_parser("policy")
    policy_parser.add_argument("--workflows", type=Path, default=Path(".github/workflows"))
    policy_parser.set_defaults(func=lambda args: _policy_command(args))

    discover_parser = subparsers.add_parser("discover")
    discover_parser.add_argument("--manifest", type=Path, required=True)
    discover_parser.add_argument("--limit", type=int, default=100)
    discover_parser.add_argument("--query")
    discover_parser.add_argument("--api-key")
    discover_parser.set_defaults(func=discover)

    process_parser = subparsers.add_parser("process")
    process_parser.add_argument("--manifest", type=Path, required=True)
    process_parser.add_argument("--output", type=Path, required=True)
    process_parser.add_argument("--max-items", type=int, default=5)
    process_parser.add_argument("--max-retries", type=int, default=3)
    process_parser.add_argument("--deadline-minutes", type=int, default=330)
    process_parser.set_defaults(func=process)

    package_parser = subparsers.add_parser("package")
    package_parser.add_argument("--manifest", type=Path, required=True)
    package_parser.add_argument("--items", type=Path, required=True)
    package_parser.add_argument("--output", type=Path, required=True)
    package_parser.add_argument("--shard-id", required=True)
    package_parser.set_defaults(func=package)

    summary_parser = subparsers.add_parser("summary")
    summary_parser.add_argument("--manifest", type=Path, required=True)
    summary_parser.set_defaults(func=summary)

    args = parser.parse_args(argv)
    return int(args.func(args))


def _policy_command(args: argparse.Namespace) -> int:
    policy = load_json(args.policy)
    errors = validate_zero_cost(args.workflows, policy["hf_repo_id"])
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(json.dumps({"zero_cost_policy": "pass", "hf_repo_id": policy["hf_repo_id"]}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
