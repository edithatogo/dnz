# Glama Submission Notes

Glama accepts open-source MCP servers straight from a GitHub repository.
For `dnz`, the relevant repository fields are:

- Title: `DigitalNZ`
- Description: `Search DigitalNZ records and facets through a local MCP server.`
- License: `MIT`
- Build command: `pixi run build`
- Server command: `cargo run --bin dnz-mcp`

The project is already visible to Glama through the repository URL and the minimal `glama.json` file in the repo root.
Live index publication remains a manual review/submission step and is not automated from this workspace.
