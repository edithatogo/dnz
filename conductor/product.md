# Initial Concept
There are two subrepos that reflect resources related to the digitalnz API. What I want to do is to create a system for interacting with the API, including CLI and MCP, and potentially other forms of APIs, ABIs, skills, workflows and agents. The end products would be tools that would be submitted to the likes of MCP registries, to skills, plugins and extensions registries for all of the major AI tools- e.g. codex, claude, gemini, qwen, opencode, cline, github copilot, etc. Designed using bleeding-edge approaches, strict CI/CD, maximal use of automation, and a Rust-based core.

# Product Guide: DigitalNZ Integration Hub (Rust Core)

## 1. Vision & Purpose
This project is a high-performance integration hub and toolkit built in Rust. It exposes New Zealand's digital heritage (via the DigitalNZ API) to modern computing interfaces, developer ecosystems, and AI-driven workflows. By building a unified, native-compiled CLI tool and a standardized Model Context Protocol (MCP) server, this project delivers lightning-fast and memory-safe digital heritage tools to both human developers and AI agents.

## 2. Target Audience
- **Researchers and Academics:** Accessing high-throughput data harvesting and analysis tools.
- **GLAM Professionals:** Curating collections with responsive tools and automated data pipelines.
- **Developers:** Building tools and workflows using native binaries, shared libraries (ABIs), and robust API bindings.
- **AI Agents & Assistants:** Consuming structured data through MCP servers and custom skills in platforms like Claude, Gemini, Qwen, Codex, Cline, and GitHub Copilot.

## 3. Product Features & Interfaces
- **MCP Server (Rust-based):** A native, asynchronous MCP server implementing Model Context Protocol to expose DigitalNZ searches and facets to LLMs.
- **Command Line Interface (CLI):** A compiled, modern CLI tool (`dnz-cli`) optimized for speed, supporting complex query composition, persistent local caching, harvesting, and stdout JSON output.
- **AI Agent Skills & Workflows:** Configuration-driven skills, workflows, and metadata schemas ready for submission to AI extension registries.
- **Python / FFI bindings:** Potential exports of the core Rust API to Python or C-compatible FFI for integration into existing ecosystems (like the GLAM Workbench).
- **Local analytics and RAG utilities:** Persistent query caching, local vector-search helpers, and on-demand embedding model artifact downloads for offline-friendly workflows.

## 4. Ecosystem & Registry Submissions
The tools produced in this repository will be compiled, packaged, and submitted to:
- Model Context Protocol (MCP) registries.
- IDE Extension & Copilot Registries (e.g., GitHub Copilot, Cline/VSCode).
- Specialized AI tool and agent registries (Gemini, Claude, OpenCode, Qwen, Codex).
