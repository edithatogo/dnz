# Publication & Deployment Plan: DigitalNZ Integration Hub

This document details the release, packaging, and registry publication workflows for the DigitalNZ Integration Hub.

---

## 1. Distribution Matrix

| Component | Target Registry | Package Format | Distribution Mechanism |
| :--- | :--- | :--- | :--- |
| **`dnz-core`** | Crates.io | Rust Library Crate | `cargo publish` |
| **`dnz-cli`** | GitHub Releases / Homebrew / WinGet | Pre-compiled native binaries | GitHub Actions release matrix / Homebrew Formula / WinGet Manifest |
| **`dnz-mcp`** | MCP Registry | Stdio Binary | Registered at [glam-mcp](https://github.com/modelcontextprotocol/servers) |
| **`dnz-python`** | PyPI (Python Package Index) | Maturin Wheel Binary | `maturin publish` |
| **`docs`** | GitHub Pages / Cloudflare Pages | HTML / CSS | `astro build` + Actions Deploy |

---

## 2. Release Workflows

### Phase 1: Local Pre-flight Verification
Before tagging a release:
1. Bump version number matching SemVer in:
   - `crates/dnz-core/Cargo.toml`
   - `crates/dnz-cli/Cargo.toml`
   - `crates/dnz-mcp/Cargo.toml`
   - `crates/dnz-python/Cargo.toml`
   - `docs/package.json`
2. Run validation check suites:
   ```bash
   pixi run clippy
   pixi run test
   ```

### Phase 2: Tagging & Release Compilations
1. Tag the release commit and push to remote:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```
2. **GitHub Actions Trigger:** The push event activates `.github/workflows/release.yml` which compiles native binaries for:
   - Windows (`x86_64`)
   - macOS (`x86_64` & Apple Silicon)
   - Linux (`x86_64`)
3. The compiled binaries are automatically attached to the GitHub release page.

### Phase 3: PyPI Publication (`dnz-python`)
Maturin automates compilation of Python FFI wheels. To publish to PyPI:
```bash
cd crates/dnz-python
maturin publish --username __token__ --password ${{ secrets.PYPI_API_TOKEN }}
```

### Phase 4: Registry Submissions
1. **MCP Registry:** Add the server configuration schema from [mcp-manifest.json](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/dnz/crates/dnz-mcp/mcp-manifest.json) into the official community servers list (e.g. at [github.com/modelcontextprotocol/servers](https://github.com/modelcontextprotocol/servers)).
2. **Cline/VSCode Extension Registries:** Package and list this server as a one-click install inside Open-VSX and VSCode extension marketplaces.

### Phase 5: Documentation Deployment
The Astro site in `docs/` is compiled and published:
```bash
cd docs
npm run build
```
The build artifacts in `dist/` are automatically synced to GitHub Pages using the `actions/deploy-pages` step.
