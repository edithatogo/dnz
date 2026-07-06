#!/usr/bin/env python3
"""Record submission workflow events as non-committing learning candidates."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


KIND_MESSAGES = {
    "registry": "Registry submission rejected for {artifact} on {target}",
    "review": "Registry review follow-up needed for {artifact} on {target}",
    "skills-feedback": "Skills feedback follow-up needed for {artifact} in {target}",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--kind",
        required=True,
        choices=sorted(KIND_MESSAGES),
        help="Submission event category to record.",
    )
    parser.add_argument(
        "--backlog",
        default="conductor/improvement-backlog.md",
        help="Path to the backlog markdown file.",
    )
    parser.add_argument(
        "--workflow",
        required=True,
        help="Workflow or process name that emitted the event.",
    )
    parser.add_argument(
        "--run-id",
        required=True,
        help="Unique workflow run identifier.",
    )
    parser.add_argument(
        "--run-url",
        required=True,
        help="Run URL for the event evidence.",
    )
    parser.add_argument(
        "--artifact",
        default="unknown artifact",
        help="Artifact or package name affected by the event.",
    )
    parser.add_argument(
        "--target",
        default="unknown target",
        help="Registry, review queue, or feedback target name.",
    )
    parser.add_argument(
        "--feedback",
        default="",
        help="Short human-readable summary of the event outcome.",
    )
    parser.add_argument(
        "--snapshot",
        default="",
        help="Optional file path to write only the new candidate lines for artifact use.",
    )
    return parser.parse_args()


def run_record_learning_candidate(
    backlog: Path,
    message: str,
    evidence: list[str],
    snapshot: str,
) -> None:
    helper = Path(__file__).with_name("record_learning_candidate.py")
    command = [
        sys.executable,
        str(helper),
        "--backlog",
        str(backlog),
        "--message",
        message,
    ]
    for item in evidence:
        command.extend(["--evidence", item])
    if snapshot:
        command.extend(["--snapshot", snapshot])

    subprocess.run(command, check=True)


def main() -> int:
    args = parse_args()
    backlog = Path(args.backlog)
    message = KIND_MESSAGES[args.kind].format(
        artifact=args.artifact,
        target=args.target,
    )

    evidence = [
        f"kind={args.kind}",
        f"workflow={args.workflow}",
        f"run_id={args.run_id}",
        f"run_url={args.run_url}",
    ]
    if args.feedback.strip():
        evidence.append(f"feedback={args.feedback.strip()}")
    if args.artifact.strip():
        evidence.append(f"artifact={args.artifact.strip()}")
    if args.target.strip():
        evidence.append(f"target={args.target.strip()}")

    run_record_learning_candidate(backlog, message, evidence, args.snapshot)
    print(f"Recorded {args.kind} submission event in {backlog}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
