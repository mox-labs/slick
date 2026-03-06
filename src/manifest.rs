//! SLICK component manifest — authoring-layer types (CRD equivalent).
//!
//! These types describe components at authoring time: what kind they are,
//! what they consume and produce. The Composer uses manifests to plan
//! pipelines. The runtime layer ([`super::TypedConfig`], [`super::TypedRegistry`])
//! instantiates them.
//!
//! # CRD/xDS Layering
//!
//! | Layer | Types | Purpose |
//! |-------|-------|---------|
//! | Authoring (CRD) | [`ComponentManifest`], [`ComponentKind`] | Describe + compose |
//! | Runtime (xDS) | [`super::TypedConfig`], [`super::TypedRegistry`] | Instantiate + run |
//!
//! Bridge: `ComponentManifest.type_url` = `TypedConfig.type_url`.
//!
//! # Cross-Surface
//!
//! These types are surface-agnostic. The same [`ComponentKind::Capability`]
//! describes a Rust processor, a Svelte panel, or a Python function.
//! Crusts (PyO3, wasm-bindgen) expose identical types to Python and TypeScript.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The 4 core component kinds. Each implies a different runtime contract.
///
/// | Kind | Contract | Example |
/// |------|----------|---------|
/// | [`Agent`](Self::Agent) | Autonomous reasoning, session-based | Code review agent |
/// | [`Capability`](Self::Capability) | Stateless function, single invocation | AccessControl processor |
/// | [`Skill`](Self::Skill) | Knowledge/context, no execution | Design tokens |
/// | [`Flow`](Self::Flow) | Orchestrated DAG, artifact ledger | CI pipeline |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentKind {
    /// Autonomous reasoning with session state. Perceive → reason → act loop.
    Agent,
    /// Stateless, single invocation. Input → output, no session.
    Capability,
    /// Knowledge and context. Loaded into working memory, not executed.
    Skill,
    /// Orchestrated multi-step process. DAG with consumes/produces contracts.
    Flow,
}

impl fmt::Display for ComponentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::Capability => write!(f, "capability"),
            Self::Skill => write!(f, "skill"),
            Self::Flow => write!(f, "flow"),
        }
    }
}

/// Authoring-layer component manifest (CRD equivalent).
///
/// The `type_url` bridges to the runtime layer — it's the lookup key
/// in [`super::TypedRegistry`].
///
/// # Example
///
/// ```
/// use slick::manifest::{ComponentManifest, ComponentKind};
///
/// let manifest = ComponentManifest {
///     kind: ComponentKind::Capability,
///     type_url: "mox.geist.processors.v1.AccessControl".into(),
///     description: "Deny-first access control processor".into(),
///     consumes: vec![],
///     produces: Some("mox.geist.v1.AuthResult".into()),
/// };
///
/// assert_eq!(manifest.kind, ComponentKind::Capability);
/// assert_eq!(manifest.type_url, "mox.geist.processors.v1.AccessControl");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentManifest {
    /// Behavioral contract category.
    pub kind: ComponentKind,
    /// Globally unique identity. Bridges to runtime layer.
    pub type_url: String,
    /// Human-readable description.
    pub description: String,
    /// Input type URLs this component requires.
    #[serde(default)]
    pub consumes: Vec<String>,
    /// Output type URL this component produces. `None` for Skills (context only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_kind_display() {
        assert_eq!(ComponentKind::Agent.to_string(), "agent");
        assert_eq!(ComponentKind::Capability.to_string(), "capability");
        assert_eq!(ComponentKind::Skill.to_string(), "skill");
        assert_eq!(ComponentKind::Flow.to_string(), "flow");
    }

    #[test]
    fn component_kind_serde_roundtrip() {
        for kind in [
            ComponentKind::Agent,
            ComponentKind::Capability,
            ComponentKind::Skill,
            ComponentKind::Flow,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: ComponentKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn component_kind_deserializes_lowercase() {
        assert_eq!(
            serde_json::from_str::<ComponentKind>(r#""agent""#).unwrap(),
            ComponentKind::Agent
        );
        assert_eq!(
            serde_json::from_str::<ComponentKind>(r#""capability""#).unwrap(),
            ComponentKind::Capability
        );
        assert_eq!(
            serde_json::from_str::<ComponentKind>(r#""skill""#).unwrap(),
            ComponentKind::Skill
        );
        assert_eq!(
            serde_json::from_str::<ComponentKind>(r#""flow""#).unwrap(),
            ComponentKind::Flow
        );
    }

    #[test]
    fn manifest_serializes_roundtrip() {
        let manifest = ComponentManifest {
            kind: ComponentKind::Capability,
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            description: "Deny-first access control".into(),
            consumes: vec![],
            produces: Some("mox.geist.v1.AuthResult".into()),
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let back: ComponentManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kind, ComponentKind::Capability);
        assert_eq!(back.type_url, "mox.geist.processors.v1.AccessControl");
        assert_eq!(back.produces.as_deref(), Some("mox.geist.v1.AuthResult"));
    }

    #[test]
    fn manifest_minimal_fields() {
        let json = r#"{
            "kind": "skill",
            "type_url": "slick.v1.RustMastery",
            "description": "Rust architectural judgment"
        }"#;
        let manifest: ComponentManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.kind, ComponentKind::Skill);
        assert!(manifest.consumes.is_empty());
        assert!(manifest.produces.is_none());
    }

    #[test]
    fn agent_manifest() {
        let manifest = ComponentManifest {
            kind: ComponentKind::Agent,
            type_url: "mox.hud.adapters.v1.Cloudflare".into(),
            description: "Cloudflare runtime adapter".into(),
            consumes: vec![],
            produces: Some("mox.hud.v1.Plane".into()),
        };
        assert_eq!(manifest.kind, ComponentKind::Agent);
    }

    #[test]
    fn flow_manifest_with_consumes_produces() {
        let manifest = ComponentManifest {
            kind: ComponentKind::Flow,
            type_url: "ix.v1.ExperimentFlow".into(),
            description: "Probe → trial → sensor → reading".into(),
            consumes: vec!["ix.v1.Probes".into(), "ix.v1.Subject".into()],
            produces: Some("ix.v1.Readings".into()),
        };

        assert_eq!(manifest.consumes.len(), 2);
        assert_eq!(manifest.produces.as_deref(), Some("ix.v1.Readings"));
    }

    #[test]
    fn capability_manifest() {
        let manifest = ComponentManifest {
            kind: ComponentKind::Capability,
            type_url: "mox.hud.panels.v1.NavPanel".into(),
            description: "File tree explorer panel".into(),
            consumes: vec!["mox.hud.v1.Plane".into()],
            produces: Some("mox.hud.v1.PanelRender".into()),
        };

        assert_eq!(manifest.kind, ComponentKind::Capability);
        assert_eq!(manifest.consumes.len(), 1);
    }

    #[test]
    fn cross_surface_type_url_is_bridge() {
        let manifest = ComponentManifest {
            kind: ComponentKind::Capability,
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            description: "Access control".into(),
            consumes: vec![],
            produces: None,
        };

        let config = crate::TypedConfig {
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            config: serde_json::json!({"default_action": "deny"}),
        };

        assert_eq!(manifest.type_url, config.type_url);
    }
}
