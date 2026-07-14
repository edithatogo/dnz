//! PyO3 FFI wrapper exposing dnz-core client to Python.

use dnz_core::Client;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

static PYTHON_RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();

fn python_runtime() -> PyResult<&'static tokio::runtime::Runtime> {
    match PYTHON_RUNTIME
        .get_or_init(|| tokio::runtime::Runtime::new().map_err(|error| error.to_string()))
    {
        Ok(runtime) => Ok(runtime),
        Err(error) => Err(PyRuntimeError::new_err(error.clone())),
    }
}

/// Python wrapper around the native dnz-core Client.
#[pyclass]
pub struct PyClient {
    inner: Client,
}

#[pymethods]
impl PyClient {
    /// Create a new PyClient using an API key string.
    #[new]
    pub fn new(api_key: String) -> Self {
        Self {
            inner: Client::new(api_key),
        }
    }

    /// Set search text and return a builder.
    pub fn search(&self, text: String) -> PyQueryBuilder {
        PyQueryBuilder {
            client: self.inner.clone(),
            text,
            page: 1,
            per_page: 20,
            fields: Vec::new(),
            sort: None,
            direction: None,
            and_filters: HashMap::new(),
            or_filters: HashMap::new(),
            without_filters: HashMap::new(),
        }
    }

    /// Set a record ID and return a metadata builder.
    pub fn record(&self, id: String) -> PyRecordBuilder {
        PyRecordBuilder {
            client: self.inner.clone(),
            id,
            fields: Vec::new(),
        }
    }

    /// Set a record ID and return a More Like This builder.
    pub fn more_like_this(&self, id: String) -> PyMoreLikeThisBuilder {
        PyMoreLikeThisBuilder {
            client: self.inner.clone(),
            id,
            page: 1,
            per_page: 20,
            fields: Vec::new(),
        }
    }
}

/// Python wrapper for query builder chaining.
#[pyclass]
pub struct PyQueryBuilder {
    client: Client,
    text: String,
    page: u32,
    per_page: u32,
    fields: Vec<String>,
    sort: Option<String>,
    direction: Option<String>,
    and_filters: HashMap<String, Vec<String>>,
    or_filters: HashMap<String, Vec<String>>,
    without_filters: HashMap<String, Vec<String>>,
}

#[pymethods]
impl PyQueryBuilder {
    /// Set result page index.
    pub fn page(&mut self, page: u32) {
        self.page = page;
    }

    /// Set result count limit.
    pub fn per_page(&mut self, per_page: u32) {
        self.per_page = per_page;
    }

    /// Restrict the fields returned in the result records.
    pub fn fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }

    /// Sort by field and direction.
    pub fn sort(&mut self, field: String, direction: String) {
        self.sort = Some(field);
        self.direction = Some(direction);
    }

    /// Add an AND constraint filter.
    pub fn and_filter(&mut self, field: String, values: Vec<String>) {
        self.and_filters.insert(field, values);
    }

    /// Add an OR constraint filter.
    pub fn or_filter(&mut self, field: String, values: Vec<String>) {
        self.or_filters.insert(field, values);
    }

    /// Add an exclude filter.
    pub fn without_filter(&mut self, field: String, values: Vec<String>) {
        self.without_filters.insert(field, values);
    }

    /// Run the search query and return results as a JSON string.
    pub fn send(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| {
            let runtime = python_runtime()?;

            let mut builder = self
                .client
                .search(&self.text)
                .page(self.page)
                .per_page(self.per_page)
                .fields(self.fields.clone());

            if let (Some(s), Some(d)) = (self.sort.clone(), self.direction.clone()) {
                builder = builder.sort(s, d);
            }

            for (f, v) in &self.and_filters {
                builder = builder.and_filter(f.clone(), v.clone());
            }

            for (f, v) in &self.or_filters {
                builder = builder.or_filter(f.clone(), v.clone());
            }

            for (f, v) in &self.without_filters {
                builder = builder.without_filter(f.clone(), v.clone());
            }

            let future = builder.send();
            let response = runtime
                .block_on(future)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            let json_str = serde_json::to_string(&response)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(json_str)
        })
    }

    /// Return the normalized response as JSON, preserving the raw adapter contract.
    pub fn send_raw(&self, py: Python<'_>) -> PyResult<String> {
        self.send(py)
    }
}

/// Python wrapper for record metadata lookup.
#[pyclass]
pub struct PyRecordBuilder {
    client: Client,
    id: String,
    fields: Vec<String>,
}

#[pymethods]
impl PyRecordBuilder {
    /// Restrict the metadata fields returned for the record.
    pub fn fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }

    /// Return the normalized record as JSON.
    pub fn send(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| {
            let runtime = python_runtime()?;
            let response = runtime
                .block_on(
                    self.client
                        .record(&self.id)
                        .fields(self.fields.clone())
                        .send(),
                )
                .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
            serde_json::to_string(&response)
                .map_err(|error| PyRuntimeError::new_err(error.to_string()))
        })
    }

    /// Return the normalized record using the raw JSON adapter contract.
    pub fn send_raw(&self, py: Python<'_>) -> PyResult<String> {
        self.send(py)
    }
}

/// Python wrapper for related-record lookup.
#[pyclass]
pub struct PyMoreLikeThisBuilder {
    client: Client,
    id: String,
    page: u32,
    per_page: u32,
    fields: Vec<String>,
}

#[pymethods]
impl PyMoreLikeThisBuilder {
    /// Set result page index.
    pub fn page(&mut self, page: u32) {
        self.page = page;
    }

    /// Set result count limit.
    pub fn per_page(&mut self, per_page: u32) {
        self.per_page = per_page;
    }

    /// Restrict the fields returned in related records.
    pub fn fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }

    /// Return the normalized related-record response as JSON.
    pub fn send(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| {
            let runtime = python_runtime()?;
            let response = runtime
                .block_on(
                    self.client
                        .more_like_this(&self.id)
                        .page(self.page)
                        .per_page(self.per_page)
                        .fields(self.fields.clone())
                        .send(),
                )
                .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
            serde_json::to_string(&response)
                .map_err(|error| PyRuntimeError::new_err(error.to_string()))
        })
    }

    /// Return the normalized response using the raw JSON adapter contract.
    pub fn send_raw(&self, py: Python<'_>) -> PyResult<String> {
        self.send(py)
    }
}

/// The dnz python module definition.
#[pymodule]
fn dnz(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add_class::<PyQueryBuilder>()?;
    m.add_class::<PyRecordBuilder>()?;
    m.add_class::<PyMoreLikeThisBuilder>()?;
    Ok(())
}
