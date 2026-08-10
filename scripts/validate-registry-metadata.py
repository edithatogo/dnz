"""Validate registry metadata without network access or credentials."""

from __future__ import annotations

import json
import re
from pathlib import Path
from urllib.parse import urlparse


ROOT = Path(__file__).resolve().parents[1]
SUBMISSION = ROOT / "registry" / "dataset" / "registry-submission.json"
CROISSANT = ROOT / "registry" / "dataset" / "croissant.json"
CARD = ROOT / "registry" / "dataset" / "dataset-card.md"


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return value


def require_https(value: str, label: str) -> None:
    parsed = urlparse(value)
    if parsed.scheme != "https" or not parsed.netloc:
        raise ValueError(f"{label} must be an absolute HTTPS URL")


def main() -> int:
    submission = load_json(SUBMISSION)
    croissant = load_json(CROISSANT)
    if submission.get("schema_version") != 1:
        raise ValueError("unsupported registry submission schema version")
    dataset = submission.get("dataset")
    if not isinstance(dataset, dict):
        raise ValueError("dataset metadata is required")
    for key in ("id", "name", "description", "repository", "provider_rights", "sources"):
        if not dataset.get(key):
            raise ValueError(f"dataset.{key} is required")
    require_https(dataset["repository"], "dataset.repository")
    for source in dataset["sources"]:
        if not isinstance(source, dict) or not source.get("title") or not source.get("role"):
            raise ValueError("each source needs title and role")
        if "url" in source:
            require_https(source["url"], "source.url")
    artifacts = submission.get("artifacts")
    if not isinstance(artifacts, list) or not artifacts:
        raise ValueError("at least one registry artifact is required")
    for artifact in artifacts:
        if not all(artifact.get(key) for key in ("target", "status", "external_gate")):
            raise ValueError("each artifact needs target, status, and external_gate")

    if croissant.get("@type") != "cr:Dataset":
        raise ValueError("croissant metadata must declare cr:Dataset")
    for key in ("name", "description", "url", "license", "distribution", "rights"):
        if not croissant.get(key):
            raise ValueError(f"croissant.{key} is required")
    require_https(croissant["url"], "croissant.url")
    require_https(croissant["license"], "croissant.license")
    if not isinstance(croissant["distribution"], list) or not croissant["distribution"]:
        raise ValueError("croissant.distribution must be non-empty")
    for distribution in croissant["distribution"]:
        require_https(distribution["contentUrl"], "distribution.contentUrl")

    card = CARD.read_text(encoding="utf-8")
    if "rights" not in card.lower() or "DigitalNZ developer documentation" not in card:
        raise ValueError("dataset card must state rights and official provenance")
    if re.search(r"(?:api[_-]?key|token|secret|password)\s*[:=]\s*\S+", card, re.IGNORECASE):
        raise ValueError("dataset card contains a credential-like value")
    print("registry metadata validation: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
