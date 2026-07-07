from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "check_code_scanning_alerts.py"
SPEC = importlib.util.spec_from_file_location("check_code_scanning_alerts", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"Unable to load module from {MODULE_PATH}")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class CheckCodeScanningAlertsTests(unittest.TestCase):
    def test_alert_severity_prefers_security_severity_level(self) -> None:
        alert = {"rule": {"security_severity_level": "high", "severity": "medium"}}
        self.assertEqual(MODULE.alert_severity(alert), "high")

    def test_blocking_alerts_matches_high_and_critical_on_commit(self) -> None:
        alerts = [
            {
                "number": 1,
                "rule": {"id": "sql-injection", "severity": "medium"},
                "most_recent_instance": {
                    "commit_sha": "abc123",
                    "message": {"text": "medium severity alert"},
                },
                "html_url": "https://example.com/1",
            },
            {
                "number": 2,
                "rule": {"id": "xss", "security_severity_level": "high"},
                "most_recent_instance": {
                    "commit_sha": "abc123",
                    "message": {"text": "high severity alert"},
                },
                "html_url": "https://example.com/2",
            },
            {
                "number": 3,
                "rule": {"id": "ssrf", "security_severity_level": "critical"},
                "most_recent_instance": {
                    "commit_sha": "abc123",
                    "message": {"text": "critical severity alert"},
                },
                "html_url": "https://example.com/3",
            },
            {
                "number": 4,
                "rule": {"id": "xss", "security_severity_level": "high"},
                "most_recent_instance": {
                    "commit_sha": "def456",
                    "message": {"text": "wrong commit"},
                },
                "html_url": "https://example.com/4",
            },
        ]

        hits = MODULE.blocking_alerts(alerts, commit_sha="abc123")

        self.assertEqual([hit.number for hit in hits], [2, 3])
        self.assertEqual([hit.severity for hit in hits], ["high", "critical"])


if __name__ == "__main__":
    unittest.main()
