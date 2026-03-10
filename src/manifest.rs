//! SLICK component manifest — authoring-layer types.
//!
//! These types describe components at authoring time: what kind they are,
//! what they consume and produce, and how to invoke them. The runtime layer
//! ([`super::TypedConfig`], [`super::TypedRegistry`]) instantiates them.
//!
//! | Layer | Types | Purpose |
//! |-------|-------|---------|
//! | Authoring | [`Manifest`], [`Kind`] | Describe + compose |
//! | Runtime | [`super::TypedConfig`], [`super::TypedRegistry`] | Instantiate + run |
//!
//! Bridge: `Manifest.type_url` = `TypedConfig.type_url`.
//!
//! Cross-surface: the same [`Kind::Capability`] describes a Rust processor,
//! a Svelte panel, or a Python function. Crusts (PyO3, wasm-bindgen) expose
//! identical types to Python and TypeScript.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The 4 component kinds. Each implies a different runtime contract.
///
/// | Kind | Contract | Example |
/// |------|----------|---------|
/// | [`Agent`](Self::Agent) | Autonomous reasoning, session-based | Code review agent |
/// | [`Capability`](Self::Capability) | Stateless function, single invocation | AccessControl processor |
/// | [`Skill`](Self::Skill) | Knowledge/context, no execution | Design tokens |
/// | [`Flow`](Self::Flow) | Orchestrated DAG, artifact ledger | CI pipeline |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Autonomous reasoning with session state. Perceive → reason → act loop.
    Agent,
    /// Stateless, single invocation. Input → output, no session.
    Capability,
    /// Knowledge and context. Loaded into working memory, not executed.
    Skill,
    /// Orchestrated multi-step process. DAG with consumes/produces contracts.
    Flow,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::Capability => write!(f, "capability"),
            Self::Skill => write!(f, "skill"),
            Self::Flow => write!(f, "flow"),
        }
    }
}

/// Component manifest — describes a component for composition and discovery.
///
/// The `type_url` bridges to the runtime layer — it's the lookup key
/// in [`super::TypedRegistry`]. The `invoke` field stores the execution
/// incantation (e.g. a uvx command), opaque to SLICK.
///
/// # Example
///
/// ```
/// use slick::manifest::{Manifest, Kind};
///
/// let manifest = Manifest {
///     kind: Kind::Capability,
///     type_url: "mox.geist.processors.v1.AccessControl".into(),
///     description: "Deny-first access control processor".into(),
///     invoke: Some("uvx mox/tools/access-control".into()),
///     consumes: vec![],
///     produces: Some("mox.geist.v1.AuthResult".into()),
/// };
///
/// assert_eq!(manifest.kind, Kind::Capability);
/// assert_eq!(manifest.type_url, "mox.geist.processors.v1.AccessControl");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Behavioral contract category.
    pub kind: Kind,
    /// Globally unique identity. Bridges to runtime layer.
    pub type_url: String,
    /// Human + LLM readable description.
    pub description: String,
    /// Execution incantation (e.g. `uvx mox/tools/recon`). Opaque to SLICK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke: Option<String>,
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
    fn kind_display() {
        assert_eq!(Kind::Agent.to_string(), "agent");
        assert_eq!(Kind::Capability.to_string(), "capability");
        assert_eq!(Kind::Skill.to_string(), "skill");
        assert_eq!(Kind::Flow.to_string(), "flow");
    }

    #[test]
    fn kind_serde_roundtrip() {
        for kind in [Kind::Agent, Kind::Capability, Kind::Skill, Kind::Flow] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: Kind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn kind_deserializes_lowercase() {
        assert_eq!(
            serde_json::from_str::<Kind>(r#""agent""#).unwrap(),
            Kind::Agent
        );
        assert_eq!(
            serde_json::from_str::<Kind>(r#""capability""#).unwrap(),
            Kind::Capability
        );
        assert_eq!(
            serde_json::from_str::<Kind>(r#""skill""#).unwrap(),
            Kind::Skill
        );
        assert_eq!(
            serde_json::from_str::<Kind>(r#""flow""#).unwrap(),
            Kind::Flow
        );
    }

    #[test]
    fn manifest_serializes_roundtrip() {
        let manifest = Manifest {
            kind: Kind::Capability,
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            description: "Deny-first access control".into(),
            invoke: Some("uvx mox/tools/access-control".into()),
            consumes: vec![],
            produces: Some("mox.geist.v1.AuthResult".into()),
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let back: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kind, Kind::Capability);
        assert_eq!(back.type_url, "mox.geist.processors.v1.AccessControl");
        assert_eq!(back.invoke.as_deref(), Some("uvx mox/tools/access-control"));
        assert_eq!(back.produces.as_deref(), Some("mox.geist.v1.AuthResult"));
    }

    #[test]
    fn manifest_minimal_fields() {
        let json = r#"{
            "kind": "skill",
            "type_url": "slick.v1.RustMastery",
            "description": "Rust architectural judgment"
        }"#;
        let manifest: Manifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.kind, Kind::Skill);
        assert!(manifest.invoke.is_none());
        assert!(manifest.consumes.is_empty());
        assert!(manifest.produces.is_none());
    }

    #[test]
    fn agent_manifest() {
        let manifest = Manifest {
            kind: Kind::Agent,
            type_url: "mox.hud.adapters.v1.Cloudflare".into(),
            description: "Cloudflare runtime adapter".into(),
            invoke: Some("uvx mox/tools/cloudflare-adapter".into()),
            consumes: vec![],
            produces: Some("mox.hud.v1.Plane".into()),
        };
        assert_eq!(manifest.kind, Kind::Agent);
        assert!(manifest.invoke.is_some());
    }

    #[test]
    fn flow_manifest_with_consumes_produces() {
        let manifest = Manifest {
            kind: Kind::Flow,
            type_url: "ix.v1.ExperimentFlow".into(),
            description: "Probe → trial → sensor → reading".into(),
            invoke: None,
            consumes: vec!["ix.v1.Probes".into(), "ix.v1.Subject".into()],
            produces: Some("ix.v1.Readings".into()),
        };

        assert_eq!(manifest.consumes.len(), 2);
        assert_eq!(manifest.produces.as_deref(), Some("ix.v1.Readings"));
        assert!(manifest.invoke.is_none()); // Flows are config, not CLI tools
    }

    #[test]
    fn capability_manifest() {
        let manifest = Manifest {
            kind: Kind::Capability,
            type_url: "mox.hud.panels.v1.NavPanel".into(),
            description: "File tree explorer panel".into(),
            invoke: None,
            consumes: vec!["mox.hud.v1.Plane".into()],
            produces: Some("mox.hud.v1.PanelRender".into()),
        };

        assert_eq!(manifest.kind, Kind::Capability);
        assert_eq!(manifest.consumes.len(), 1);
    }

    #[test]
    fn cross_surface_type_url_is_bridge() {
        let manifest = Manifest {
            kind: Kind::Capability,
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            description: "Access control".into(),
            invoke: None,
            consumes: vec![],
            produces: None,
        };

        let config = crate::TypedConfig {
            type_url: "mox.geist.processors.v1.AccessControl".into(),
            config: serde_json::json!({"default_action": "deny"}),
        };

        assert_eq!(manifest.type_url, config.type_url);
    }

    #[test]
    fn invoke_skipped_when_none() {
        let manifest = Manifest {
            kind: Kind::Skill,
            type_url: "slick.v1.DesignTokens".into(),
            description: "Brand design tokens".into(),
            invoke: None,
            consumes: vec![],
            produces: None,
        };

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(!json.contains("invoke"));
    }
}
