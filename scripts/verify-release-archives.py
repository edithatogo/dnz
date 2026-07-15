"""Verify a staged release directory and emit deterministic SHA-256 sums."""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path


FORBIDDEN_PARTS = {".git", "digitalnz", "pydnz", "target", "__pycache__"}
REQUIRED_FILES = {"LICENSE", "README.md", "CHANGELOG.md"}


def verify(root: Path) -> list[str]:
    errors: list[str] = []
    names = {path.name for path in root.iterdir() if path.is_file()}
    errors.extend(f"missing required release file: {name}" for name in sorted(REQUIRED_FILES - names))
    files = sorted(path for path in root.rglob("*") if path.is_file())
    for path in files:
        if FORBIDDEN_PARTS.intersection(path.relative_to(root).parts):
            errors.append(f"forbidden release path: {path.relative_to(root)}")
    sums = []
    for path in files:
        if path.name == "SHA256SUMS":
            continue
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        sums.append(f"{digest}  {path.relative_to(root).as_posix()}")
    (root / "SHA256SUMS").write_text("\n".join(sums) + "\n", encoding="utf-8")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    errors = verify(args.root.resolve())
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    print(f"Release archive staging verified: {args.root.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
