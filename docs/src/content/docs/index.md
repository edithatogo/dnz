---
title: DigitalNZ Integration Hub
description: Astro/Starlight documentation entrypoint for DigitalNZ Integration Hub.
---

# DigitalNZ Integration Hub

This repository owns a local Astro/Starlight documentation surface for the Legal NZ system.

## Standard commands

- `npm run docs:dev`
- `npm run docs:build`
- `npm run docs:check`

## Source material

Existing Astro docs app under docs/ plus Rust, MCP, and Python FFI references.

## API reference

- [API Documentation Map](./generated/api-documentation-map.md)
- [Major DigitalNZ Collections](./generated/digitalnz-major-collections.md)

## Policy

This docs app follows the root Legal NZ Astro documentation policy: Astro is the published documentation shell, while generated API, CLI, dataset, or prose references feed into Astro rather than replacing it.
