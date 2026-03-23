# slickit

Semantic, LLM-Interpretable Component Kit. Foundation types for composable components across the CIX ecosystem.

**Package**: `slickit` on crates.io, PyPI, npm
**Lib name**: `slick` (what you `use`)
**License**: Apache-2.0

## Architecture

Two layers, one crate:

| Layer | Feature | Types |
|-------|---------|-------|
| Runtime | default | `TypedStruct`, `TypedRegistry`, `TypedRegistryBuilder`, `RegistryError` |
| Authoring | `manifest` | `Manifest` |

Bridge: `Manifest.type_url` = `TypedStruct.type_url`.

### MSG Framework

| Layer | What | Where |
|-------|------|-------|
| **M (Mechanics)** | Manifest — structural surface | `slickit` core |
| **S (Semantics)** | Skills — natural language judgment | Referenced via `relations["skills"]` |
| **G (Governance)** | Trust, provenance, policies | External (CIX, x.uma) |

### TypedStruct

Typed structured data envelope. Isomorphic to xDS `TypedStruct`.

```rust
pub struct TypedStruct {
    pub type_url: String,
    pub value: serde_json::Value,
}
```

### TypedRegistry

Generic `type_url → factory → instance`. Inspired by Envoy's `TypedExtensionConfig` + `FactoryRegistry`.

- Builder pattern, immutable after `.build()`
- `Send + Sync` factories
- Two params: `T` (instance type), `E` (error type)
- `register_unique` panics on duplicates (for distributed registration via `inventory`)

### Manifest

Five fields. Pure structure.

```rust
pub struct Manifest {
    pub type_url: String,                        // globally unique identity
    pub source: String,                          // git URL or local path
    pub requires: Vec<String>,                   // input port declarations
    pub provides: Vec<String>,                   // output port declarations
    pub relations: HashMap<String, Vec<String>>, // extensible typed edges
}
```

Well-known relation keys: `skills`, `tested_with`, `replaces`, `depends_on`.

Kind is convention in type_url namespace:
- `cix.agents.*` → Agent
- `cix.commands.*` → Capability
- `cix.skills.*` → Skill
- `cix.flows.*` → Flow

### Crusts

Thin language bindings — no logic, just type bridging:

- `crusts/python/` — PyO3, publishes as `slickit` on PyPI
- `crusts/wasm/` — wasm-bindgen, publishes as `slickit` on npm

Rust is canonical. Crusts mirror.

## What SLICK Does NOT Own

- No execution runtime (geist-run)
- No mediation layer (geist-edge)
- No policy engine (x.uma)
- No component protocol (geist-run)
- No skill content (referenced via relations, not stored)
- No governance (external — CIX, x.uma)

## Conventions

### type_url

Format: `<namespace>.<version>.<Resource>` — e.g., `cix.commands.v1.Recon`

### Namespacing

- `slick.*` — framework types (Manifest, TypedStruct)
- `cix.*` — component ecosystem (commands, agents, flows, skills)
- `<vendor>.*` — vendor components

### source

Git URL or local path:
- `git+https://github.com/mox-labs/tools/recon`
- `git+https://github.com/mox-labs/tools/recon#v1.0.0` (pinned)
- `./tools/recon` (local)

### Skills

Referenced via `relations["skills"]` as URIs:
- `git+https://github.com/mox-labs/skills/recon`
- `./skills/my-custom-skill`

Convention: `--skill` flag outputs SKILL.md for greenfield components.
Brownfield: set `relations.skills` explicitly.

## Downstream Consumers

| System | Uses | Language |
|--------|------|----------|
| geist-edge | `TypedRegistry` for processor registry | Rust |
| x.uma/rumi | `TypedRegistry` for matcher/input/action registries | Rust |
| x.uma/puma | `Manifest`, `TypedStruct` | Python (via slickit) |
| x.uma/bumi | `Manifest`, `TypedStruct` | TypeScript (via slickit) |
| Matrix | `Manifest` for component descriptions | Python (via slickit) |
| mox.hud | `Manifest` for panel descriptors | TypeScript (via slickit) |
| CIX | `Manifest` as component index | Python (via slickit) |

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
cargo test --all-features    # 22 tests
cargo clippy --all-features  # lint
cargo doc --all-features     # docs at target/doc/slick/
```
