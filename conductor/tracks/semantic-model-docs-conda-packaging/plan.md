# Track Plan: Semantic Docs & Conda-forge Packaging

- [x] Task 16.1: Develop a script parsing TMDL definitions to generate Markdown data dictionary tables.
  - *Evidence:* Added `scripts/tmdl-data-dictionary.ps1` and generated `docs/src/content/docs/generated/semantic-model-dictionary.md`.
  - *Commit:* `feat(track-16): task 16.1 - write TMDL data dictionary extraction script`
- [x] Task 16.2: Integrate the generated data dictionaries into the Astro architecture page.
  - *Evidence:* Updated `docs/src/content/docs/guides/architecture.md` and `docs/src/pages/architecture.astro` with semantic model dictionary references.
  - *Commit:* `docs(track-16): task 16.2 - integrate semantic model docs into Astro portal`
- [x] Task 16.3: Create Conda recipe files for distributing `dnz-python`.
  - *Evidence:* Added `conda-recipe/meta.yaml`, `conda-recipe/build.sh`, and `conda-recipe/bld.bat` using `maturin` to build/install the Python FFI wheel.
  - *Commit:* `chore(track-16): task 16.3 - setup Conda recipe configuration files`
- [x] Task 16.4: Integrate TMDL syntax verification tests into the GitHub Actions CI workflow.
  - *Evidence:* Added `scripts/validate-tmdl.ps1`, CI `Validate TMDL Scaffold` step, and `pixi` `tmdl-validate` task.
  - *Commit:* `chore(track-16): task 16.4 - add TMDL verification rules to CI build`
