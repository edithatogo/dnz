"""Report drift between the checked-in DigitalNZ contract and Rust route builders.

The checked-in contract intentionally records both official prose and OpenAPI
paths. This offline check does not silently replace that evidence with a live
network fetch; it reports missing route fragments and the documented drift.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


EXPECTED_FRAGMENTS = {
    "search": ("records.json", "response_format"),
    "record": ("records", "record_id"),
    "more_like_this": ("more_like_this",),
}


def contract_endpoints(text: str) -> dict[str, dict[str, str]]:
    endpoints: dict[str, dict[str, str]] = {}
    current: str | None = None
    for line in text.splitlines():
        match = re.match(r"^  ([a-z_]+):\s*$", line)
        if match and match.group(1) in EXPECTED_FRAGMENTS:
            current = match.group(1)
            endpoints[current] = {}
            continue
        if current:
            field = re.match(r"^    ([a-z_]+):\s*(\S+)\s*$", line)
            if field:
                endpoints[current][field.group(1)] = field.group(2)
            elif line and not line.startswith(" "):
                current = None
    return endpoints


def build_report(root: Path, contract_path: Path, source_path: Path) -> dict:
    contract_text = contract_path.read_text(encoding="utf-8")
    source_text = source_path.read_text(encoding="utf-8")
    endpoints = contract_endpoints(contract_text)
    checks = []
    for name, fragments in EXPECTED_FRAGMENTS.items():
        present = [fragment for fragment in fragments if fragment in source_text]
        missing = [fragment for fragment in fragments if fragment not in source_text]
        checks.append(
            {
                "endpoint": name,
                "present_fragments": present,
                "missing_fragments": missing,
                "status": "pass" if not missing else "review",
            }
        )
    return {
        "schema_version": 1,
        "contract": str(contract_path.relative_to(root)).replace("\\", "/"),
        "source": str(source_path.relative_to(root)).replace("\\", "/"),
        "contract_status": re.search(r"^status:\s*(\S+)", contract_text, re.MULTILINE).group(1)
        if re.search(r"^status:\s*(\S+)", contract_text, re.MULTILINE)
        else "unknown",
        "documented_endpoints": endpoints,
        "checks": checks,
        "limitations": [
            "This is an offline route-surface report; it does not prove current provider behavior.",
            "Run the opt-in live smoke workflow separately when external service access is available.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract", type=Path, default=Path("conductor/contracts/digitalnz-v3.yaml"))
    parser.add_argument("--source", type=Path, default=Path("crates/dnz-core/src/client.rs"))
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = build_report(root, args.contract.resolve(), args.source.resolve())
    print(json.dumps(report, indent=2, sort_keys=True))
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0 if all(check["status"] == "pass" for check in report["checks"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
