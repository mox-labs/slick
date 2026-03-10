//! slick-ts — TypeScript bindings for slickit via wasm-bindgen.
//!
//! Exposes SLICK's component type system to TypeScript/Svelte: Kind,
//! Manifest, TypedConfig.
//!
//! Built with `wasm-pack build --target web`. Produces `.d.ts` type definitions
//! alongside the WASM binary.

use wasm_bindgen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Kind
// ═══════════════════════════════════════════════════════════════════════

/// The 4 component kinds.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Agent = 0,
    Capability = 1,
    Skill = 2,
    Flow = 3,
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
// Manifest (opaque, JSON bridge)
// ═══════════════════════════════════════════════════════════════════════

/// Component manifest — describes a component for composition and discovery.
#[wasm_bindgen]
pub struct Manifest {
    inner: slick::Manifest,
}

#[wasm_bindgen]
impl Manifest {
    /// Create a Manifest from a JS object.
    ///
    /// Expected shape:
    /// ```js
    /// {
    ///   kind: "agent" | "capability" | "skill" | "flow",
    ///   type_url: "mox.geist.processors.v1.AccessControl",
    ///   description: "...",
    ///   invoke: "uvx mox/tools/access-control",  // optional
    ///   consumes: [],
    ///   produces: "..."
    /// }
    /// ```
    #[wasm_bindgen(js_name = "fromObject")]
    pub fn from_object(obj: JsValue) -> Result<Manifest, JsValue> {
        let inner: slick::Manifest = serde_wasm_bindgen::from_value(obj)
            .map_err(|e| JsValue::from_str(&format!("invalid manifest: {e}")))?;
        Ok(Self { inner })
    }

    /// Serialize to a JS object.
    #[wasm_bindgen(js_name = "toObject")]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))
    }

    /// Create from a JSON string.
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<Manifest, JsValue> {
        let inner: slick::Manifest = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("invalid JSON: {e}")))?;
        Ok(Self { inner })
    }

    /// Serialize to a JSON string.
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))
    }

    /// Get the component kind.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> Kind {
        self.inner.kind.into()
    }

    /// Get the type URL.
    #[wasm_bindgen(getter, js_name = "typeUrl")]
    pub fn type_url(&self) -> String {
        self.inner.type_url.clone()
    }

    /// Get the description.
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.inner.description.clone()
    }

    /// Get the invoke incantation.
    #[wasm_bindgen(getter)]
    pub fn invoke(&self) -> Option<String> {
        self.inner.invoke.clone()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TypedConfig
// ═══════════════════════════════════════════════════════════════════════

/// Config envelope: type URL + opaque config.
#[wasm_bindgen]
pub struct TypedConfig {
    inner: slick::TypedConfig,
}

#[wasm_bindgen]
impl TypedConfig {
    /// Create from a JS object { type_url: string, config: any }.
    #[wasm_bindgen(js_name = "fromObject")]
    pub fn from_object(obj: JsValue) -> Result<TypedConfig, JsValue> {
        let inner: slick::TypedConfig = serde_wasm_bindgen::from_value(obj)
            .map_err(|e| JsValue::from_str(&format!("invalid config: {e}")))?;
        Ok(Self { inner })
    }

    /// Serialize to a JS object.
    #[wasm_bindgen(js_name = "toObject")]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))
    }

    /// Get the type URL.
    #[wasm_bindgen(getter, js_name = "typeUrl")]
    pub fn type_url(&self) -> String {
        self.inner.type_url.clone()
    }
}
