//! slick — Python bindings for slickit via PyO3.
//!
//! Exposes SLICK's component type system to Python: Manifest, TypedStruct.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════
// Manifest
// ═══════════════════════════════════════════════════════════════════════

/// Component manifest — the structural surface for composition and discovery.
#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Manifest {
    pub type_url: String,
    pub source: String,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
    pub relations: HashMap<String, Vec<String>>,
}

#[pymethods]
impl Manifest {
    #[new]
    #[pyo3(signature = (type_url, source, requires=None, provides=None, relations=None))]
    fn new(
        type_url: String,
        source: String,
        requires: Option<Vec<String>>,
        provides: Option<Vec<String>>,
        relations: Option<HashMap<String, Vec<String>>>,
    ) -> Self {
        Self {
            type_url,
            source,
            requires: requires.unwrap_or_default(),
            provides: provides.unwrap_or_default(),
            relations: relations.unwrap_or_default(),
        }
    }

    /// Serialize to JSON string.
    fn to_json(&self) -> PyResult<String> {
        let inner = to_inner_manifest(self);
        serde_json::to_string_pretty(&inner)
            .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))
    }

    /// Deserialize from JSON string.
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        let inner: slick::Manifest =
            serde_json::from_str(json).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(from_inner_manifest(inner))
    }

    fn __repr__(&self) -> String {
        format!("Manifest(type_url={:?})", self.type_url)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TypedStruct
// ═══════════════════════════════════════════════════════════════════════

/// Typed structured data envelope: type URL + opaque JSON value.
#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct TypedStruct {
    pub type_url: String,
    pub value: String, // JSON string (opaque to Python)
}

#[pymethods]
impl TypedStruct {
    #[new]
    fn new(type_url: String, value: String) -> PyResult<Self> {
        let _: serde_json::Value = serde_json::from_str(&value)
            .map_err(|e| PyValueError::new_err(format!("invalid JSON value: {e}")))?;
        Ok(Self { type_url, value })
    }

    fn __repr__(&self) -> String {
        format!("TypedStruct(type_url={:?})", self.type_url)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Conversion helpers (PyO3 ↔ slickit)
// ═══════════════════════════════════════════════════════════════════════

fn to_inner_manifest(m: &Manifest) -> slick::Manifest {
    slick::Manifest {
        type_url: m.type_url.clone(),
        source: m.source.clone(),
        requires: m.requires.clone(),
        provides: m.provides.clone(),
        relations: m.relations.clone(),
    }
}

fn from_inner_manifest(inner: slick::Manifest) -> Manifest {
    Manifest {
        type_url: inner.type_url,
        source: inner.source,
        requires: inner.requires,
        provides: inner.provides,
        relations: inner.relations,
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Module
// ═══════════════════════════════════════════════════════════════════════

/// Python module: `slickit._native`
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Manifest>()?;
    m.add_class::<TypedStruct>()?;
    Ok(())
}
