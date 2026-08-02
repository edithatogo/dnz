"""Offline, dependency-light checks for GitHub Actions safety policy."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = ROOT / ".github" / "workflows"
SHA_REF = re.compile(r"^[0-9a-f]{40}$")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--strict", action="store_true", help="fail when an action ref is not an immutable SHA")
    args = parser.parse_args()
    workflow_count = 0
    action_refs = 0
    mutable_refs: list[str] = []
    failures: list[str] = []
    for path in sorted(WORKFLOWS.glob("*.y*ml")):
        workflow_count += 1
        document = yaml.safe_load(path.read_text(encoding="utf-8"))
        if not isinstance(document, dict):
            failures.append(f"{path.name}: workflow must be a YAML mapping")
            continue
        permissions = document.get("permissions")
        if permissions == "write-all":
            failures.append(f"{path.name}: write-all permissions are forbidden")
        if "pull_request_target" in document.get(True, {}) or "pull_request_target" in document:
            failures.append(f"{path.name}: pull_request_target requires manual review")
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            match = re.search(r"\buses:\s*([^\s#]+)", line)
            if not match:
                continue
            action_refs += 1
            reference = match.group(1).split("@", 1)[-1]
            if not SHA_REF.fullmatch(reference):
                mutable_refs.append(f"{path.name}:{line_number}:{reference}")
        if path.name == "hardening.yml" and permissions != {"contents": "read"}:
            failures.append("hardening.yml: top-level permissions must remain contents: read")
    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1
    if args.strict and mutable_refs:
        for reference in mutable_refs:
            print(f"FAIL: mutable action reference: {reference}")
        return 1
    print(f"workflow policy validation: PASS ({workflow_count} workflows, {action_refs} action references)")
    if mutable_refs:
        print(f"mutable action references pending pinning: {len(mutable_refs)}")
        for reference in mutable_refs:
            print(f"  {reference}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
