"""Validate DNZ's single-minor Python runtime policy."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

SUPPORTED_MINOR = "3.14"
ROOT = Path(__file__).resolve().parents[1]


def validate_runtime_policy(root: Path = ROOT) -> list[str]:
    """Return policy errors for Python declarations outside 3.14."""
    errors: list[str] = []

    pixi = tomllib.loads((root / "pixi.toml").read_text(encoding="utf-8"))
    if pixi["dependencies"].get("python") != ">=3.14,<3.15":
        errors.append("pixi.toml must constrain Python to >=3.14,<3.15")

    package = tomllib.loads(
        (root / "crates" / "dnz-python" / "pyproject.toml").read_text(encoding="utf-8"),
    )
    if package["project"].get("requires-python") != ">=3.14,<3.15":
        errors.append("dnz-python requires-python must be >=3.14,<3.15")

    workflow_pattern = re.compile(r"python-version:\s*['\"]?(3\.\d+)")
    for workflow in sorted((root / ".github" / "workflows").glob("*.y*ml")):
        for declared in workflow_pattern.findall(workflow.read_text(encoding="utf-8")):
            if declared != SUPPORTED_MINOR:
                errors.append(f"{workflow.relative_to(root)} declares Python {declared}")

    wheel_script = (root / "scripts" / "verify-python-wheel.ps1").read_text(
        encoding="utf-8",
    )
    if "Python312" in wheel_script:
        errors.append("verify-python-wheel.ps1 retains a Python312 fallback")

    transcription_lock = (
        root / "rnz" / "transcription-requirements.lock"
    ).read_text(encoding="utf-8")
    if "--python-version 3.14" not in transcription_lock.splitlines()[1]:
        errors.append("transcription lock provenance must target Python 3.14")

    transcription_input = (
        root / "rnz" / "transcription-requirements.in"
    ).read_text(encoding="utf-8")
    archive_script = (root / "scripts" / "rnz_archive.py").read_text(encoding="utf-8")
    if re.search(r"^whisperx(?:\W|$)", transcription_input, re.MULTILINE | re.IGNORECASE):
        errors.append("transcription dependencies retain Python-incompatible WhisperX")
    if re.search(r"(?:import|from)\s+whisperx(?:\W|$)", archive_script):
        errors.append("RNZ archive runtime imports Python-incompatible WhisperX")

    return errors


def main() -> int:
    """Report policy errors and return a failing status when any exist."""
    errors = validate_runtime_policy()
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print("Python runtime policy: 3.14-only")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
