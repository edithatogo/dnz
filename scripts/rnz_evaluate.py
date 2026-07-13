#!/usr/bin/env python3
"""Offline RNZ quality and pilot coverage evaluation; no production downloads."""

from __future__ import annotations

import argparse
import json
import re
from difflib import SequenceMatcher
from datetime import datetime, timezone
from pathlib import Path


def normalize(text: str) -> str:
    return re.sub(r"\s+", " ", text.casefold()).strip()


def transcript_score(reference: str, predicted: str) -> dict[str, float]:
    ref, pred = normalize(reference), normalize(predicted)
    ref_words, pred_words = ref.split(), pred.split()
    distance = abs(len(ref_words) - len(pred_words))
    distance += sum(left != right for left, right in zip(ref_words, pred_words))
    return {
        "word_error_rate": round(distance / max(1, len(ref_words)), 6),
        "sequence_similarity": round(SequenceMatcher(None, ref, pred).ratio(), 6),
    }


def boundary_score(reference: list[float], predicted: list[float], tolerance: float = 2.0) -> dict[str, float]:
    matched = sum(1 for point in predicted if any(abs(point - expected) <= tolerance for expected in reference))
    precision = matched / max(1, len(predicted))
    recall = matched / max(1, len(reference))
    return {"precision": round(precision, 6), "recall": round(recall, 6), "f1": round(2 * precision * recall / max(1e-9, precision + recall), 6)}


def evaluate(fixture: dict, pilot: dict) -> dict:
    fixture_rows = []
    for item in fixture.get("transcripts", []):
        fixture_rows.append({"id": item["id"], "transcript": transcript_score(item["reference"], item["predicted"])})
    boundaries = [boundary_score(item["reference"], item["predicted"], item.get("tolerance_seconds", 2.0)) for item in fixture.get("boundaries", [])]
    processed = int(pilot.get("processed_count", 0))
    target = int(pilot.get("target_count", 100))
    return {
        "schema_version": 1,
        "generated_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "fixture": {"transcripts": fixture_rows, "boundaries": boundaries},
        "pilot": {"processed_count": processed, "target_count": target, "coverage_ratio": round(min(1.0, processed / max(1, target)), 6), "manual_review_required": processed >= target},
        "promotion": {"canonical_output_unchanged": True, "production_enrichment_gate": "manual_review_required", "blocked": processed < target},
        "limitations": [
            "Synthetic fixture scores do not estimate production accuracy.",
            "Pilot promotion remains blocked until the target is processed and manually reviewed.",
            "Acoustic, language, entity and duplicate-model claims require domain-specific reviewed labels.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture", type=Path, required=True)
    parser.add_argument("--pilot", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    report = evaluate(json.loads(args.fixture.read_text(encoding="utf-8")), json.loads(args.pilot.read_text(encoding="utf-8")))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps({"blocked": report["promotion"]["blocked"], "output": str(args.output)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
