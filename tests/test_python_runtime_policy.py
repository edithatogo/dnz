from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / "scripts"
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from check_python_runtime_policy import validate_runtime_policy  # noqa: E402


class PythonRuntimePolicyTests(unittest.TestCase):
    def test_repository_supports_only_python_314(self) -> None:
        self.assertEqual([], validate_runtime_policy(ROOT))


if __name__ == "__main__":
    unittest.main()
