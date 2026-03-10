---
name: registry-patterns
description: >
  Domain knowledge for SLICK's TypedRegistry design. Use when: modifying registry.rs,
  adding registry features, reviewing registry PRs, helping downstream projects adopt
  TypedRegistry, or discussing the BoxedIntoFactory pattern. Covers the Envoy xDS lineage,
  monomorphization-based factory erasure, and the WASM component model connection.
---

# Registry Patterns

SLICK's `TypedRegistry` descends from Envoy's `TypedExtensionConfig` + `FactoryRegistry`, but replaces C++/JVM factory objects with Rust's monomorphization + trait-on-type pattern.

## The BoxedIntoFactory Pattern

**Origin**: Combining axum's `BoxedIntoRoute` mechanism with Envoy's registry pattern. Discovered during x.uma design (2026-02-13).

### The C++/JVM Problem

Envoy (C++) and kuma (Kotlin) require a separate factory object per extension type:

```kotlin
// Kotlin: one factory per extension type
class HeaderInputFactory : DataInputFactory<HttpMessage> {
    override fun typeUrl() = "xuma.http.v1.HeaderInput"
    override fun create(config: Any): DataInput<HttpMessage> { ... }
}
registry.register(HeaderInputFactory())
```

N extension types = N factory structs.

### The Rust Solution

The factory's three jobs split across the type system:

| Factory job | C++/JVM | Rust |
|---|---|---|
| Type URL | `factory.typeUrl()` | Registration site: `.register("type_url", ...)` |
| Config type | `config.unpack(ProtoClass)` | Associated type or closure capture |
| Construction | `factory.create(config)` | Trait method or closure |

SLICK simplifies further — no trait required on the registered type. Just a closure:

```rust
let registry = TypedRegistryBuilder::<String, String>::new()
    .register("example.v1", |value| {
        serde_json::from_value::<String>(value.clone())
            .map_err(|e| e.to_string())
    })
    .build();
```

The closure captures all construction knowledge. After `.register()`, the concrete type is erased — registry only sees `Box<dyn Fn(&Value) -> Result<T, E>>`.

### When Downstream Projects Need Traits

x.uma and geist-edge use trait-on-type for stronger contracts:

```rust
trait IntoDataInput<Ctx>: Sized {
    type Proto: prost::Message + Default;
    fn from_proto(config: Self::Proto) -> Result<Self, MatcherError>;
}

// Registration erases T into a closure stored in TypedRegistry
builder.input::<HeaderInput>("xuma.http.v1.HeaderInput")
```

SLICK provides the generic storage (`TypedRegistry`). Domain projects add traits for their specific contracts.

## Key Properties

- **Zero factory boilerplate** — closures, not factory structs
- **Frozen after build** — `.build()` returns immutable `TypedRegistry`
- **String-based lookup** — `type_url` strings, not `TypeId` (cross-crate compatible)
- **Send + Sync** — factories are thread-safe
- **Composable** — multiple `TypedRegistry` instances for different extension seams (x.uma uses three: inputs, matchers, actions)

## The xDS Lineage

Envoy's extension loading:

```
TypedExtensionConfig { type_url, config_bytes }
    → FactoryRegistry.getFactory(type_url)
    → factory.create(config_bytes)
    → runtime extension instance
```

SLICK's equivalent:

```
TypedConfig { type_url, config: Value }
    → TypedRegistry.create(type_url, &config)
    → factory closure called
    → typed instance
```

Same pattern, Rust idioms. `Value` instead of proto bytes (JSON-first for LLM interop).

## WASM Component Model Connection

The WASM component model solves a related problem at a different level:
- **WASM**: safe cross-language interop via canonical ABI (types cross component boundaries)
- **SLICK**: semantic interop via type_url + Manifest (components describe themselves for LLM composition)

They compose: WASM provides the execution sandbox, SLICK provides the semantic surface. A component can be a WASM module with a SLICK Manifest.

## References

- `~/oss/research/axum-mastery.md` lines 390-512 — BoxedIntoFactory derivation
- `~/oss/research/component-model-mastery.md` — WASM component model protocols
- `~/oss/research/serde-mastery.md` — bounded data model (29-type system)
- `~/oss/research/tower-mastery.md` — composition semantics
