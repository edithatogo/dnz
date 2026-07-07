# Track Specification: Repository Alignment, Security Gates & Archival

## Overview

Establish the `dnz` repo so its workflow, security, archive, and Conductor conventions stay aligned with the adjacent legal-nz repos that already have the expected operational shape.

This track is the coordination umbrella for the repo-local parity work that future implementation tracks will consume. It covers the minimum GitHub Actions security gates, archive publication shape, and Conductor bookkeeping needed for `dnz` to behave like the other maintained repos in this workspace family.

## Functional Requirements

- Audit the current workflow and registry surface against the adjacent repos that already define the expected pattern.
  - Check GitHub Actions layout, job permissions, pinned actions, and branch/PR gating.
  - Check archive publication and metadata handling for Hugging Face and Zenodo parity.
  - Check Conductor registry and track structure for consistency with the repo’s other completed tracks.
- Define the required parity set for `dnz`.
  - Security gating should explicitly cover dependency review and code scanning signals that can fail PRs when high or critical findings exist.
  - Archive publication should have a documented trigger model, artifact layout, and metadata contract.
  - Conductor bookkeeping should keep the active track, index, and any follow-on issue links in one place.
- Record follow-on work as explicit sub-tasks or linked issues.
  - Separate repo-local parity work from any downstream consumer or adjacent repo changes.
  - Keep implementation scope small enough that the next track can execute it without re-deriving the plan.

## Non-Functional Requirements

- The track itself should not invent a new operational pattern when an adjacent repo pattern already exists.
- The parity plan should be reviewable from the repo alone, without needing hidden context from the chat.
- No secrets, credentials, or unpublished archive contents should be added while setting up the track.

## Acceptance Criteria

- A Conductor track exists for repo alignment, security gates, and archival parity.
- The track documents the current gap set and the desired adjacent-repo baseline.
- The track can be used to drive follow-on implementation without re-scoping the work.
- `conductor/tracks.md` points to the new track folder.

## Out of Scope

- Implementing the GitHub Actions, archive publication, or release changes in this step.
- Opening or merging implementation PRs unless a later track explicitly does that work.
- Changing runtime behavior in the CLI or core library as part of the track scaffold.
