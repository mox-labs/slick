//! slick-ts — TypeScript bindings for slickit via wasm-bindgen.
//!
//! Exposes SLICK's component type system to TypeScript/Svelte:
//! Manifest, TypedStruct.

use wasm_bindgen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Manifest (opaque, JSON bridge)
// ═══════════════════════════════════════════════════════════════════════

/// Component manifest — the structural surface for composition and discovery.
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
    ///   type_url: "cix.commands.v1.Recon",
    ///   source: "git+https://github.com/mox-labs/tools/recon",
    ///   requires: ["cix.v1.Target"],
    ///   provides: ["cix.v1.ReconReport"],
    ///   relations: { skills: ["git+https://github.com/mox-labs/skills/recon"] }
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

    /// Get the type URL.
    #[wasm_bindgen(getter, js_name = "typeUrl")]
    pub fn type_url(&self) -> String {
        self.inner.type_url.clone()
    }

    /// Get the source location.
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.inner.source.clone()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TypedStruct
// ═══════════════════════════════════════════════════════════════════════

/// Typed structured data envelope: type URL + opaque value.
#[wasm_bindgen]
pub struct TypedStruct {
    inner: slick::TypedStruct,
}

#[wasm_bindgen]
impl TypedStruct {
    /// Create from a JS object { type_url: string, value: any }.
    #[wasm_bindgen(js_name = "fromObject")]
    pub fn from_object(obj: JsValue) -> Result<TypedStruct, JsValue> {
        let inner: slick::TypedStruct = serde_wasm_bindgen::from_value(obj)
            .map_err(|e| JsValue::from_str(&format!("invalid typed struct: {e}")))?;
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
