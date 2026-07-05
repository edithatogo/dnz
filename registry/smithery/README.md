# Smithery Submission Notes

Smithery currently supports:

- URL publishing for public streamable HTTP MCP servers.
- MCPB bundle publishing for local stdio servers.

`dnz-mcp` is currently a local stdio binary, so the live Smithery submission should wait for an MCPB bundle or a public HTTP wrapper. Do not submit the repository as if it already exposes a public streamable HTTP endpoint.

Required configuration values for a future Smithery listing:

- `DIGITALNZ_API_KEY`: required secret.
- `DNZ_CACHE_PATH`: optional local cache path.
- `DNZ_LOG`: optional diagnostics log level.
