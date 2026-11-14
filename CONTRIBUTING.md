# Contributing to dnz

Contributions improve the DigitalNZ Integration Hub's Rust core, language bindings, command-line tools, and supporting documentation while preserving their explicit contracts.

## Before you start

Read the [README](README.md), [Pixi workspace](pixi.toml), and [Rust workspace](Cargo.toml). Identify the crate, binding, workflow, or data contract affected by the change and coordinate with existing task ownership.

Search existing issues and pull requests to avoid duplicating work. Describe substantial proposals before implementation, and use repository issue or pull request templates when available. Small corrections can go directly into a focused pull request.

For bug reports, include the affected version or commit, environment, minimal reproduction, expected and actual behavior, and sanitized diagnostics. For feature requests, explain the use case and intended outcome.

## Development and validation

The README documents these workspace tasks:

```sh
pixi run build
pixi run test
pixi run clippy
```

Inspect `pixi.toml` for the supported Rust and Python requirements and the current task implementations. Several tasks invoke PowerShell wrappers; record the platform and shell actually used rather than assuming equivalent execution everywhere. Preserve the existing formatting, security, contract, coverage, and metadata checks applicable to the change.

Use synthetic or permitted public fixtures and isolate network-dependent integration evidence. Explain changes to identifiers, schemas, API behavior, or binding compatibility with reproducible examples. A successful local test does not establish a successful archival run, publication, or deployed service. Keep API credentials, restricted records, and large generated artifacts outside source changes.

Update affected documentation and examples alongside behavior changes. Add a focused regression case for a bug fix where executable tests are appropriate. Record checks that were not run and the reason.

## Submitting a pull request

Create a focused branch from the current `main` branch and open a pull request back to it. Keep unrelated formatting, refactoring, and dependency changes separate.

Include:

- The problem, proposed change, and resulting behavior.
- Related issues or existing track references.
- Exact validation commands or inspection steps, results, and limitations.
- Compatibility implications and any data or dependency provenance.

Inspect the complete diff before submission. Required automated checks must pass before merge. Follow the existing repository and account policies; this guide does not introduce a second-person approval requirement or relax existing checks.

## Security and sensitive information

Follow the [account security policy](https://github.com/edithatogo/.github/blob/main/SECURITY.md). Report vulnerabilities privately through GitHub's **Report a vulnerability** option if available. Otherwise request a private reporting contact without disclosing the vulnerability publicly. Keep credentials, personal data, and unredacted logs out of commits, issues, and examples.

## Licensing and conduct

Respect the [existing license](LICENSE) and preserve third-party notices. Submit only material you are entitled to contribute, and document unclear permissions before adding third-party content. Follow the [account code of conduct](https://github.com/edithatogo/.github/blob/main/CODE_OF_CONDUCT.md). This guide adds no CLA, mandatory sign-off, or relicensing condition.

## Reference guidance

- [GitHub contributor guidelines](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/setting-guidelines-for-repository-contributors)
- [OpenSSF Best Practices criteria](https://www.bestpractices.dev/en/criteria)
