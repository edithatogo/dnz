# DNZ Conductor

The conductor is the repository's durable execution and handoff system. It coordinates API parity, compatibility, correctness, platform features, tests, documentation, and releases without replacing the implementation or duplicating issue tracking.

Start with `conductor/README.md`, then read `conductor/manifest.yaml` and `conductor/state.yaml`. `manifest.yaml` is the sole source of track definitions, priorities, dependencies, and acceptance criteria; `state.yaml` is the sole source of status, blockers, verification, and handoff state. `conductor/tracks.md` is only a compatibility index. Contracts under `conductor/contracts/` are executable expectations and drift notes, not substitutes for current official sources.

The conductor loop is: discover → contract → implement → verify → review → document → record. A capability is complete only when core behaviour, relevant adapters, tests, docs, and state agree. Root `task_plan.md`, `progress.md`, and `subagents.yaml` should link to or mirror conductor state rather than define contradictory plans.
