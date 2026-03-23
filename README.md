# slickit

Semantic, LLM-Interpretable Component Kit. Foundation types for composable components.

## What

Three types that let components be discovered, composed, and instantiated across languages:

- **`Manifest`** — structural surface (type_url, source, requires, provides, relations)
- **`TypedStruct`** — typed data envelope (type_url + opaque JSON value)
- **`TypedRegistry`** — type_url → factory → instance

MSG framework: **Manifest = Mechanics. Skills = Semantics. Governance = external.**

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
use slick::{Manifest, TypedStruct, TypedRegistryBuilder};
use std::collections::HashMap;

// Describe a component
let manifest = Manifest {
    type_url: "cix.commands.v1.Recon".into(),
    source: "git+https://github.com/mox-labs/tools/recon".into(),
    requires: vec!["cix.v1.Target".into()],
    provides: vec!["cix.v1.ReconReport".into()],
    relations: HashMap::from([
        ("skills".into(), vec!["git+https://github.com/mox-labs/skills/recon".into()]),
    ]),
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
from slickit import Manifest, TypedStruct

manifest = Manifest(
    type_url="cix.commands.v1.Recon",
    source="git+https://github.com/mox-labs/tools/recon",
    requires=["cix.v1.Target"],
    provides=["cix.v1.ReconReport"],
    relations={"skills": ["git+https://github.com/mox-labs/skills/recon"]},
)

json_str = manifest.to_json()
manifest2 = Manifest.from_json(json_str)
```

### TypeScript

```typescript
import { Manifest } from "slickit";

const manifest = Manifest.fromObject({
    type_url: "cix.commands.v1.Recon",
    source: "git+https://github.com/mox-labs/tools/recon",
    requires: ["cix.v1.Target"],
    provides: ["cix.v1.ReconReport"],
    relations: { skills: ["git+https://github.com/mox-labs/skills/recon"] },
});

console.log(manifest.typeUrl); // "cix.commands.v1.Recon"
```

## Manifest

Five fields. Pure structure.

```yaml
type_url: cix.commands.v1.Recon
source: git+https://github.com/mox-labs/tools/recon
requires: [cix.v1.Target]
provides: [cix.v1.ReconReport]
relations:
  skills: [git+https://github.com/mox-labs/skills/recon]
  tested_with: [cix.flows.v1.ReconPipeline]
```

- **type_url** — globally unique identity (namespace convention for kind)
- **source** — where it lives (git URL, local path)
- **requires** — input port declarations
- **provides** — output port declarations
- **relations** — extensible typed edges (skills, tested_with, replaces, etc.)

## Dependencies

`serde` + `serde_json`. Nothing else.

## License

Apache-2.0 — see [LICENSE](LICENSE).
