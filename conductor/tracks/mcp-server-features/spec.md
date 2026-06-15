# Track Specification: MCP Server Features

## Overview
This track implements the Model Context Protocol (MCP) server endpoints. It configures tools, resources, and prompt contexts that expose the DigitalNZ search engine capability directly to LLMs over standard input/output channels.

## User Stories / Requirements
- As an AI agent, I want an MCP tool called `search_digitalnz` to find records using keywords and filters.
- As an AI agent, I want an MCP tool called `get_digitalnz_facets` to analyze collection distributions.
- As an AI system, I want structured JSON-RPC errors and payloads returned over standard I/O channels.

## Technical Constraints
- Follow standard MCP specification patterns (JSON-RPC 2.0).
- Run asynchronously under `tokio` runtime.
