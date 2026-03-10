//! slick — Python bindings for slickit via PyO3.
//!
//! Exposes SLICK's component type system to Python: Kind, Manifest, TypedConfig.
//!
//! Same types as the Rust canonical definitions, compiled into a native
//! Python extension. No hand-maintained Python types — single source of truth.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Kind
// ═══════════════════════════════════════════════════════════════════════

/// The 4 component kinds. Each implies a different runtime contract.
#[pyclass(frozen, eq, hash)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// Autonomous reasoning, session-based.
    Agent,
    /// Stateless function, single invocation.
    Capability,
    /// Knowledge/context, no execution.
    Skill,
    /// Orchestrated DAG, artifact ledger.
    Flow,
}

#[pymethods]
impl Kind {
    fn __repr__(&self) -> &'static str {
        match self {
            Self::Agent => "Kind.Agent",
            Self::Capability => "Kind.Capability",
            Self::Skill => "Kind.Skill",
            Self::Flow => "Kind.Flow",
        }
    }

    fn __str__(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Capability => "capability",
            Self::Skill => "skill",
            Self::Flow => "flow",
        }
    }
}

impl From<Kind> for slick::Kind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Agent => Self::Agent,
            Kind::Capability => Self::Capability,
            Kind::Skill => Self::Skill,
            Kind::Flow => Self::Flow,
        }
    }
}

impl From<slick::Kind> for Kind {
    fn from(kind: slick::Kind) -> Self {
        match kind {
            slick::Kind::Agent => Self::Agent,
            slick::Kind::Capability => Self::Capability,
            slick::Kind::Skill => Self::Skill,
            slick::Kind::Flow => Self::Flow,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Manifest
// ═══════════════════════════════════════════════════════════════════════

/// Component manifest — describes a component for composition and discovery.
#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Manifest {
    pub kind: Kind,
    pub type_url: String,
    pub description: String,
    pub invoke: Option<String>,
    pub consumes: Vec<String>,
    pub produces: Option<String>,
}

#[pymethods]
impl Manifest {
    #[new]
    #[pyo3(signature = (kind, type_url, description, invoke=None, consumes=None, produces=None))]
    fn new(
        kind: Kind,
        type_url: String,
        description: String,
        invoke: Option<String>,
        consumes: Option<Vec<String>>,
        produces: Option<String>,
    ) -> Self {
        Self {
            kind,
            type_url,
            description,
            invoke,
            consumes: consumes.unwrap_or_default(),
            produces,
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
        format!(
            "Manifest(kind={}, type_url={:?})",
            self.kind.__str__(),
            self.type_url
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TypedConfig
// ═══════════════════════════════════════════════════════════════════════

/// Config envelope: type URL + opaque config JSON.
#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct TypedConfig {
    pub type_url: String,
    pub config: String, // JSON string (opaque to Python)
}

#[pymethods]
impl TypedConfig {
    #[new]
    fn new(type_url: String, config: String) -> PyResult<Self> {
        let _: serde_json::Value = serde_json::from_str(&config)
            .map_err(|e| PyValueError::new_err(format!("invalid JSON config: {e}")))?;
        Ok(Self { type_url, config })
    }

    fn __repr__(&self) -> String {
        format!("TypedConfig(type_url={:?})", self.type_url)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Conversion helpers (PyO3 ↔ slickit)
// ═══════════════════════════════════════════════════════════════════════

fn to_inner_manifest(m: &Manifest) -> slick::Manifest {
    slick::Manifest {
        kind: m.kind.into(),
        type_url: m.type_url.clone(),
        description: m.description.clone(),
        invoke: m.invoke.clone(),
        consumes: m.consumes.clone(),
        produces: m.produces.clone(),
    }
}

fn from_inner_manifest(inner: slick::Manifest) -> Manifest {
    Manifest {
        kind: inner.kind.into(),
        type_url: inner.type_url,
        description: inner.description,
        invoke: inner.invoke,
        consumes: inner.consumes,
        produces: inner.produces,
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Module
// ═══════════════════════════════════════════════════════════════════════

/// Python module: `slickit._native`
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Kind>()?;
    m.add_class::<Manifest>()?;
    m.add_class::<TypedConfig>()?;
    Ok(())
}
