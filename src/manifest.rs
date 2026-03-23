//! SLICK component manifest — the structural surface.
//!
//! A Manifest is pure Mechanics in the MSG framework:
//! - **M (Manifest)**: identity, source, ports, relations — structure
//! - **S (Skills)**: referenced via `relations["skills"]` — natural language
//! - **G (Governance)**: external (CIX, x.uma) — not on the Manifest
//!
//! Bridge: `Manifest.type_url` = `TypedStruct.type_url`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component manifest — the structural surface for composition and discovery.
///
/// Five fields: identity (`type_url`), location (`source`), port declarations
/// (`requires`, `provides`), and extensible typed edges (`relations`).
///
/// # Example
///
/// ```
/// use slick::manifest::Manifest;
/// use std::collections::HashMap;
///
/// let manifest = Manifest {
///     type_url: "cix.commands.v1.Recon".into(),
///     source: "git+https://github.com/mox-labs/tools/recon".into(),
///     requires: vec!["cix.v1.Target".into()],
///     provides: vec!["cix.v1.ReconReport".into()],
///     relations: HashMap::from([
///         ("skills".into(), vec!["git+https://github.com/mox-labs/skills/recon".into()]),
///     ]),
/// };
///
/// assert_eq!(manifest.type_url, "cix.commands.v1.Recon");
/// assert_eq!(manifest.requires, vec!["cix.v1.Target"]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Globally unique identity. Bridges to runtime layer via TypedStruct.type_url.
    /// Format: `<namespace>.<version>.<Resource>`
    pub type_url: String,

    /// Where the component lives. Git URL, local path.
    /// e.g., `git+https://github.com/mox-labs/tools/recon`, `./tools/recon`
    pub source: String,

    /// Input type_urls this component requires (port declarations).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,

    /// Output type_urls this component provides (port declarations).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provides: Vec<String>,

    /// Extensible typed relations. Convention-based keys.
    /// Well-known: `skills`, `tested_with`, `replaces`, `depends_on`.
    /// SLICK stores, never interprets.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub relations: HashMap<String, Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_serializes_roundtrip() {
        let manifest = Manifest {
            type_url: "cix.commands.v1.Recon".into(),
            source: "git+https://github.com/mox-labs/tools/recon".into(),
            requires: vec!["cix.v1.Target".into()],
            provides: vec!["cix.v1.ReconReport".into()],
            relations: HashMap::from([
                ("skills".into(), vec!["git+https://github.com/mox-labs/skills/recon".into()]),
            ]),
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let back: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.type_url, "cix.commands.v1.Recon");
        assert_eq!(back.source, "git+https://github.com/mox-labs/tools/recon");
        assert_eq!(back.requires, vec!["cix.v1.Target"]);
        assert_eq!(back.provides, vec!["cix.v1.ReconReport"]);
        assert_eq!(
            back.relations.get("skills").unwrap(),
            &vec!["git+https://github.com/mox-labs/skills/recon"]
        );
    }

    #[test]
    fn manifest_minimal() {
        let json = r#"{
            "type_url": "cix.skills.v1.RustMastery",
            "source": "git+https://github.com/mox-labs/skills/rust-mastery"
        }"#;
        let manifest: Manifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.type_url, "cix.skills.v1.RustMastery");
        assert!(manifest.requires.is_empty());
        assert!(manifest.provides.is_empty());
        assert!(manifest.relations.is_empty());
    }

    #[test]
    fn manifest_with_relations() {
        let manifest = Manifest {
            type_url: "cix.commands.v1.HttpClient".into(),
            source: "./tools/http-client".into(),
            requires: vec![],
            provides: vec!["cix.v1.HttpResponse".into()],
            relations: HashMap::from([
                ("skills".into(), vec!["./skills/http-patterns.md".into()]),
                ("tested_with".into(), vec!["cix.flows.v1.ApiPipeline".into()]),
                ("replaces".into(), vec!["cix.commands.v0.OldHttpClient".into()]),
            ]),
        };

        assert_eq!(manifest.relations.len(), 3);
        assert!(manifest.relations.contains_key("skills"));
        assert!(manifest.relations.contains_key("tested_with"));
        assert!(manifest.relations.contains_key("replaces"));
    }

    #[test]
    fn cross_surface_type_url_is_bridge() {
        let manifest = Manifest {
            type_url: "cix.commands.v1.AccessControl".into(),
            source: "git+https://github.com/mox-labs/tools/acl".into(),
            requires: vec![],
            provides: vec![],
            relations: HashMap::new(),
        };

        let ts = crate::TypedStruct {
            type_url: "cix.commands.v1.AccessControl".into(),
            value: serde_json::json!({"default_action": "deny"}),
        };

        assert_eq!(manifest.type_url, ts.type_url);
    }

    #[test]
    fn empty_relations_skipped_in_json() {
        let manifest = Manifest {
            type_url: "cix.skills.v1.Patterns".into(),
            source: "./skills/patterns".into(),
            requires: vec![],
            provides: vec![],
            relations: HashMap::new(),
        };

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(!json.contains("relations"));
        assert!(!json.contains("requires"));
        assert!(!json.contains("provides"));
    }

    #[test]
    fn flow_manifest() {
        let manifest = Manifest {
            type_url: "cix.flows.v1.SecurePipeline".into(),
            source: "git+https://github.com/mox-labs/flows/secure-pipeline".into(),
            requires: vec!["cix.v1.Target".into(), "cix.v1.Credentials".into()],
            provides: vec!["cix.v1.SecureReport".into()],
            relations: HashMap::from([
                ("skills".into(), vec!["git+https://github.com/mox-labs/skills/security".into()]),
            ]),
        };

        assert_eq!(manifest.requires.len(), 2);
        assert_eq!(manifest.provides.len(), 1);
    }
}
