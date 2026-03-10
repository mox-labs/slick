# Contributing

## Setup

```bash
# Clone and test
git clone https://github.com/mox-labs/slick.git
cd slick
cargo test --all-features
```

## Conventions

**Commits**: [Conventional Commits](https://www.conventionalcommits.org/)

```
feat(manifest): add invoke field
fix(registry): handle empty config
refactor(wasm): simplify Kind conversion
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`

**Versioning**: [SemVer](https://semver.org/). The version in root `Cargo.toml` is the source of truth. Crust versions follow.

**Branching**: One branch per change, branch from `main`.

```bash
git checkout -b feat/your-feature origin/main
```

## Structure

```
src/
  lib.rs          # re-exports
  registry.rs     # TypedConfig, TypedRegistry, TypedRegistryBuilder
  manifest.rs     # Kind, Manifest (feature = "manifest")
crusts/
  python/         # PyO3 bindings → PyPI
  wasm/           # wasm-bindgen bindings → npm
```

Rust is canonical. Crusts are thin wrappers — no logic, just type bridging.

## Testing

```bash
cargo test --all-features    # all tests + doc tests
cargo clippy --all-features  # lint
cargo doc --all-features     # docs
```

## Releasing

Push a version tag to trigger the release workflow:

```bash
# Update version in Cargo.toml, crusts/python/Cargo.toml, crusts/wasm/Cargo.toml
git tag v0.1.0
git push origin v0.1.0
```

Publishes to crates.io, PyPI, and npm.

## License

By contributing, you agree that your contributions will be licensed under Apache-2.0.
