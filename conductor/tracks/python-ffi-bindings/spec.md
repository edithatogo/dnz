# Track Specification: Python FFI Bindings

## Overview
This track exports the high-performance Rust core client to Python using `pyo3` and `maturin`. This allows notebook environments (like the GLAM Workbench) to use the Rust client transparently.

## User Stories / Requirements
- As a Jupyter Notebook user, I want to import the Rust-compiled client as a Python library (`import dnz`).
- As a Python developer, I want full type hints and docstrings exposed in the Python package.
- As a pipeline, I want Python dataframes (via PyArrow/pandas) compiled directly from the native client.

## Technical Constraints
- Bridge built using `pyo3` library bindings.
- Package bundling handled via `maturin`.
- Memory sharing using Apache Arrow formats.
