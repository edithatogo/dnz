import importlib.util
import json
import tempfile
import unittest
from email.message import Message
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("rnz_archive", ROOT / "scripts" / "rnz_archive.py")
rnz_archive = importlib.util.module_from_spec(SPEC)
assert SPEC.loader
SPEC.loader.exec_module(rnz_archive)


class RNZArchiveTests(unittest.TestCase):
    def test_host_allowlist_rejects_lookalikes_and_http(self):
        allowed = ["rnz.co.nz"]
        self.assertTrue(rnz_archive.host_allowed("https://media.rnz.co.nz/a.mp3", allowed))
        self.assertFalse(rnz_archive.host_allowed("https://rnz.co.nz.example.org/a.mp3", allowed))
        with self.assertRaises(ValueError):
            rnz_archive.host_allowed("http://rnz.co.nz/a.mp3", allowed)

    def test_manifest_is_append_only_and_latest_state_wins(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / "manifest.jsonl"
            base = {"record_id": "123", "rights_basis": "authorized"}
            first = rnz_archive.append_event(manifest, {**base, "event": "discovered"})
            second = rnz_archive.append_event(manifest, {**base, "event": "retry", "retry_count": 1})
            events = rnz_archive.read_events(manifest)
            self.assertEqual(2, len(events))
            self.assertNotEqual(first["event_id"], second["event_id"])
            self.assertEqual("retry", rnz_archive.latest_by_record(events)["123"]["event"])

    def test_append_event_replaces_inherited_identity_and_timestamp(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / "manifest.jsonl"
            event = rnz_archive.append_event(
                manifest,
                {
                    "record_id": "123",
                    "event": "retry",
                    "rights_basis": "authorized",
                    "event_id": "inherited",
                    "timestamp": "2000-01-01T00:00:00Z",
                },
            )
            self.assertNotEqual("inherited", event["event_id"])
            self.assertNotEqual("2000-01-01T00:00:00Z", event["timestamp"])

    def test_media_candidates_are_ordered_and_deduplicated(self):
        record = {
            "object_url": ["https://rnz.co.nz/a.mp3", "https://rnz.co.nz/a.mp3"],
            "source_url": "https://rnz.co.nz/story",
        }
        self.assertEqual(
            ["https://rnz.co.nz/a.mp3", "https://rnz.co.nz/story"],
            rnz_archive.media_candidates(record),
        )

    def test_exact_record_discovery_uses_requested_digitalnz_id(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / "manifest.jsonl"
            args = type(
                "Args",
                (),
                {
                    "policy": ROOT / "rnz" / "archive-policy.json",
                    "manifest": manifest,
                    "limit": 1,
                    "query": None,
                    "api_key": None,
                    "record_id": "41680626",
                },
            )()
            response = {
                "record": {
                    "id": 41680626,
                    "title": "Tagata o te moana",
                    "primary_collection": ["Radio New Zealand"],
                    "landing_url": "https://www.rnz.co.nz/programmes/audio/2018696673/item",
                }
            }
            with mock.patch.object(rnz_archive, "fetch_json", return_value=response):
                rnz_archive.discover(args)
            event = rnz_archive.read_events(manifest)[0]
            self.assertEqual("41680626", event["record_id"])
            self.assertEqual(response["record"], event["digitalnz_metadata"])

    def test_audio_extraction_requires_matching_rnz_id(self):
        body = """
        latest-bulletin='{"id":999,"audioSrc":"https://podcast.radionz.co.nz/news/latest.mp3"}'
        media='{"id":2018696673,"audioSrc":"https://podcast.radionz.co.nz/tagata/item.mp3"}'
        """
        source = "https://www.rnz.co.nz/programme/audio/2018696673/example"
        self.assertEqual(
            "https://podcast.radionz.co.nz/tagata/item.mp3",
            rnz_archive.extract_matching_audio_url(source, body),
        )
        self.assertIsNone(
            rnz_archive.extract_matching_audio_url(
                "https://www.rnz.co.nz/programme/audio/1234/example", body
            )
        )

    def test_compound_page_extracts_only_downloadable_rnz_children(self):
        body = """
        <rnz-queue-media media='{"id":201,"title":"First","audioSrc":"https://podcast.radionz.co.nz/a.mp3","canDownload":true}'></rnz-queue-media>
        <rnz-queue-media media='{"id":202,"title":"Blocked","audioSrc":"https://podcast.radionz.co.nz/b.mp3","canDownload":false}'></rnz-queue-media>
        <rnz-queue-media media='{"id":203,"title":"External","audioSrc":"https://example.org/c.mp3","canDownload":true}'></rnz-queue-media>
        """
        policy = {"allowed_media_domains": ["radionz.co.nz"]}
        self.assertEqual(
            [{"media_id": "201", "media_url": "https://podcast.radionz.co.nz/a.mp3", "title": "First"}],
            rnz_archive.embedded_rnz_media(body, policy),
        )

    def test_child_record_ids_are_stable_and_windows_safe(self):
        child = rnz_archive.child_record_id("41680624", "2018696729")
        self.assertEqual("41680624--rnz-2018696729", child)
        self.assertNotRegex(child, r'[<>:"/\\|?*]')

    def test_audio_id_page_rejects_unrelated_fallback_media(self):
        class Response:
            headers = Message()

            def __enter__(self):
                self.headers["Content-Type"] = "text/html"
                return self

            def __exit__(self, *args):
                return False

            def geturl(self):
                return "https://www.rnz.co.nz/programme/audio/1234/example"

            def read(self, _limit):
                return b'<audio src="https://podcast.radionz.co.nz/news/latest.mp3">'

        policy = {
            "allowed_media_domains": ["rnz.co.nz", "radionz.co.nz"],
            "allowed_content_types": ["audio/mpeg"],
        }
        with mock.patch.object(rnz_archive.urllib.request, "urlopen", return_value=Response()):
            with self.assertRaisesRegex(ValueError, "requested audio ID"):
                rnz_archive.resolve_media_url(
                    "https://www.rnz.co.nz/programme/audio/1234/example", policy
                )

    def test_caption_outputs_use_anonymous_speakers(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            files = rnz_archive.write_caption_files(
                [{"start": 1.0, "end": 2.5, "speaker": "SPEAKER_00", "text": "Kia ora"}],
                output,
            )
            self.assertIn("SPEAKER_00", (output / files["srt"]).read_text(encoding="utf-8"))
            self.assertTrue((output / files["vtt"]).read_text(encoding="utf-8").startswith("WEBVTT"))

    def test_normalization_selects_audio_and_ignores_attached_art(self):
        completed = type("Completed", (), {"returncode": 0, "stderr": ""})()
        with mock.patch.object(rnz_archive.subprocess, "run", return_value=completed) as run:
            rnz_archive.normalize_audio(Path("source.mp3"), Path("audio.flac"))
        command = run.call_args.args[0]
        self.assertIn("0:a:0", command)
        self.assertIn("-vn", command)

    def test_rttm_uses_dataframe_style_rows(self):
        lines = rnz_archive.rttm_lines(
            "recording",
            [{"start": 1.25, "end": 3.75, "speaker": "SPEAKER_00"}],
        )
        self.assertEqual(
            "SPEAKER recording 1 1.250 2.500 <NA> <NA> SPEAKER_00 <NA> <NA>",
            lines[0],
        )

    def test_transcript_analysis_flags_broadcast_topics_maori_and_overlap(self):
        segments = [
            {
                "start": 0.0,
                "end": 4.0,
                "text": "Headlines from Parliament and Te Tiriti negotiations",
                "words": [{"word": "Headlines", "score": 0.9}],
            },
            {
                "start": 10.0,
                "end": 14.0,
                "text": "Joining us to discuss iwi and housing",
                "words": [{"word": "housing", "score": 0.8}],
            },
        ]
        rows = [
            {"start": 0.0, "end": 5.0, "speaker": "SPEAKER_00"},
            {"start": 4.0, "end": 8.0, "speaker": "SPEAKER_01"},
        ]
        analysis = rnz_archive.transcript_analysis(segments, 20.0, rows)
        self.assertIn("parliament_and_politics", analysis["topics"])
        self.assertIn("te_tiriti_and_maori_affairs", analysis["topics"])
        self.assertIn("maori_language_or_names_review_recommended", analysis["quality_flags"])
        self.assertEqual(2, analysis["speaker_count"])
        self.assertEqual(1.0, analysis["overlap_seconds"])
        self.assertEqual(2, len(analysis["chapters"]))

    def test_transcript_analysis_flags_low_speech_coverage(self):
        analysis = rnz_archive.transcript_analysis([], 60.0, [])
        self.assertIn("possible_music_or_non_speech", analysis["quality_flags"])
        self.assertIn("no_speech_segments", analysis["quality_flags"])

    def test_transcript_analysis_emits_review_only_sensitive_signals(self):
        analysis = rnz_archive.transcript_analysis(
            [{"start": 0, "end": 2, "text": "A caller discussed family violence", "words": []}],
            2.0,
            [],
        )
        self.assertIn("callers_or_public_contributors", analysis["sensitive_review_signals"])
        self.assertIn("distressing_subject_matter", analysis["sensitive_review_signals"])
        self.assertTrue(any("must not trigger" in item for item in analysis["limitations"]))

    def test_analysis_schema_requires_every_generated_field(self):
        schema = json.loads((ROOT / "rnz" / "analysis.schema.json").read_text(encoding="utf-8"))
        analysis = rnz_archive.transcript_analysis([], 60.0, [])
        self.assertEqual(set(schema["required"]), set(analysis))

    def test_item_integrity_rejects_non_anonymous_speaker_labels(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            required = (
                "audio.flac", "transcript.srt", "transcript.vtt", "transcript.txt",
                "diarization.rttm", "analysis.json", "chapters.json",
            )
            for name in required:
                (output / name).write_text("content", encoding="utf-8")
            (output / "transcript.json").write_text(
                json.dumps({"segments": [{"speaker": "KNOWN_PERSON"}]}), encoding="utf-8"
            )
            with mock.patch.object(rnz_archive, "ffprobe_duration", return_value=10.0):
                with self.assertRaisesRegex(RuntimeError, "non-anonymous"):
                    rnz_archive.verify_item_outputs(output, 10.0)

    def test_zero_cost_policy_rejects_paid_services(self):
        with tempfile.TemporaryDirectory() as directory:
            workflows = Path(directory)
            (workflows / "rnz.yml").write_text(
                "jobs:\n  paid:\n    runs-on: gpu-runner\n    steps:\n      - run: hf jobs run image command\n",
                encoding="utf-8",
            )
            errors = rnz_archive.validate_zero_cost(workflows, "edithatogo/digitalnz")
            self.assertTrue(any("runner" in error.lower() for error in errors))
            self.assertTrue(any("jobs" in error.lower() for error in errors))

    def test_repo_workflows_pass_zero_cost_policy(self):
        policy = json.loads((ROOT / "rnz" / "archive-policy.json").read_text(encoding="utf-8"))
        self.assertEqual([], rnz_archive.validate_zero_cost(ROOT / ".github" / "workflows", policy["hf_repo_id"]))
        self.assertEqual([], rnz_archive.validate_model_pins(policy))

    def test_model_policy_rejects_moving_or_missing_revisions(self):
        policy = {
            "models": {
                "transcription_primary_repo": "owner/model",
                "transcription_fallback_repo": "owner/fallback",
                "diarization": "owner/diarization",
                "transcription_primary_revision": "main",
            }
        }
        errors = rnz_archive.validate_model_pins(policy)
        self.assertTrue(any("transcription_primary_revision" in error for error in errors))
        self.assertTrue(any("diarization_revision" in error for error in errors))

    def test_shard_id_is_required(self):
        with self.assertRaises(SystemExit):
            rnz_archive.main([
                "package",
                "--manifest", "state.jsonl",
                "--items", "items",
                "--output", "release",
            ])

    def test_package_manifest_only_contains_packaged_records(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "state.jsonl"
            items = root / "items"
            release = root / "release"
            for record_id in ("included", "excluded"):
                rnz_archive.append_event(
                    manifest,
                    {
                        "record_id": record_id,
                        "event": "processed",
                        "title": record_id,
                        "rights_basis": "authorized",
                    },
                )
            included = items / "2026-07" / "items" / "included"
            included.mkdir(parents=True)
            (included / "provenance.json").write_text("{}", encoding="utf-8")
            args = type(
                "Args",
                (),
                {
                    "manifest": manifest,
                    "items": items,
                    "output": release,
                    "shard_id": "test",
                },
            )()
            try:
                rnz_archive.package(args)
            except ModuleNotFoundError as exc:
                self.skipTest(f"optional packaging dependency unavailable: {exc}")
            release_manifest = (release / "manifest-test.jsonl").read_text(encoding="utf-8")
            self.assertIn("included", release_manifest)
            self.assertNotIn("excluded", release_manifest)

    def test_package_rejects_empty_shard(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "state.jsonl"
            manifest.write_text("", encoding="utf-8")
            args = type(
                "Args",
                (),
                {
                    "manifest": manifest,
                    "items": root / "items",
                    "output": root / "release",
                    "shard_id": "empty",
                },
            )()
            with self.assertRaisesRegex(RuntimeError, "empty archive shard"):
                rnz_archive.package(args)


if __name__ == "__main__":
    unittest.main()
