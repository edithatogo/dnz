# Python 3.14-only runtime

## Overview

Make Python 3.14 the sole supported Python runtime for DNZ while preserving the Rust API, CLI, MCP, schemas, and release semantics.

## Functional requirements

- Align Pixi constraints and environments, Python package metadata, workflows, scripts, tooling targets, classifiers, and documentation on Python 3.14.
- Regenerate affected lockfiles with the repository package manager.
- Upgrade only dependencies proven necessary for Python 3.14 compatibility.
- Add an executable policy check that rejects supported-runtime declarations below Python 3.14.

## Non-functional requirements

- Preserve the dirty primary checkout by working in an isolated clean clone.
- Keep normal tests offline and mock-backed.
- Preserve credential-redaction and release-safety boundaries.
- Avoid unrelated Rust or Python dependency modernization.

## Acceptance criteria

- Every supported Python runtime declaration resolves to Python 3.14.
- Pixi lock/install and relevant Python binding/package checks pass.
- Rust formatting, workspace tests, and Clippy with warnings denied pass.
- Hosted CI passes for the migration branch.
- Exact validation evidence is recorded before completion.

## Out of scope

- Live DigitalNZ API calls.
- Publication or release actions.
- Rust API, CLI, MCP, or schema redesign.
- Unrelated dependency upgrades.
