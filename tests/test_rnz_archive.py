import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


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

    def test_media_candidates_are_ordered_and_deduplicated(self):
        record = {
            "object_url": ["https://rnz.co.nz/a.mp3", "https://rnz.co.nz/a.mp3"],
            "source_url": "https://rnz.co.nz/story",
        }
        self.assertEqual(
            ["https://rnz.co.nz/a.mp3", "https://rnz.co.nz/story"],
            rnz_archive.media_candidates(record),
        )

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

    def test_caption_outputs_use_anonymous_speakers(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            files = rnz_archive.write_caption_files(
                [{"start": 1.0, "end": 2.5, "speaker": "SPEAKER_00", "text": "Kia ora"}],
                output,
            )
            self.assertIn("SPEAKER_00", (output / files["srt"]).read_text(encoding="utf-8"))
            self.assertTrue((output / files["vtt"]).read_text(encoding="utf-8").startswith("WEBVTT"))

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

    def test_shard_id_is_required(self):
        with self.assertRaises(SystemExit):
            rnz_archive.main([
                "package",
                "--manifest", "state.jsonl",
                "--items", "items",
                "--output", "release",
            ])


if __name__ == "__main__":
    unittest.main()
