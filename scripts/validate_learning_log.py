#!/usr/bin/env python3
"""Validate conductor learning-log entries against the local JSON schema."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from dataclasses import dataclass, field
from pathlib import Path


ENTRY_HEADING = re.compile(r"^##\s+(?P<heading>.+)$")
SCALAR_FIELD = re.compile(r"^-\s+`(?P<key>[a-z_]+)`:\s*(?P<value>.*)$")
ARRAY_ITEM = re.compile(r"^-\s+(?P<value>.+)$")


@dataclass
class Entry:
    heading: str
    fields: dict[str, object] = field(default_factory=dict)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--log",
        default="conductor/learning-log.md",
        help="Path to the learning log markdown file.",
    )
    parser.add_argument(
        "--schema",
        default="conductor/templates/learning-entry.schema.json",
        help="Path to the learning-entry JSON schema.",
    )
    return parser.parse_args()


def load_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def parse_entries(text: str) -> list[Entry]:
    entries: list[Entry] = []
    current: Entry | None = None
    current_array_key: str | None = None

    for raw_line in text.splitlines():
        line = raw_line.rstrip()
        heading_match = ENTRY_HEADING.match(line)
        if heading_match:
            current = Entry(heading=heading_match.group("heading"))
            entries.append(current)
            current_array_key = None
            continue

        if current is None:
            continue

        scalar_match = SCALAR_FIELD.match(line)
        if scalar_match:
            key = scalar_match.group("key")
            value = scalar_match.group("value").strip()
            if value.startswith("`") and value.endswith("`") and len(value) >= 2:
                value = value[1:-1]
            if key in {"lessons_learned", "next_check_to_add", "evidence"}:
                current.fields[key] = []
                current_array_key = key
            else:
                current.fields[key] = value
                current_array_key = key
            continue

        if raw_line.lstrip().startswith("- ") and current_array_key in {
            "lessons_learned",
            "next_check_to_add",
            "evidence",
        }:
            item = ARRAY_ITEM.match(raw_line.lstrip())
            if item:
                items = current.fields.setdefault(current_array_key, [])
                assert isinstance(items, list)
                items.append(item.group("value").strip())

    return entries


def validate_entry(entry: Entry, schema: dict[str, object]) -> list[str]:
    errors: list[str] = []
    required = set(schema.get("required", []))
    properties = schema.get("properties", {})

    for key in sorted(required):
        if key not in entry.fields:
            errors.append(f"{entry.heading}: missing required field `{key}`")
            continue
        value = entry.fields[key]
        if isinstance(value, str) and not value.strip():
            errors.append(f"{entry.heading}: field `{key}` is empty")
        if isinstance(value, list) and not value:
            errors.append(f"{entry.heading}: field `{key}` is empty")

    observed_on = entry.fields.get("observed_on")
    if isinstance(observed_on, str):
        try:
            dt.date.fromisoformat(observed_on)
        except ValueError:
            errors.append(
                f"{entry.heading}: field `observed_on` is not a valid ISO date"
            )

    for key in ("scope", "severity", "status"):
        allowed = properties.get(key, {}).get("enum", [])
        if allowed and key in entry.fields and entry.fields[key] not in allowed:
            errors.append(
                f"{entry.heading}: field `{key}` has unexpected value `{entry.fields[key]}`"
            )

    for key in ("lessons_learned", "next_check_to_add"):
        value = entry.fields.get(key)
        if isinstance(value, list) and not value:
            errors.append(f"{entry.heading}: field `{key}` must contain at least one item")

    return errors


def main() -> int:
    args = parse_args()
    log_path = Path(args.log)
    schema_path = Path(args.schema)

    schema = json.loads(load_text(schema_path))
    entries = parse_entries(load_text(log_path))
    errors: list[str] = []

    if not entries:
        errors.append(f"{log_path}: no learning entries found")

    for entry in entries:
        errors.extend(validate_entry(entry, schema))

    if errors:
        for error in errors:
            print(error)
        return 1

    print(f"Validated {len(entries)} learning entr{'y' if len(entries) == 1 else 'ies'} in {log_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
