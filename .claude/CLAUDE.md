# slickit

Semantic, LLM-Interpretable Component Kit. Foundation types for composable components across the mox ecosystem.

**Package**: `slickit` on crates.io, PyPI, npm
**Lib name**: `slick` (what you `use`)
**License**: Apache-2.0

## Architecture

Two layers, one crate:

| Layer | Feature | Types |
|-------|---------|-------|
| Runtime | default | `TypedConfig`, `TypedRegistry`, `TypedRegistryBuilder`, `RegistryError` |
| Authoring | `manifest` | `Kind`, `Manifest` |

Bridge: `Manifest.type_url` = `TypedConfig.type_url`.

### TypedRegistry

Generic `type_url → factory → instance`. Inspired by Envoy's `TypedExtensionConfig` + `FactoryRegistry`, but uses Rust's monomorphization + trait-on-type pattern instead of C++/JVM factory objects (see BoxedIntoFactory pattern in `~/oss/research/axum-mastery.md`).

- Builder pattern, immutable after `.build()`
- `Send + Sync` factories
- Two params: `T` (instance type), `E` (error type)
- `register_unique` panics on duplicates (for distributed registration via `inventory`)

### Manifest

Component descriptor for composition and discovery:

```rust
pub struct Manifest {
    pub kind: Kind,                 // Agent | Capability | Skill | Flow
    pub type_url: String,           // globally unique identity
    pub description: String,        // human + LLM readable
    pub invoke: Option<String>,     // execution incantation, opaque to SLICK
    pub requires: Vec<String>,      // input type_urls
    pub provides: Vec<String>,      // output type_urls
}
```

### The Four Kinds

| Kind | Contract |
|------|----------|
| Agent | Autonomous reasoning, session-based |
| Capability | Stateless function, single invocation |
| Skill | Knowledge/context, no execution |
| Flow | Orchestrated composition of components |

### Crusts

Thin language bindings — no logic, just type bridging:

- `crusts/python/` — PyO3, publishes as `slickit` on PyPI
- `crusts/wasm/` — wasm-bindgen, publishes as `slickit` on npm

Rust is canonical. Crusts mirror.

## What SLICK Does NOT Own

- No circuit/DAG types (geist-edge)
- No circuit executor (geist-edge)
- No policy engine (x.uma)
- No component protocol (geist-edge)
- No skill content (convention: `invoke --skill`)

## Conventions

### type_url

Format: `<namespace>.<version>.<Resource>` — e.g., `mox.geist.processors.v1.AccessControl`

### Skill discovery

Convention, not infrastructure:
- Capability/Agent: `invoke --skill` outputs SKILL.md
- Skill (Kind): the component IS the semantic surface
- Flow: `SKILL.md` in repo by convention

### invoke field

Opaque to SLICK. Stores the execution incantation (e.g., `uvx mox/tools/recon`). SLICK stores it, never interprets it.

## Downstream Consumers

| System | Uses | Language |
|--------|------|----------|
| geist-edge | `TypedRegistry` for processor registry | Rust |
| x.uma/rumi | `TypedRegistry` for matcher/input/action registries | Rust |
| x.uma/puma | `Manifest`, `Kind`, `TypedConfig` | Python (via slickit) |
| x.uma/bumi | `Manifest`, `Kind`, `TypedConfig` | TypeScript (via slickit) |
| Matrix | `Manifest` for component descriptions | Python (via slickit) |
| mox.hud | `Manifest` for panel descriptors | TypeScript (via slickit) |
| CIX | `Manifest` as component index | Python (via slickit) |

Migration plans live in each project's `scratch/slick-migration.md`.

## Versioning

SemVer. Root `Cargo.toml` is the source of truth. Crust versions follow.

When bumping: update `Cargo.toml`, `crusts/python/Cargo.toml`, `crusts/wasm/Cargo.toml`.

## Releasing

```bash
git tag v<version>
git push origin v<version>
```

Triggers `.github/workflows/release.yml` → test gate → publishes to crates.io, PyPI, npm.

## Testing

```bash
cargo test --all-features    # 24 unit + 2 doc tests
cargo clippy --all-features  # lint
cargo doc --all-features     # docs at target/doc/slick/
```

## Guild Vocabulary

| Guild Term | SLICK Equivalent |
|------------|-----------------|
| Boundary | Crust (PyO3/wasm-bindgen binding layer) |
| Port | `TypedRegistry` (accepts any factory matching the type contract) |
| Adapter | Factory closure registered via `.register()` |
| Domain | `Kind` + `Manifest` (the component model) |
| Valve | Not applicable (SLICK has no flow control) |
