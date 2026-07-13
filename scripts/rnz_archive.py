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
TOPIC_TERMS = {
    "parliament_and_politics": ("parliament", "minister", "government", "election", "mp ", "political party"),
    "te_tiriti_and_maori_affairs": ("te tiriti", "waitangi", "iwi", "hapu", "hapū", "marae", "maori", "māori"),
    "health": ("health", "hospital", "patient", "doctor", "nurse", "covid"),
    "housing": ("housing", "rent", "tenant", "homeless", "mortgage"),
    "justice_and_law": ("court", "judge", "law", "police", "justice", "sentence"),
    "environment_and_climate": ("climate", "environment", "emissions", "conservation", "flood", "drought"),
    "foreign_affairs_and_pacific": ("pacific", "foreign affairs", "diplomat", "samoa", "tonga", "fiji", "vanuatu"),
}
SECTION_TERMS = {
    "headlines": ("headlines", "coming up", "in the news"),
    "weather": ("weather forecast", "the forecast", "degrees", "showers"),
    "interview": ("joining us", "my guest", "welcome to the programme"),
    "credits": ("produced by", "executive producer", "you have been listening"),
}
SENSITIVE_REVIEW_TERMS = {
    "children_or_minors": (" child ", " children ", " minor ", " school student "),
    "callers_or_public_contributors": (" caller ", " call us ", " text us ", " talkback "),
    "victims_or_survivors": (" victim ", " survivor ", " bereaved "),
    "distressing_subject_matter": (" suicide ", " sexual assault ", " family violence ", " graphic content "),
}


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def child_record_id(parent_record_id: str, media_id: str) -> str:
    return f"{parent_record_id}--rnz-{media_id}"


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


def validate_model_pins(policy: dict[str, Any]) -> list[str]:
    models = policy.get("models", {})
    errors = []
    for name in ("transcription_primary_revision", "transcription_fallback_revision", "diarization_revision"):
        if not re.fullmatch(r"[0-9a-f]{40}", str(models.get(name, ""))):
            errors.append(f"models.{name} must be an immutable 40-character commit revision")
    for name in ("transcription_primary_repo", "transcription_fallback_repo", "diarization"):
        if not re.fullmatch(r"[^/\s]+/[^/\s]+", str(models.get(name, ""))):
            errors.append(f"models.{name} must be a Hugging Face repository ID")
    return errors


def validate_cost_contract(policy: dict[str, Any]) -> list[str]:
    contract = policy.get("cost_contract", {})
    required = {
        "maximum_external_spend_usd": 0,
        "hosted_compute": "github_public_standard_runner_only",
        "hosted_storage": "free_public_tiers_only",
        "paid_fallback": False,
        "local_compute_required": False,
        "on_free_quota_exhaustion": "pause_and_open_issue",
    }
    return [
        f"cost_contract.{name} must be {expected!r}"
        for name, expected in required.items()
        if contract.get(name) != expected
    ]


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


class EmbeddedRNZMediaParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.items: list[dict[str, Any]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag != "rnz-queue-media":
            return
        media = dict(attrs).get("media")
        if not media:
            return
        try:
            item = json.loads(html.unescape(media))
        except json.JSONDecodeError:
            return
        if item.get("id") and item.get("audioSrc") and item.get("canDownload", True):
            self.items.append(item)


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


def embedded_rnz_media(body: str, policy: dict[str, Any]) -> list[dict[str, str]]:
    parser = EmbeddedRNZMediaParser()
    parser.feed(body)
    results = []
    seen = set()
    for item in parser.items:
        media_url = str(item["audioSrc"])
        media_id = str(item["id"])
        if media_id in seen or not host_allowed(media_url, policy["allowed_media_domains"]):
            continue
        seen.add(media_id)
        results.append(
            {"media_id": media_id, "media_url": media_url, "title": str(item.get("title") or media_id)}
        )
    return results


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
    if args.record_id:
        headers = {"Authentication-Token": api_key} if api_key else None
        payload = fetch_json(
            f"https://api.digitalnz.org/v3/records/{args.record_id}.json",
            {},
            headers=headers,
        )
        record = payload.get("record", {})
        record_id = str(record.get("id", "")).strip()
        primary = record.get("primary_collection") or []
        collection = primary[0] if primary else ""
        if record_id != args.record_id or collection not in collections:
            raise ValueError("requested DigitalNZ record is not in an approved RNZ collection")
        candidates = media_candidates(record)
        source_url = next(
            (url for url in candidates if host_allowed(url, policy["allowed_media_domains"])),
            None,
        )
        if not source_url:
            raise ValueError("requested DigitalNZ record has no allowlisted RNZ source URL")
        if record_id not in existing:
            append_event(
                args.manifest,
                {
                    "record_id": record_id,
                    "event": "discovered",
                    "rights_basis": policy["rights_basis"],
                    "source_url": source_url,
                    "collection": collection,
                    "title": record.get("title"),
                    "digitalnz_metadata": record,
                    "metadata_retrieved_at": utc_now(),
                    "retry_count": 0,
                },
            )
        return 0
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
            detail_payload = fetch_json(
                f"https://api.digitalnz.org/v3/records/{record_id}.json",
                {},
                headers=headers,
            )
            detail = detail_payload.get("record", record)
            candidates = media_candidates(detail)
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
                    "title": detail.get("title"),
                    "digitalnz_metadata": detail,
                    "metadata_retrieved_at": utc_now(),
                    "retry_count": 0,
                },
            )
            time.sleep(0.2)
            remaining -= 1
            if remaining <= 0:
                break
    return 0


def resolve_media_assets(source_url: str, policy: dict[str, Any]) -> list[dict[str, str]]:
    if not host_allowed(source_url, policy["allowed_media_domains"]):
        raise ValueError("source host is not allowlisted")
    if urllib.parse.urlparse(source_url).path.lower().endswith(MEDIA_EXTENSIONS):
        return [{"media_id": "direct", "media_url": source_url, "title": "Direct media"}]
    request = urllib.request.Request(source_url, headers={"User-Agent": "dnz-rnz-archive/1"})
    with urllib.request.urlopen(request, timeout=60) as response:
        final_url = response.geturl()
        if not host_allowed(final_url, policy["allowed_media_domains"]):
            raise ValueError("landing-page redirect left the allowlist")
        content_type = response.headers.get_content_type()
        if content_type in policy["allowed_content_types"]:
            return [{"media_id": "direct", "media_url": final_url, "title": "Direct media"}]
        body = response.read(5_000_000).decode("utf-8", errors="ignore")
    matching = extract_matching_audio_url(source_url, body)
    if matching and host_allowed(matching, policy["allowed_media_domains"]):
        media_id = re.search(r"/audio/(\d+)(?:/|$)", urllib.parse.urlparse(source_url).path)
        return [{"media_id": media_id.group(1) if media_id else "direct", "media_url": matching, "title": "Matched RNZ media"}]
    if re.search(r"/audio/\d+(?:/|$)", urllib.parse.urlparse(source_url).path):
        embedded = embedded_rnz_media(body, policy)
        if embedded:
            return embedded
        raise ValueError("landing page does not contain media for the requested audio ID")
    parser = MediaHTMLParser()
    parser.feed(body)
    for candidate in parser.urls:
        candidate = urllib.parse.urljoin(final_url, candidate)
        if candidate.lower().split("?", 1)[0].endswith(MEDIA_EXTENSIONS) and host_allowed(
            candidate, policy["allowed_media_domains"]
        ):
            return [{"media_id": "embedded", "media_url": candidate, "title": "Embedded media"}]
    raise ValueError("no allowlisted media URL found")


