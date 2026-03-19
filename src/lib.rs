//! slickit — Semantic, LLM-Interpretable Component Kit.
//!
//! Two layers, both in-memory:
//!
//! | Layer | Feature | Types | Consumer |
//! |-------|---------|-------|----------|
//! | Runtime | default | [`TypedStruct`], [`TypedRegistry`], [`RegistryError`] | geist-edge |
//! | Authoring | `manifest` | [`Kind`], [`Manifest`] | Composer |
//!
//! Bridge: `Manifest.type_url` = `TypedStruct.type_url`.
//!
//! Cross-surface: Rust is canonical. Crusts (PyO3, wasm-bindgen) expose
//! identical types to Python and TypeScript.
//!
//! # Example (runtime layer)
//!
//! ```
//! use slick::{TypedStruct, TypedRegistryBuilder};
//!
//! let registry = TypedRegistryBuilder::<String, String>::new()
//!     .register("example.v1", |value| {
//!         let name: String = serde_json::from_value(value.clone())
//!             .map_err(|e| e.to_string())?;
//!         Ok(name)
//!     })
//!     .build();
//!
//! let val = serde_json::json!("hello");
//! let instance = registry.create("example.v1", &val).unwrap();
//! assert_eq!(instance, "hello");
//! ```

mod registry;

pub use registry::{RegistryError, TypedRegistry, TypedRegistryBuilder, TypedStruct};

#[cfg(feature = "manifest")]
pub mod manifest;

#[cfg(feature = "manifest")]
pub use manifest::{Kind, Manifest};
