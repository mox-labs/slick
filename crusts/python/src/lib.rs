//! slick — Python bindings for slickit via PyO3.
//!
//! Exposes SLICK's component type system to Python: ComponentKind,
//! ComponentManifest, TypedConfig.
//!
//! Same types as the Rust canonical definitions, compiled into a native
//! Python extension. No hand-maintained Python types — single source of truth.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// ComponentKind
// ═══════════════════════════════════════════════════════════════════════

/// The 4 core component kinds. Each implies a different runtime contract.
#[pyclass(frozen, eq, hash)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentKind {
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
impl ComponentKind {
    fn __repr__(&self) -> &'static str {
        match self {
            Self::Agent => "ComponentKind.Agent",
            Self::Capability => "ComponentKind.Capability",
            Self::Skill => "ComponentKind.Skill",
            Self::Flow => "ComponentKind.Flow",
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

impl From<ComponentKind> for slick::ComponentKind {
    fn from(kind: ComponentKind) -> Self {
        match kind {
            ComponentKind::Agent => Self::Agent,
            ComponentKind::Capability => Self::Capability,
            ComponentKind::Skill => Self::Skill,
            ComponentKind::Flow => Self::Flow,
        }
    }
}

impl From<slick::ComponentKind> for ComponentKind {
    fn from(kind: slick::ComponentKind) -> Self {
        match kind {
            slick::ComponentKind::Agent => Self::Agent,
            slick::ComponentKind::Capability => Self::Capability,
            slick::ComponentKind::Skill => Self::Skill,
            slick::ComponentKind::Flow => Self::Flow,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// ComponentManifest
// ═══════════════════════════════════════════════════════════════════════

/// Authoring-layer component manifest (CRD equivalent).
#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ComponentManifest {
    pub kind: ComponentKind,
    pub type_url: String,
    pub description: String,
    pub consumes: Vec<String>,
    pub produces: Option<String>,
}

#[pymethods]
impl ComponentManifest {
    #[new]
    #[pyo3(signature = (kind, type_url, description, consumes=None, produces=None))]
    fn new(
        kind: ComponentKind,
        type_url: String,
        description: String,
        consumes: Option<Vec<String>>,
        produces: Option<String>,
    ) -> Self {
        Self {
            kind,
            type_url,
            description,
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
        let inner: slick::ComponentManifest =
            serde_json::from_str(json).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(from_inner_manifest(inner))
    }

    fn __repr__(&self) -> String {
        format!(
            "ComponentManifest(kind={}, type_url={:?})",
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

fn to_inner_manifest(m: &ComponentManifest) -> slick::ComponentManifest {
    slick::ComponentManifest {
        kind: m.kind.into(),
        type_url: m.type_url.clone(),
        description: m.description.clone(),
        consumes: m.consumes.clone(),
        produces: m.produces.clone(),
    }
}

fn from_inner_manifest(inner: slick::ComponentManifest) -> ComponentManifest {
    ComponentManifest {
        kind: inner.kind.into(),
        type_url: inner.type_url,
        description: inner.description,
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
    m.add_class::<ComponentKind>()?;
    m.add_class::<ComponentManifest>()?;
    m.add_class::<TypedConfig>()?;
    Ok(())
}