def resolve_media_url(source_url: str, policy: dict[str, Any]) -> str:
    assets = resolve_media_assets(source_url, policy)
    if len(assets) != 1:
        raise ValueError(f"landing page contains {len(assets)} child recordings")
    return assets[0]["media_url"]


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
    completed = subprocess.run(
        [
            "ffmpeg", "-nostdin", "-y", "-i", str(source),
            "-map", "0:a:0", "-vn", "-sn", "-dn",
            "-ac", "1", "-ar", "16000", "-c:a", "flac", str(destination),
        ],
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode:
        details = completed.stderr.strip().splitlines()[-3:]
        raise RuntimeError(f"ffmpeg normalization failed: {' | '.join(details)}")


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


def overlap_seconds(rows: list[dict[str, Any]]) -> float:
    intervals = sorted((float(row["start"]), float(row["end"])) for row in rows)
    overlap = 0.0
    previous_end = 0.0
    for start, end in intervals:
        overlap += max(0.0, min(previous_end, end) - start)
        previous_end = max(previous_end, end)
    return overlap


def transcript_analysis(
    segments: list[dict[str, Any]], duration: float, diarization_rows: list[dict[str, Any]]
) -> dict[str, Any]:
    spoken = [segment for segment in segments if str(segment.get("text", "")).strip()]
    speech_seconds = sum(max(0.0, float(s["end"]) - float(s["start"])) for s in spoken)
    words = [word for segment in spoken for word in segment.get("words", [])]
    scores = [float(word["score"]) for word in words if word.get("score") is not None]
    normalized = [re.sub(r"\W+", " ", str(s["text"]).lower()).strip() for s in spoken]
    repeats = sum(1 for previous, current in zip(normalized, normalized[1:]) if current and current == previous)
    full_text = " ".join(normalized)
    topic_tags = [topic for topic, terms in TOPIC_TERMS.items() if any(term in full_text for term in terms)]
    section_hints = [
        {"type": kind, "start": float(segment["start"]), "text": str(segment["text"]).strip()}
        for segment in spoken
        for kind, terms in SECTION_TERMS.items()
        if any(term in str(segment["text"]).lower() for term in terms)
    ]
    padded_text = f" {full_text} "
    sensitive_review_signals = [
        signal for signal, terms in SENSITIVE_REVIEW_TERMS.items() if any(term in padded_text for term in terms)
    ]
    chapters = []
    for index, segment in enumerate(spoken):
        if index == 0 or float(segment["start"]) - float(spoken[index - 1]["end"]) >= 5.0:
            chapters.append(
                {
                    "start": float(segment["start"]),
                    "title": str(segment["text"]).strip()[:120],
                    "basis": "speech_after_pause" if index else "recording_start",
                }
            )
    coverage = min(1.0, speech_seconds / duration) if duration > 0 else 0.0
    quality_flags = []
    if coverage < 0.1:
        quality_flags.append("possible_music_or_non_speech")
    if scores and sum(scores) / len(scores) < 0.65:
        quality_flags.append("low_word_confidence")
    if repeats >= 3:
        quality_flags.append("repetition_detected")
    if not chapters:
        quality_flags.append("no_speech_segments")
    maori_markers = re.findall(r"\b(?:te|ngā|nga|iwi|hapū|hapu|marae|whānau|whanau|māori|maori)\b", full_text)
    if maori_markers:
        quality_flags.append("maori_language_or_names_review_recommended")
    return {
        "speech_coverage": round(coverage, 6),
        "average_word_confidence": round(sum(scores) / len(scores), 6) if scores else None,
        "adjacent_repeated_segments": repeats,
        "speaker_count": len({row.get("speaker") for row in diarization_rows if row.get("speaker")}),
        "overlap_seconds": round(overlap_seconds(diarization_rows), 3),
        "topics": topic_tags,
        "section_hints": section_hints,
        "chapters": chapters,
        "quality_flags": list(dict.fromkeys(quality_flags)),
        "maori_marker_count": len(maori_markers),
        "sensitive_review_signals": sensitive_review_signals,
        "limitations": [
            "Topic and section tags are deterministic search hints, not editorial classifications.",
            "Māori markers trigger review and do not constitute language identification.",
            "Speaker labels are anonymous and must not be used for speaker identification.",
            "Sensitive-content signals only request review and must not trigger automatic restriction or removal.",
        ],
    }


def verify_item_outputs(output_dir: Path, expected_duration: float) -> dict[str, Any]:
    required = (
        "audio.flac", "transcript.json", "transcript.srt", "transcript.vtt",
        "transcript.txt", "diarization.rttm", "analysis.json", "chapters.json", "review.json",
    )
    missing = [name for name in required if not (output_dir / name).is_file() or (output_dir / name).stat().st_size == 0]
    if missing:
        raise RuntimeError(f"item output verification failed; missing or empty: {', '.join(missing)}")
    normalized_duration = ffprobe_duration(output_dir / "audio.flac")
    if abs(normalized_duration - expected_duration) > 1.0:
        raise RuntimeError("normalized audio duration differs from the source by more than one second")
    transcript = load_json(output_dir / "transcript.json")
    if not isinstance(transcript.get("segments"), list):
        raise RuntimeError("canonical transcript has no segment list")
    invalid_speakers = {
        str(segment.get("speaker"))
        for segment in transcript["segments"]
        if segment.get("speaker") and not re.fullmatch(r"SPEAKER_\d+|SPEAKER_UNKNOWN", str(segment["speaker"]))
    }
    if invalid_speakers:
        raise RuntimeError("canonical transcript contains non-anonymous speaker labels")
    return {
        "verified": True,
        "normalized_duration_seconds": normalized_duration,
        "normalized_sha256": sha256_file(output_dir / "audio.flac"),
        "verified_outputs": list(required),
    }


def transcribe(audio: Path, output_dir: Path, policy: dict[str, Any], hf_token: str, duration: float) -> dict[str, Any]:
    import whisperx
    from faster_whisper.utils import download_model
    from huggingface_hub import snapshot_download
    from whisperx.diarize import DiarizationPipeline

    models = policy["models"]
    device = "cpu"
    transcription_path = download_model(
        models["transcription_primary_repo"],
        revision=models["transcription_primary_revision"],
    )
    model = whisperx.load_model(transcription_path, device, compute_type="int8", language=None)
    raw = model.transcribe(str(audio), batch_size=1)
    language = raw.get("language", "unknown")
    quality_flags: list[str] = []
    rows: list[dict[str, Any]] = []
    try:
        align_model, metadata = whisperx.load_align_model(language_code=language, device=device)
        aligned = whisperx.align(raw["segments"], align_model, metadata, str(audio), device, return_char_alignments=False)
    except Exception:
        aligned = raw
        quality_flags.append("alignment_unavailable")
    if not hf_token:
        raise RuntimeError("HF_TOKEN with gated pyannote model access is required")
    try:
        diarization_path = snapshot_download(
            models["diarization"],
            revision=models["diarization_revision"],
            token=hf_token,
        )
        diarizer = DiarizationPipeline(
            model_name=diarization_path, token=hf_token, device=device
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
    analysis = transcript_analysis(result["segments"], duration, rows)
    result["quality_flags"].extend(analysis["quality_flags"])
    result["quality_flags"] = list(dict.fromkeys(result["quality_flags"]))
    review_reasons = list(
        dict.fromkeys(
            [f"quality:{flag}" for flag in result["quality_flags"]]
            + [f"sensitive:{signal}" for signal in analysis["sensitive_review_signals"]]
        )
    )
    review = {
        "required": bool(review_reasons),
        "status": "pending" if review_reasons else "not_required",
        "reasons": review_reasons,
        "created_at": utc_now(),
        "disposition": None,
        "reviewer": None,
        "reviewed_at": None,
    }
    (output_dir / "transcript.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    (output_dir / "analysis.json").write_text(json.dumps(analysis, ensure_ascii=False, indent=2), encoding="utf-8")
    (output_dir / "chapters.json").write_text(json.dumps(analysis["chapters"], ensure_ascii=False, indent=2), encoding="utf-8")
    (output_dir / "review.json").write_text(json.dumps(review, ensure_ascii=False, indent=2), encoding="utf-8")
    result["outputs"] = write_caption_files(result["segments"], output_dir)
    result["outputs"].update({"analysis": "analysis.json", "chapters": "chapters.json", "review": "review.json", "rttm": "diarization.rttm"})
    result["review"] = review
    return result


def process(args: argparse.Namespace) -> int:
    policy = load_json(args.policy)
    events = read_events(args.manifest)
    latest = latest_by_record(events)
    pending = [event for event in latest.values() if event["event"] in {"discovered", "retry"}]
    if args.record_id:
        pending = [event for event in pending if event["record_id"] == args.record_id]
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
            assets = resolve_media_assets(item["source_url"], policy)
            if len(assets) > 1:
                parent_record_id = record_id
                append_event(
                    args.manifest,
                    {
                        **item,
                        "event": "expanded",
                        "child_count": len(assets),
                        "child_record_ids": [child_record_id(parent_record_id, asset["media_id"]) for asset in assets],
                    },
                )
                children = []
                for asset in assets:
                    child = append_event(
                        args.manifest,
                        {
                            **item,
                            "record_id": child_record_id(parent_record_id, asset["media_id"]),
                            "parent_record_id": parent_record_id,
                            "rnz_media_id": asset["media_id"],
                            "source_url": asset["media_url"],
                            "title": asset["title"],
                            "event": "discovered",
                            "retry_count": 0,
                        },
                    )
                    children.append((child, asset))
                item, asset = children[0]
                record_id = str(item["record_id"])
                output_dir = args.output / datetime.now(timezone.utc).strftime("%Y-%m") / "items" / record_id
                output_dir.mkdir(parents=True, exist_ok=True)
                source_path = output_dir / "source.media"
                normalized = output_dir / "audio.flac"
                media_url = asset["media_url"]
            else:
                media_url = assets[0]["media_url"]
            sha256, size, content_type = download_media(media_url, source_path, policy)
            duration = ffprobe_duration(source_path)
            if duration > float(policy["max_duration_seconds"]):
                raise ValueError("media exceeds maximum duration")
            normalize_audio(source_path, normalized)
            append_event(args.manifest, {**item, "event": "downloaded", "media_url": media_url, "sha256": sha256, "size_bytes": size, "content_type": content_type, "duration_seconds": duration})
            result = transcribe(normalized, output_dir, policy, hf_token, duration)
            integrity = verify_item_outputs(output_dir, duration)
            duplicate_of = next(
                (
                    event["record_id"]
                    for event in read_events(args.manifest)
                    if event.get("event") == "processed"
                    and event.get("normalized_sha256") == integrity["normalized_sha256"]
                    and event.get("record_id") != record_id
                ),
                None,
            )
            provenance = {"record_id": record_id, "parent_record_id": item.get("parent_record_id"), "source_url": item["source_url"], "media_url": media_url, "sha256": sha256, "rights_basis": policy["rights_basis"], "models": policy["models"], "integrity": integrity, "duplicate_of": duplicate_of, "generated_at": utc_now()}
            (output_dir / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
            append_event(args.manifest, {**item, "event": "processed", "media_url": media_url, "sha256": sha256, "normalized_sha256": integrity["normalized_sha256"], "duplicate_of": duplicate_of, "size_bytes": size, "duration_seconds": duration, "models": policy["models"], "outputs": result["outputs"], "quality_flags": result["quality_flags"], "review_required": result["review"]["required"], "review_reasons": result["review"]["reasons"]})
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
    payload["review_required"] = sum(1 for event in latest.values() if event.get("review_required"))
    print(json.dumps(payload, indent=2, sort_keys=True))
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a", encoding="utf-8") as handle:
            handle.write("## RNZ archive summary\n\n")
            handle.write(f"- Records: {len(latest)}\n")
            for state, count in sorted(counts.items()):
                handle.write(f"- {state}: {count}\n")
            handle.write(f"- Review required: {payload['review_required']}\n")
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
    discover_parser.add_argument("--record-id")
    discover_parser.add_argument("--api-key")
    discover_parser.set_defaults(func=discover)

    process_parser = subparsers.add_parser("process")
    process_parser.add_argument("--manifest", type=Path, required=True)
    process_parser.add_argument("--output", type=Path, required=True)
    process_parser.add_argument("--max-items", type=int, default=5)
    process_parser.add_argument("--max-retries", type=int, default=3)
    process_parser.add_argument("--deadline-minutes", type=int, default=330)
    process_parser.add_argument("--record-id")
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
    errors.extend(validate_cost_contract(policy))
    errors.extend(validate_model_pins(policy))
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(json.dumps({"zero_cost_policy": "pass", "hf_repo_id": policy["hf_repo_id"]}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
