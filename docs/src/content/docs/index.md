---
title: DigitalNZ Integration Hub
description: Bleeding-edge Rust-based client library, CLI, and MCP server for digital heritage.
template: splash
hero:
  tagline: Lightning-fast, memory-safe API integration with local semantic vector search, autopilot crawlers, and RAG optimization.
  actions:
    - text: Read the Docs
      link: /guides/getting-started/
      icon: right-arrow
      variant: primary
    - text: View Workspace
      link: https://github.com/GLAM-Workbench/digitalnz
      icon: external
---

import { Card, CardGrid } from '@astrojs/starlight/components';

## Key Capabilities

<CardGrid stencil>
  <Card title="Model Context Protocol (MCP)" icon="rocket">
    Standard-compliant async stdio server exposing New Zealand's cultural archives directly to LLM agents.
  </Card>
  <Card title="Agentic Autopilot Crawler" icon="magnifier">
    Recursively partitions search queries by year to completely bypass the 1,000-record API limit boundaries.
  </Card>
  <Card title="Local Semantic Search" icon="setting">
    Offline vector similarity search and cosine ranking computed natively on-device using the `candle` ML framework.
  </Card>
  <Card title="Python FFI Bindings" icon="laptop">
    Imports compiled Rust client performance straight into Jupyter Notebooks and Pandas workflows via PyO3.
  </Card>
</CardGrid>
