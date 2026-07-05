# Smithery Submission Notes

Smithery currently supports:

- URL publishing for public streamable HTTP MCP servers.
- MCPB bundle publishing for local stdio servers.

`dnz-mcp` is currently a local stdio binary. Use the published MCPB bundle for Smithery's local stdio publishing path. Do not submit the repository as if it already exposes a public streamable HTTP endpoint.

Published bundle:

- URL: https://github.com/edithatogo/dnz/releases/download/v0.1.0/dnz-mcp-0.1.0.mcpb
- SHA-256: `c06f3c4da99b24d3d70545df2e4c802f9d4ecbdb7f4323991d78d104deb41ee6`

## Submission Attempt

Command:

```powershell
smithery mcp publish dist\dnz-mcp-0.1.0.mcpb -n edithatogo/dnz
```

Result:

```text
Deployment failed: 400 {"error":"Invalid input: expected object, received undefined; Invalid input: expected object, received undefined"}
```

The bundle manifest validates with `npx -y @anthropic-ai/mcpb validate`, and the bundle is readable with `npx -y @anthropic-ai/mcpb info`. Treat the Smithery listing as submitted/rejected pending registry-specific follow-up.

Required configuration values for a future Smithery listing:

- `DIGITALNZ_API_KEY`: required secret.
- `DNZ_CACHE_PATH`: optional local cache path.
- `DNZ_LOG`: optional diagnostics log level.
