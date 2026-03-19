# slickit

Semantic, LLM-Interpretable Component Kit. Foundation types for composable components.

## What

Four types that let components describe themselves and be discovered, composed, and instantiated across languages:

- **`Kind`** — Agent, Capability, Skill, Flow
- **`Manifest`** — component descriptor (identity, kind, invoke, requires/provides contracts)
- **`TypedConfig`** — config envelope (type_url + opaque JSON)
- **`TypedRegistry`** — type_url → factory → instance

One Rust core, two crusts: Python (PyO3) and TypeScript (wasm-bindgen).

## Install

```bash
# Rust
cargo add slickit --features manifest

# Python
pip install slickit

# TypeScript (npm publish pending)
bun add slickit
```

## Usage

### Rust

```rust
use slick::{Kind, Manifest, TypedConfig, TypedRegistryBuilder};

// Describe a component
let manifest = Manifest {
    kind: Kind::Capability,
    type_url: "mox.tools.v1.Recon".into(),
    description: "Reconnaissance and information gathering".into(),
    invoke: Some("uvx mox/tools/recon".into()),
    requires: vec!["mox.v1.Target".into()],
    provides: vec!["mox.v1.ReconReport".into()],
};

// Register and instantiate
let registry = TypedRegistryBuilder::<String, String>::new()
    .register("example.v1", |value| {
        serde_json::from_value::<String>(value.clone())
            .map_err(|e| e.to_string())
    })
    .build();

let instance = registry.create("example.v1", &serde_json::json!("hello")).unwrap();
```

### Python

```python
from slickit import Kind, Manifest, TypedConfig

manifest = Manifest(
    kind=Kind.Capability,
    type_url="mox.tools.v1.Recon",
    description="Reconnaissance and information gathering",
    invoke="uvx mox/tools/recon",
    requires=["mox.v1.Target"],
    provides=["mox.v1.ReconReport"],
)

# Serialize / deserialize
json_str = manifest.to_json()
manifest2 = Manifest.from_json(json_str)
```

### TypeScript

```typescript
import { Kind, Manifest } from "slickit";

const manifest = Manifest.fromObject({
    kind: "capability",
    type_url: "mox.tools.v1.Recon",
    description: "Reconnaissance and information gathering",
    invoke: "uvx mox/tools/recon",
    requires: ["mox.v1.Target"],
    provides: ["mox.v1.ReconReport"],
});

console.log(manifest.typeUrl); // "mox.tools.v1.Recon"
```

## The Four Kinds

| Kind | Contract | Example |
|------|----------|---------|
| **Agent** | Autonomous reasoning, session-based | Code review agent |
| **Capability** | Stateless function, single invocation | Access control processor |
| **Skill** | Knowledge/context, no execution | Design tokens |
| **Flow** | Orchestrated composition of components | CI pipeline |

## Dependencies

`serde` + `serde_json`. Nothing else.

## License

Apache-2.0 — see [LICENSE](LICENSE).
