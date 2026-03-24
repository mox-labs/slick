# slickit

Semantic, LLM-Interpretable Component Kit. Foundation types for composable components.

## Quick Start

### 1. Make your component composable

Create a `manifest.yaml`:

```yaml
type_url: cix.capability.v1.MyTool
source: git+https://github.com/you/my-tool
requires: [cix.v1.Input]
provides: [cix.v1.Output]
relations:
  skills: [git+https://github.com/you/skills/my-tool]
```

That's it. Your component is now discoverable and composable.

### 2. Add a Skill (optional)

Write a `SKILL.md` — natural language context for LLM composition:

```markdown
# MyTool

Use when you need to transform Input into Output.
Handles edge cases X and Y. Fails on Z — use OtherTool for that.
Composes well with cix.capability.v1.Validator as a pre-step.
```

The Manifest is the structural surface (what). The Skill is the semantic surface (when, why, how).

### 3. Compose

An LLM reads Manifests + Skills, matches requires/provides, generates a Flow:

```yaml
type_url: cix.flows.v1.MyPipeline
source: ./flows/my-pipeline
requires: [cix.v1.RawData]
provides: [cix.v1.Report]
```

The Flow is itself a component. It composes into larger Flows. The registry grows through use.

## What

Three types:

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

let manifest = Manifest {
    type_url: "cix.capability.v1.Recon".into(),
    source: "git+https://github.com/mox-labs/tools/recon".into(),
    requires: vec!["cix.v1.Target".into()],
    provides: vec!["cix.v1.ReconReport".into()],
    relations: HashMap::from([
        ("skills".into(), vec!["git+https://github.com/mox-labs/skills/recon".into()]),
    ]),
};

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
    type_url="cix.capability.v1.Recon",
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
    type_url: "cix.capability.v1.Recon",
    source: "git+https://github.com/mox-labs/tools/recon",
    requires: ["cix.v1.Target"],
    provides: ["cix.v1.ReconReport"],
    relations: { skills: ["git+https://github.com/mox-labs/skills/recon"] },
});
```

## Manifest

Five fields. Pure structure.

| Field | What | Example |
|-------|------|---------|
| **type_url** | Globally unique identity | `cix.capability.v1.Recon` |
| **source** | Where it lives | `git+https://github.com/mox-labs/tools/recon` |
| **requires** | Input port declarations | `[cix.v1.Target]` |
| **provides** | Output port declarations | `[cix.v1.ReconReport]` |
| **relations** | Extensible typed edges | `{skills: [...], tested_with: [...]}` |

Kind is convention in the type_url namespace:

| Namespace | Kind |
|-----------|------|
| `cix.capability.*` | Capability (stateless, input → output) |
| `cix.agents.*` | Agent (autonomous, session-based) |
| `cix.skills.*` | Skill (knowledge, no execution) |
| `cix.flows.*` | Flow (composition of components) |

## Dependencies

`serde` + `serde_json`. Nothing else.

## License

Apache-2.0 — see [LICENSE](LICENSE).
