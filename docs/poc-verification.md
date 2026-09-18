# Room-scoped memory POC verification

## Outcome

The graph-only memory capability and mediated ingestion boundary are verified.
The deterministic gateway `Decision:` facilitator remains the active provider.
The optional Phase 4 provider switch is deferred because Cognee-RS adoption is
still recorded as `Cannot adopt yet`, and the memory engine has no proven
proposal-generating implementation beyond the validated draft port adapter.

## Evidence and revisions

- Cognee evidence: `docs/dependency-evidence/cognee-rs.md`; result:
  `Cannot adopt yet`; no Cognee dependency is present in `Cargo.toml`.
- Mediated transport: `docs/mediated-ingestion.md`; private authenticated
  HTTP ingestion, bounded gateway queue, post-commit dispatch.
- Active facilitator: deterministic `Decision:` parser in the gateway.
- Memory-engine adapter: inactive draft validation only; it cannot persist,
  transition, or activate a decision.
- Contract: `n2n.room.v1` remains unchanged.

## Release-gate commands and results

### Memory engine

```text
cargo fmt --check
cargo test
```

Pass. Room isolation, provenance, context projection, extraction failure,
ingestion authentication, and inactive facilitator-port tests all pass.

### Gateway

```text
cargo fmt --check
cargo clippy -- -D warnings
DATABASE_URL=postgres://n2n:n2n@127.0.0.1:54329/n2n cargo test --quiet
```

Pass against the migrated PostgreSQL integration database. The suite covers
authorization, room flow, replay, facilitator behavior, mediated ingestion,
and human-only decision transitions.

### Contract and repository checks

```text
npm test                         # thought-khoral-contracts
bash scripts/verify-spec-hierarchy.sh
bash scripts/verify-repository-references.sh
bash scripts/verify-thoughtkhoral-identity.sh
bash scripts/test-verify-thoughtkhoral-identity.sh
```

Pass. Valid and invalid contract fixtures retain their expected outcomes, and
the identity/reference gates pass.

### Platform

```text
bash scripts/smoke-memory-engine.sh
bash scripts/validate-kube.sh
```

Pass. Compose exposes the memory engine as a healthy private service, and the
Kubernetes workload manifests were accepted by the local Podman validation.

## Boundary and failure evidence

1. Every derived fact, graph node, edge, supporting-context item, and draft
   candidate carries one room scope and source-event provenance.
2. Cross-room provenance and timestamp mismatches are rejected before graph
   writes or facilitator-port output.
3. Unauthenticated, malformed, unavailable, slow, or queue-saturated memory
   work cannot reject an already committed chat event.
4. The gateway remains the only writer of `decision.proposed` and the only
   caller of the human-only `decision.transition` path.
5. The memory engine receives no gateway `DATABASE_URL`; its Compose service
   has only its listen address and private shared secret.
6. Duplicate request handling and the existing deterministic parser remain
   covered by gateway integration tests.

## Rollback condition

If a later memory-provider experiment fails room/provenance, failure-isolation,
credential-separation, or human-activation checks, keep the deterministic
`Decision:` provider active and remove only the explicit Phase 4 provider
configuration. No contract migration or decision-ledger rollback is required.
