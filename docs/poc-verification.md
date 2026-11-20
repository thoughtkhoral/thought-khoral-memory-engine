# Room-scoped memory POC verification

## Current workspace context — 2026-11-20

This component remains the incubating in-memory room/provenance and private mediated-ingestion proof at runtime baseline `7b46518`. The published contract and prepared Codex consumers/profile do not enable memory retrieval/writes, an active facilitator, Cognee or durable storage. Existing fixture/receipt/error-handling proof limits and human activation authority remain as specified. No new memory runtime test or implementation is claimed by this documentation refresh.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

<a id="current-scope--documentation-reconciliation-2026-11-19"></a>

## Historical scope — documentation reconciliation 2026-11-19

This record preserves the earlier POC execution evidence below. It has not been
rerun during this documentation pass. Its references to a live `Decision:`
parser and rollback to that parser describe the former baseline, superseded by
root [Decision 008](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/008-slash-decisions-and-facilitator-boundary.md).
The parser is retired; no automatic prefix or memory-derived proposer is active.
Future facilitator activation requires a new approved plan against that boundary.

The approved local [What](../.ai/specs/what/poc-room-scoped-memory.md),
[How](../.ai/specs/how/poc-room-scoped-memory.md), and
[task-gated plan](../.ai/specs/how/poc-room-scoped-memory-implementation-plan.md)
authorize the incubating room-scoped graph and private ingestion proof. Local
source inspection confirms the in-memory graph, deterministic fixture extractor,
authenticated ingestion process, and inactive draft validation adapter. Cognee
adoption, durable storage, and production deployment remain deferred. This
source inspection establishes implementation scope, not fresh runtime evidence.
Codex conversation milestone acceptance is tracked separately in the
[shared status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md).

## Historical POC outcome

The graph-only memory capability and mediated ingestion boundary are verified.
The deterministic gateway `Decision:` facilitator remains the active provider.
The optional Phase 4 provider switch is deferred because Cognee-RS adoption is
still recorded as `Cannot adopt yet`, and the memory engine has no proven
proposal-generating implementation beyond the validated draft port adapter.

## Historical evidence and revisions

Original verification record: memory-engine commit
[`860389c`](https://github.com/thoughtkhoral/thought-khoral-memory-engine/commit/860389c)
(2026-09-18), before the 2026-09-24 parser supersession. Current reconciliation
was based on memory-engine HEAD `037526f` plus the approved local specifications.

- Cognee evidence: `docs/dependency-evidence/cognee-rs.md`; result:
  `Cannot adopt yet`; no Cognee dependency is present in `Cargo.toml`.
- Mediated transport: `docs/mediated-ingestion.md`; private authenticated
  HTTP ingestion, bounded gateway queue, post-commit dispatch.
- Active facilitator: deterministic `Decision:` parser in the gateway.
- Memory-engine adapter: inactive draft validation only; it cannot persist,
  transition, or activate a decision.
- Contract: `n2n.room.v1` remains unchanged.

## Historical release-gate commands and results

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

## Historical boundary and failure evidence

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

## Historical rollback condition — superseded parser baseline

If a later memory-provider experiment fails room/provenance, failure-isolation,
credential-separation, or human-activation checks, keep the deterministic
`Decision:` provider active and remove only the explicit Phase 4 provider
configuration. No contract migration or decision-ledger rollback is required.

The historical commands, including the database URL, record the original test
environment. They are not current deployment instructions or evidence that the
present gateway, platform, or memory HEAD passed those suites.
