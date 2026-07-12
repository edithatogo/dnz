# Track Specification: DigitalNZ Handoff Notification

## Overview

Create a durable handoff path to notify the DigitalNZ repository or account about the submission and hardening work completed in `dnz`.

This is a communication track. It does not change runtime behavior. Its job is to determine the right GitHub destination, draft the handoff note, and record the outcome so the upstream maintainers can see what changed and what remains open.

## Functional Requirements

- Identify the correct DigitalNZ target.
  - Prefer the canonical repository or account that owns the upstream `digitalnz` source.
  - Confirm whether the destination should be an issue, discussion, release note, or account-level contact.
- Draft a concise handoff message.
  - Summarize what was completed in `dnz`.
  - Note any registry or workflow blockers that remain external.
  - Avoid exposing secrets or unpublished internal state.
- Submit the notification through the best available channel.
  - Use a GitHub issue if the repo accepts issues.
  - Use an equivalent public channel only if that is the better maintainer path.
- Record the outcome.
  - Save the URL, title, and status of the notification.
  - Record any auth or permission failure separately.

## Non-Functional Requirements

- The notification should be respectful, specific, and brief.
- The handoff should not pretend to be a support ticket for unrelated work.
- The track should allow a login checkpoint if GitHub access is required.

## Acceptance Criteria

- A Conductor track exists for the DigitalNZ handoff notification.
- The target repo or account is explicitly identified.
- A notification draft or issue is created, or the blocker is recorded clearly.
- The outcome is linked back to the relevant `dnz` submission work.

## Out of Scope

- Changing the DigitalNZ repository content.
- Open-ended cross-repo implementation work.
- Private contact methods that cannot be represented as public issue or repo metadata.
