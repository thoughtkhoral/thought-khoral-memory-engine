# Cognee-RS adoption evidence

## Review status

**Decision:** Cannot adopt yet.

**Review date:** 2026-09-16

This evidence was gathered for the approved POC room-scoped-memory plan. It
does not add a dependency, create a Cargo manifest in the repository, or
authorize runtime code. A later task may reopen the decision only after the
license-policy and operational constraints below are explicitly resolved.

## Reviewed authoritative sources

- [Cognee-RS upstream repository](https://github.com/topoteretes/cognee-rs)
- [Cognee-RS v0.2.0 release](https://github.com/topoteretes/cognee-rs/releases/tag/v0.2.0), released 2026-07-30 at commit `400c4bc`
- [Workspace `Cargo.toml`](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/Cargo.toml)
- [Top-level `cognee` crate manifest](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/crates/lib/Cargo.toml)
- [`cognee` 0.2.0 registry metadata](https://crates.io/crates/cognee/0.2.0)
- [`cognee` 0.2.0 API documentation](https://docs.rs/cognee/0.2.0/cognee/)
- [MIT license text](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/LICENSE-MIT)
- [Apache-2.0 license text](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/LICENSE-APACHE)
- [Rust toolchain declaration](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/rust-toolchain.toml)
- [Architecture and crate breakdown](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/architecture.md)
- [Backend and feature documentation](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/tools/backends.md)
- [Native build prerequisites](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/build/prerequisites.md)
- [Configuration and logging](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/configuration.md)
- [Product-analytics behavior](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/observability/send_telemetry.md)
- [Changelog](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/CHANGELOG.md)
- [Release runbook](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/docs/RELEASE.md)
- [Upstream CI workflow](https://github.com/topoteretes/cognee-rs/blob/v0.2.0/.github/workflows/ci.yml)

## License evidence

The top-level crate declares `MIT OR Apache-2.0`. The upstream repository
contains both corresponding license texts. That top-level declaration is not
by itself sufficient for this project because the candidate must be evaluated
with its complete resolved dependency graph.

For an evidence-only temporary manifest using:

```toml
cognee = { version = "=0.2.0", default-features = false, features = ["postgres", "pggraph", "pgvector", "mock-llm"] }
```

Cargo resolved 634 package records for the Linux target, including the
temporary probe package, and the normal dependency tree emitted 2,379 lines.
The resolved metadata included these license expressions in the normal graph:

- `MPL-2.0`: `option-ext 0.2.0`, `resiter 0.5.0`.
- `CECILL-B`: `sophia 0.8.0` and its related Sophia crates.
- `BSL-1.0` or expressions containing it: `xxhash-rust 0.8.18`, `ryu 1.0.23`,
  `ryu-js 0.2.2`, and `whoami 1.6.1` variants.
- `CDLA-Permissive-2.0`: `webpki-roots 0.26.11` and `1.0.9`.
- The remainder included MIT, Apache-2.0, BSD, ISC, Unicode-3.0, CC0,
  Unlicense, Zlib, and similarly declared expressions.

No GPL, AGPL, or SSPL expression was found in the Linux-targeted metadata.
The one `UNSPECIFIED` metadata row was the temporary evidence-probe package,
not an upstream dependency. Other target metadata also surfaced an
`LGPL-2.1-or-later` option in `r-efi`; that target-specific expression must be
included in a future all-target license review. The MPL, CECILL-B, BSL, CDLA,
and LGPL expressions require an explicit project license-policy review; this
dossier does not silently declare them acceptable for distribution. The
BSL-licensed package in particular must not be admitted without an explicit
conclusion from the project’s licensing authority.

## Maturity evidence

- The upstream project has a public `v0.2.0` release, preceded by `v0.1.0`,
  `0.1.1`, and `0.1.3` releases in June and July 2026.
- The `v0.2.0` release contains breaking changes: the umbrella crate was
  renamed from `cognee-lib` to `cognee`, and graph identifiers changed to
  deterministic class-namespaced IDs. The changelog states that pre-existing
  graphs do not merge with the new IDs and require re-`cognify`; no automatic
  migration is provided.
- The repository has CI, community-check, HTTP-parity, publish-dry-run, and
  release workflows. The release runbook requires green CI and a reviewed
  release PR before publishing.
- The upstream workspace is active and substantial, but `0.2.0` is still a
  young release line with ongoing breaking-change and feature work. The
  adoption path therefore requires a pinned release, deterministic fixtures,
  and a rollback path rather than tracking `main`.

## Compatibility evidence

### Rust and build environment

- `cargo info cognee` reported version `0.2.0`, `rust-version: 1.91`, and
  documentation at `https://docs.rs/cognee/0.2.0`.
- The local toolchain is `rustc 1.93.1` / `cargo 1.93.1`, so the declared MSRV
  is compatible with the current validated Rust toolchain.
- The upstream workspace uses Rust edition 2024 and resolver 3.
- The default feature set enables ONNX, Ladybug, pgvector, PostgreSQL,
  tokenizers, document loaders, visualization, mock LLM, and telemetry. The
  upstream prerequisites require a C/C++ compiler, `cmake`, and `protoc`; ONNX
  also downloads a runtime at build time. A POC cannot accept those defaults
  without an explicit feature and build audit.
- An evidence-only `default-features = false` candidate still resolved a large
  graph and retained Lance/DataFusion/Sophia-related packages in its normal
  tree. The exact feature set must be reduced or justified before adoption.

### Storage and process boundary

- Upstream documents expose PostgreSQL relational, graph, and pgvector feature
  gates, but also describe the full relational-plus-graph-plus-vector
  PostgreSQL stack as a remaining adapter milestone. The POC therefore cannot
  assume that one PostgreSQL deployment is operationally proven without an
  integration test.
- Upstream documents state that file-backed graph storage follows a
  single-owning-process model and that cross-process locking is out of scope.
  This is compatible only with one memory-engine process owning its derived
  graph; it is not compatible with multiple racing memory workers sharing a
  file-backed graph.
- Upstream describes dataset labels as not providing a per-dataset graph
  partition. Cognee datasets therefore cannot be treated as the POC’s room
  security boundary. `roomId` partitioning must be enforced by the
  memory-engine-owned graph/provenance layer and every query/write must carry
  an explicit room scope.

### External calls and privacy

- Cognee-RS’s default telemetry feature sends a fire-and-forget HTTP POST to
  `https://test.prometh.ai` for public API calls. The documented payload can
  contain anonymous/persistent identifiers, a caller user ID, and a derived
  identifier from `LLM_API_KEY`.
- The POC cannot enable that behavior implicitly. Any future candidate must
  disable the telemetry feature at compile time or prove an equivalent
  configuration and add a test that no analytics request is emitted.
- Graph extraction requires an LLM and embeddings unless a deterministic mock
  provider is used. The POC must use a mock/cassette path for deterministic
  tests and keep live model configuration outside the gateway process.

## Adoption gate result

| Area | Result | Required condition |
| --- | --- | --- |
| Top-level license | Pass with review | MIT OR Apache-2.0 is declared by the crate and source repository. |
| Transitive license graph | **Not yet acceptable** | Licensing authority must review the normal-graph MPL-2.0, CECILL-B, BSL-1.0, and CDLA-Permissive-2.0 expressions, especially BSL-1.0. |
| Rust compatibility | Pass provisionally | Local Rust 1.93.1 satisfies the reported MSRV 1.91; an actual minimal compile remains required. |
| Build/runtime footprint | Not yet acceptable | Reduce and lock features; prove native-tool and build-time network requirements. |
| Room partitioning | Not provided by Cognee alone | Enforce `roomId` in the memory-engine-owned layer and test cross-room rejection. |
| Storage compatibility | Not yet proven | Prove the selected memory-owned Postgres/pgvector/graph configuration in an isolated integration test. |
| Telemetry/privacy | Not acceptable by default | Disable telemetry and prove no outbound analytics request. |
| Maturity/rollback | Provisionally acceptable for a bounded spike | Pin `0.2.0`, retain graph-only fallback, and do not switch the facilitator port until all later tests pass. |

Because the licensing, footprint, storage, and telemetry conditions are not
yet cleared, the result is **Cannot adopt yet**. Do not add `cognee` to the
repository’s `Cargo.toml` or `Cargo.lock`. Continue Phase 2 only with
backend-neutral graph/provenance interfaces and deterministic fixtures.

## Reproducibility commands

These commands were run against a temporary manifest outside the repository;
the temporary manifest and Cargo cache are not project artifacts.

```bash
rustc --version
cargo --version
CARGO_HOME=/private/tmp/cognee-rs-evidence-cargo-home cargo info cognee
CARGO_HOME=/private/tmp/cognee-rs-evidence-cargo-home cargo metadata \
  --format-version 1 \
  --filter-platform x86_64-unknown-linux-gnu \
  --manifest-path /private/tmp/cognee-rs-evidence.5gEZwX/Cargo.toml
CARGO_HOME=/private/tmp/cognee-rs-evidence-cargo-home cargo tree \
  --manifest-path /private/tmp/cognee-rs-evidence.5gEZwX/Cargo.toml \
  --edges normal
```

The first sandboxed registry attempt failed because the restricted environment
could not resolve `index.crates.io`; the same read-only evidence query then
completed with narrowly scoped network permission. No repository dependency or
runtime file was changed.

## Reopen conditions

Reopen Task 2 only when all of these are recorded in an updated evidence
review:

1. Licensing authority accepts the full resolved graph or a smaller reviewed
   feature graph with no unapproved BSL/MPL/CECILL/CDLA package.
2. The exact feature set builds with the project toolchain without an
   unapproved build-time download or native tool requirement.
3. Telemetry is disabled and a test proves no analytics request is emitted.
4. A memory-engine-owned storage configuration proves room-scoped graph and
   provenance behavior without gateway event-log credentials.
5. The pinned release and transitive graph are captured in a reviewed lockfile
   before any facilitator-port work begins.
