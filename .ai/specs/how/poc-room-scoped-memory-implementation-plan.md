# POC Room-Scoped Memory Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a proven, room-scoped shared-context capability in which human-approved active decisions are the conversation’s normative specifications and derived graph/retrieval memory supplies supporting context, without changing the governed-room MVP, the `n2n.room.v1` contract, or the human activation boundary.

**Status:** Approved — human approval received 2026-09-16; runtime and dependency work remains task-gated.

**Architecture:** `thought-khoral-memory-engine` runs as its own process and owns non-authoritative derived graph/provenance/context state partitioned by `roomId`; Cognee is never embedded in the room gateway. The gateway-owned active decision ledger contains normative room specifications. Mediated context projections combine active specifications with room-matched supporting context, while agent projections remain read-only. The gateway remains the sole mediator of committed room events and the sole owner of `decision.proposed` persistence and `decision.transition`. Work proceeds graph-only first, then through a mediated gateway-owned facilitator-port adapter; exactly one facilitator implementation is active at a time, with the deterministic `Decision:` parser remaining active until the explicit later switch task.

**Tech Stack:** Rust, the existing ThoughtKhoral room-event model, PostgreSQL/pgvector for memory-engine-owned derived storage where justified by the approved evidence, Cognee-RS only if its authoritative license/maturity/compatibility review passes, JSON fixtures, and the existing gateway/UI/platform test tooling. No dependency is pinned by this document.

**Spec:** `../what/poc-room-scoped-memory.md`; `poc-room-scoped-memory.md`; root `.ai/specs/decisions/005-room-scoped-poc-memory.md`; root `.ai/specs/what/n2n-solution-architecture.md`; `thought-khoral-room-gateway/.ai/specs/what/mvp-room.md`; `thought-khoral-room-gateway/.ai/specs/how/implementation.md`

## Global Constraints

- A room remains one conversation. Every derived fact, embedding, edge, lineage record, and draft candidate is partitioned by `roomId`.
- Project-level memory, topic-level memory, nested conversations, `conversationId`, and parent-project identifiers are out of scope.
- `n2n.room.v1`, its methods, fields, wire values, persisted event fields, and compatibility fixtures do not change.
- The gateway facilitator is the only draft-proposal port. No second independent `decision.propose` path, racing authenticated agent, or client-visible proposal method is added.
- Cognee runs in the `thought-khoral-memory-engine` process, never inside the gateway process and never as an unmediated model/tool call from the gateway.
- The deterministic gateway `Decision:` implementation remains the active facilitator until the explicit port-switch task in Phase 4 is completed and verified. Graph-only Cognee work may proceed before that switch.
- Humans continue to Confirm, Edit, or Dismiss. Only the gateway may apply those transitions; collective memory contains active decisions only.
- Active human-approved decisions are room-scoped normative specifications. Derived graph facts, embeddings, retrieval results, and extraction output are supporting context and never become authoritative without the existing human transition.
- Mediated human and agent context projections may include active specifications plus room-matched supporting context; agent projections are read-only and cannot activate or revise a specification.
- The first tasks must gather authoritative Cognee-RS license, maturity, and compatibility evidence. If the evidence is insufficient, the plan records that Cognee-RS cannot be adopted yet and leaves the crate unpinned.
- Extraction failure, timeout, malformed output, queue saturation, or memory-engine unavailability must not block chat, room join, the live `Decision:` facilitator, or human decision transitions.
- The memory engine receives no gateway event-log database credentials, gateway database connection string, filesystem shell access, arbitrary command execution, or unmediated side-effecting tools.
- The memory engine may use separately owned storage credentials for derived memory only if a later task proves that need and keeps the event-log database boundary explicit.
- No memory-engine service is added to the platform stack until the mediated-ingestion phase requires an executable cross-process test.
- Runtime code, Cargo dependency changes, binaries, migrations, deployment assets, and generated artifacts are not authorized until the relevant task is approved under the repository workflow.
- Commit commands below are future execution checkpoints. Do not run them during this documentation pass or without explicit user authorization to commit.

## Planned repository structure

The files below are future task outputs; this planning pass creates none of them.

```text
thought-khoral-memory-engine/
  .ai/specs/how/poc-room-scoped-memory-implementation-plan.md
  docs/dependency-evidence/cognee-rs.md
  src/{config,events,graph,provenance,context,extractor,cognee,facilitator,ingestion}.rs
  tests/{room_isolation,provenance,context_projection,extraction_failure,ingestion}_test.rs
  fixtures/room-scoped/{room-a,room-b,malformed}.json
  Cargo.toml                         # Phase 2 scaffold; no Cognee dependency
  Cargo.lock                         # generated for the backend-neutral scaffold

thought-khoral-room-gateway/
  src/facilitator.rs                 # existing port; adapter is added behind it
  src/memory_engine_client.rs        # private mediated forwarding only
  tests/{facilitator,memory_engine_integration}_test.rs

thought-khoral-platform/
  compose.yaml                        # modified only in Phase 3
  kube/memory-engine.yaml             # added only if the chosen test requires it
  scripts/smoke-memory-engine.sh
```

The exact Cognee adapter module and storage migration names remain subject to
the evidence gate. They must not be invented by pinning a crate before the
authoritative review is complete.

## Interfaces and invariants used across tasks

These are internal implementation interfaces, not changes to
`n2n.room.v1`. The gateway continues to use its existing facilitator port and
existing event persistence path.

```rust
pub struct CommittedRoomEvent {
    pub event_id: Uuid,
    pub room_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

pub struct DraftProposal {
    pub room_id: Uuid,
    pub title: String,
    pub summary: String,
    pub source_event_ids: Vec<Uuid>,
}

pub enum DecisionSpecStatus { Active, Superseded, Dismissed }

pub struct Provenance {
    pub source_event_ids: Vec<Uuid>,
    pub source_timestamps: Vec<DateTime<Utc>>,
}

pub struct DecisionSpec {
    pub room_id: Uuid,
    pub decision_id: Uuid,
    pub status: DecisionSpecStatus,
    pub title: String,
    pub summary: String,
    pub provenance: Provenance,
}

pub struct SupportingContextItem {
    pub room_id: Uuid,
    pub content: serde_json::Value,
    pub provenance: Provenance,
}

pub struct RoomContextSnapshot {
    pub room_id: Uuid,
    pub active_specs: Vec<DecisionSpec>,
    pub supporting_context: Vec<SupportingContextItem>,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub trait RoomMemoryPort {
    fn ingest(&self, event: CommittedRoomEvent) -> Result<IngestReceipt, MemoryError>;
    fn propose(&self, event: CommittedRoomEvent) -> Result<Vec<DraftProposal>, MemoryError>;
}
```

`RoomMemoryPort` is an internal boundary. The memory engine never calls
`decision.transition`; the gateway validates returned `DraftProposal` values,
adds the gateway-owned actor/provenance fields, and records any proposal as
the existing `decision.proposed` event. During graph-only phases, only
`ingest` is enabled. During the final phase, one selected implementation owns
`propose`; the inactive implementation cannot also emit drafts.

Every stored graph object must have a non-null `roomId` partition key and
source-event provenance. A query without a room scope must be rejected before
it reaches a graph or vector backend. A proposal with an empty room ID,
unknown source event, cross-room source event, malformed title/summary, or
invalid provenance is rejected and cannot affect active context.
A context snapshot must keep `active_specs` separate from
`supporting_context`; only the former is normative. An agent may read the
snapshot but never mutate either layer.

---

## Phase 1: Evidence and dependency pin

### Task 1: Produce the Cognee-RS adoption evidence dossier

**Files:**
- Create: `thought-khoral-memory-engine/docs/dependency-evidence/cognee-rs.md`
- Reference only: `thought-khoral-memory-engine/.ai/specs/how/poc-room-scoped-memory.md`, root solution architecture, and this plan
- Test: a reviewer evidence checklist plus reproducible metadata commands recorded in the dossier

**Interfaces:**
- Consumes: authoritative upstream repository metadata, release/tag history, crate/package metadata, API documentation, license/notice files, CI/toolchain declarations, and official integration documentation.
- Produces: an explicit `Adopt`, `Adopt with constraints`, or `Cannot adopt yet` result; a candidate version is not a pin until the result is `Adopt` or `Adopt with constraints`.

- [x] **Step 1: Gather authoritative identity and license evidence**

Record the upstream repository URL and reviewed commit/tag, the package registry entry, the exact license expression, `LICENSE`/`NOTICE` contents, copyright obligations, and the complete resolved transitive license graph for the candidate. Treat registry metadata as corroboration; use upstream source and package metadata as the primary record.

- [x] **Step 2: Gather maturity evidence**

Record release history, latest stable/release status, maintenance activity, CI coverage, supported Rust toolchains and targets, public examples/tests, API stability signals, known open blockers, and whether the graph/extraction APIs needed by this POC are usable without an unreviewed model or network side effect.

- [x] **Step 3: Gather compatibility evidence**

Check compatibility with the repository’s validated Rust toolchain, PostgreSQL/pgvector direction, the process boundary, room-scoped identifiers, deterministic testability, offline/error behavior, and the future private mediated-ingestion seam. Record any required feature flags, native libraries, network services, model services, or storage assumptions.

- [x] **Step 4: Apply the adoption gate before editing Cargo files**

Use this decision table in the dossier:

| Evidence result | Allowed next action |
| --- | --- |
| License, maturity, compatibility, and transitive-license evidence pass | Record the exact candidate package/version and permitted feature set; Task 2 may add the dependency only after plan approval. |
| Evidence passes only with operational limits | Record each limit and required test; Task 2 may proceed only with those limits encoded as constraints. |
| Any required evidence is unavailable, incompatible, or unacceptable | Record `Cannot adopt yet`; leave `Cargo.toml` and `Cargo.lock` without Cognee-RS and continue only with a backend-neutral graph/provenance design. |

- [x] **Step 5: Verify the dossier is reproducible**

Run the exact source and metadata commands recorded in the dossier. Expected: every claim has a dated authoritative source, every URL resolves to the reviewed artifact, and the conclusion names the blocked phase if adoption is denied.

- [ ] **Step 6: Future commit checkpoint**

After human approval of the evidence result, commit only the dossier in the memory-engine repository with a message such as:

```bash
git add docs/dependency-evidence/cognee-rs.md
git commit -m "docs: record Cognee-RS adoption evidence"
```

Do not execute this checkpoint in the current pass.

### Task 2: Pin the dependency only if Task 1 passes

**Files:**
- Modify, only when permitted: `thought-khoral-memory-engine/Cargo.toml`
- Create or modify, only when permitted: `thought-khoral-memory-engine/Cargo.lock`
- Modify: `thought-khoral-memory-engine/docs/dependency-evidence/cognee-rs.md`
- Test: locked metadata, transitive-license audit, and a minimal compile/import probe

**Interfaces:**
- Consumes: the exact adoption result and constraints from Task 1.
- Produces: one reproducible candidate dependency selection, or a documented backend-neutral mode when Task 1 records `Cannot adopt yet`.

- [ ] **Step 1: Write the dependency-selection test commands before changing manifests**

The evidence dossier must specify the exact locked checks, including `cargo metadata --locked`, `cargo tree --locked`, the selected feature set, and the project’s license-audit command. The checks must fail if the resolved graph differs from the reviewed graph or introduces an unapproved license/platform requirement.

- [ ] **Step 2: Add only the reviewed package and features**

If adoption passed, add the exact reviewed version and no default or transitive feature that was not covered by the evidence. If adoption did not pass, do not add a Cognee package; define only backend-neutral interfaces needed by later graph tests.

- [ ] **Step 3: Run the locked dependency checks**

Run the exact commands from the dossier. Expected: the lockfile is reproducible, the full normal dependency graph matches the reviewed evidence, and the minimal probe compiles without starting a service or contacting a gateway database.

- [ ] **Step 4: Future commit checkpoint**

For an approved adoption, commit the manifest, lockfile, and updated evidence together. For `Cannot adopt yet`, commit only the evidence and backend-neutral manifest changes if separately approved. Do not commit now.

---

## Phase 2: Room-scoped graph, provenance, and shared context, without a live port swap

### Task 3: Define the backend-neutral room graph, provenance, and context model

**Files:**
- Create: `thought-khoral-memory-engine/src/events.rs`
- Create: `thought-khoral-memory-engine/src/graph.rs`
- Create: `thought-khoral-memory-engine/src/provenance.rs`
- Create: `thought-khoral-memory-engine/src/context.rs`
- Create: `thought-khoral-memory-engine/fixtures/room-scoped/{room-a,room-b,malformed}.json`
- Test: `thought-khoral-memory-engine/tests/room_isolation_test.rs`
- Test: `thought-khoral-memory-engine/tests/provenance_test.rs`
- Test: `thought-khoral-memory-engine/tests/context_projection_test.rs`

**Interfaces:**
- Consumes: committed gateway events normalized into the internal `CommittedRoomEvent` shape.
- Produces: room-scoped `DerivedFact`, `GraphNode`, `GraphEdge`, provenance APIs, and a context projection that separates authoritative active decision specifications from supporting derived context; no gateway method or schema changes.

The Phase 2 scaffold may use only the existing gateway’s foundational Rust
crates for time, serialization, JSON values, and UUIDs. It must not add
Cognee, graph-database, vector-database, model, or service dependencies. This
scaffold does not change the Task 1 Cognee adoption result.

- [x] **Step 1: Write failing room-isolation and provenance tests**

Use two rooms and overlapping source-event UUIDs only where the fixture deliberately tests rejection. The tests must prove that:

```rust
assert_eq!(graph.facts_for_room(room_a).len(), 1);
assert!(graph.facts_for_room(room_a).iter().all(|fact| fact.room_id == room_a));
assert!(graph.query(room_a, "deployment").into_iter().all(|item| item.room_id == room_a));
assert!(graph.insert_edge(edge_from_room_a_to_room_b).is_err());
```

Also assert every accepted fact and edge retains at least one source event ID and its source timestamp, and that a source event from another room is rejected before persistence.
Add context-projection cases: an active decision specification appears in
`active_specs`, a derived fact appears only in `supporting_context`, superseded
or dismissed decisions are absent from `active_specs`, and a supporting item
cannot change decision status.

- [x] **Step 2: Run the focused tests and confirm the expected failure**

Run: `cargo test --test room_isolation_test --test provenance_test --test context_projection_test`

Expected: FAIL because the graph/provenance types and room guards do not yet exist.

- [x] **Step 3: Implement the minimal room-scoped model and context projection**

Define the internal types with required `room_id` and provenance fields. Make room-scoped access explicit in function signatures; do not expose `all_facts()` or an unscoped vector/graph query. Validate that every source event and every derived object shares the requested room before accepting it. Build `RoomContextSnapshot` from active gateway specifications plus room-matched supporting context, preserve provenance and expiry, exclude superseded/dismissed specifications, and never promote supporting context into a decision specification.

- [x] **Step 4: Run focused tests and a negative-contract scan**

Run: `cargo fmt --check && cargo test --test room_isolation_test --test provenance_test --test context_projection_test`

Expected: PASS, with no new `conversationId`, project identifier, or `n2n.room.v1` method/field in the source or fixtures.

- [x] **Step 5: Future commit checkpoint**

Commit the backend-neutral graph/provenance model and fixtures only after human approval of the task. Do not execute now.

### Task 4: Add graph-only extraction and deterministic supporting-context behavior

**Files:**
- Create: `thought-khoral-memory-engine/src/extractor.rs`
- Create, only if Task 1 permits adoption: `thought-khoral-memory-engine/src/cognee.rs`
- Create: `thought-khoral-memory-engine/tests/extraction_failure_test.rs`
- Modify: `thought-khoral-memory-engine/tests/provenance_test.rs`
- Test: fixture-driven extraction, provenance, room partitioning, and failure behavior

**Interfaces:**
- Consumes: `CommittedRoomEvent`, `RoomScope`, and the room graph repository from Task 3.
- Produces: non-authoritative graph facts/edges and supporting-context items only; `propose` remains disabled and no `decision.proposed` or transition event is emitted.

- [x] **Step 1: Write failing graph-only extraction tests**

Tests must feed a committed message fixture for room A, verify graph facts and edges cite that event and room, feed room B, and verify no room A query returns room B data. Add malformed extraction output, missing provenance, cross-room source IDs, backend timeout, and backend error cases; each must return a structured error or dropped result without an active decision write.

- [x] **Step 2: Run the focused tests**

Run: `cargo test --test extraction_failure_test --test provenance_test --test room_isolation_test`

Expected: FAIL until the extractor and backend adapter exist.

- [x] **Step 3: Implement graph-only extraction**

Create a backend-neutral extractor boundary. If Cognee-RS was approved, implement its adapter only inside the memory-engine process and pass an explicit `RoomScope` plus source-event provenance on every call. If it was not approved, use a deterministic fixture backend and mark the runtime provider unavailable; do not add a substitute package or simulate live Cognee adoption.

The graph-only path may persist derived facts, lineage, and supporting context,
but it must never treat those outputs as decision specifications, return drafts,
call `decision.transition`, write gateway active context, or use gateway
event-log credentials.

- [x] **Step 4: Run failure-isolation and room-partition tests**

Run: `cargo fmt --check && cargo test --test extraction_failure_test --test provenance_test --test room_isolation_test`

Expected: PASS; extraction failures are observable as structured memory errors while the graph remains partitioned and no active decision is created.

- [x] **Step 5: Prove the live facilitator remains unchanged**

Run the existing gateway facilitator regression suite from the gateway repository:

```bash
cd ../thought-khoral-room-gateway
cargo test --test facilitator_test
```

Expected: `Decision:` still produces its deterministic draft from a persisted `message.created` event, ordinary messages produce no draft, and no memory-engine process is required for the test.

Execution note: the complete facilitator regression passes when run with the
migrated PostgreSQL integration database; no memory-engine process is required.

- [ ] **Step 6: Future commit checkpoint**

Commit graph-only extraction and its fixtures separately from any later port switch. Do not execute now.

---

## Phase 3: Mediated event ingestion

### Task 5: Select and document the mediated ingestion path

**Files:**
- Create: `thought-khoral-memory-engine/docs/mediated-ingestion.md`
- Modify: `thought-khoral-memory-engine/.ai/specs/how/poc-room-scoped-memory-implementation-plan.md` only if the approved implementation reveals a necessary clarification
- Test: architecture review plus an interface/sequence test plan recorded in the ingestion document

**Interfaces:**
- Consumes: the How-approved option of gateway-mediated forwarding of committed events to the facilitator-port implementation.
- Produces: one selected private transport and an explicit rejection of racing authenticated-agent ingestion and direct gateway database access.

- [x] **Step 1: Compare only permitted mediated variants**

Evaluate a gateway-owned adapter that forwards committed events to a separately running memory-engine process, with bounded timeout, authentication, and queue behavior. Do not evaluate a client-visible `decision.propose` call, an authenticated agent that races the gateway, or a memory-engine connection to the gateway event-log database.

- [x] **Step 2: Select the concrete private transport**

Choose the simplest transport that the evidence supports and that keeps the gateway as the only facilitator owner. The selected interface must carry `roomId`, event ID, event type, timestamp, and the normalized event payload; it must return either an ingestion receipt or a typed failure. It must not add a method or field to `n2n.room.v1`.

- [x] **Step 3: Specify the non-blocking sequence**

Document and test this sequence:

```text
gateway validates and commits room event
  -> gateway publishes the committed room event normally
  -> gateway enqueues bounded mediated memory work
  -> memory engine validates room/provenance and derives graph data
  -> gateway records no active context and no second proposal path
```

The event-log transaction and chat response must not await successful extraction. Queue saturation, timeout, or memory-engine failure is recorded through safe identifiers/error codes and does not reject the committed chat event.

- [ ] **Step 4: Future commit checkpoint**

Commit the selected transport record only after human approval. Do not execute now.

### Task 6: Implement mediated ingestion and compose the memory engine only for this phase

**Files:**
- Create: `thought-khoral-memory-engine/src/ingestion.rs`
- Create: `thought-khoral-memory-engine/tests/ingestion_test.rs`
- Create: `thought-khoral-room-gateway/src/memory_engine_client.rs`
- Modify: `thought-khoral-room-gateway/src/rooms.rs` or the existing post-commit dispatch module
- Create: `thought-khoral-room-gateway/tests/memory_engine_integration_test.rs`
- Modify only in this phase: `thought-khoral-platform/compose.yaml`
- Create only if needed by the selected transport: `thought-khoral-platform/kube/memory-engine.yaml`, `thought-khoral-platform/scripts/smoke-memory-engine.sh`
- Test: mediated event delivery, timeout/failure isolation, and credential-boundary checks

**Interfaces:**
- Consumes: the selected private ingestion interface from Task 5 and committed events after gateway persistence.
- Produces: graph ingestion receipts/errors; no client-visible RPC method, no direct event-log database connection, and no active decision mutation.

- [x] **Step 1: Write failing cross-process tests**

The integration tests must prove that a committed event reaches the memory engine only after persistence, carries one `roomId`, and can be acknowledged without changing the room’s active decisions. They must also simulate an unavailable/slow/malformed memory engine and assert the gateway still completes chat, preserves the live `Decision:` behavior, and permits human transitions.

- [x] **Step 2: Run the tests before adding composition**

Run: `cargo test --test memory_engine_integration_test` in the gateway and `cargo test --test ingestion_test` in the memory engine.

Expected: FAIL because the private client, ingestion endpoint/adapter, and test process are absent.

- [x] **Step 3: Implement the bounded mediated path**

Add a gateway-owned post-commit dispatcher that forwards only normalized committed events. Use a bounded queue and timeout. Keep proposal persistence in the existing gateway facilitator/persistence path. Configure separate memory-engine storage credentials if required; never pass `DATABASE_URL` or gateway event-log credentials to the memory engine.

The memory engine must reject missing/cross-room provenance and return typed failures. Its logs may contain safe room/event identifiers and error codes, but not access tokens, raw room text, titles, or summaries.

- [x] **Step 4: Add phase-gated local composition**

Add the memory-engine service to Compose or Kubernetes manifests only now, because this phase has an executable mediated-ingestion test that requires it. Use an explicit development-only memory store/configuration, no gateway event-log credentials, and no production topology claims. Do not add the service in earlier platform tasks.

- [x] **Step 5: Run the phase tests and verify the boundary**

Run:

```bash
cargo fmt --check
cargo test --test ingestion_test
cd ../thought-khoral-room-gateway
cargo fmt --check
cargo test --test memory_engine_integration_test --test facilitator_test
cd ../thought-khoral-platform
bash scripts/smoke-memory-engine.sh
```

Expected: PASS; chat and `Decision:` remain available when memory extraction fails, and the memory engine has no gateway event-log database credential.

- [ ] **Step 6: Future commit checkpoint**

Commit memory-engine ingestion, gateway mediation, and platform composition in independently reviewable repository commits only after human approval. Do not execute now.

---

## Phase 4: Optional later facilitator-port occupation

### Task 7: Add the memory-engine facilitator adapter behind the existing port

**Files:**
- Create: `thought-khoral-memory-engine/src/facilitator.rs`
- Create: `thought-khoral-memory-engine/tests/facilitator_port_test.rs`
- Modify: `thought-khoral-room-gateway/src/facilitator.rs`
- Create or modify: `thought-khoral-room-gateway/src/memory_engine_client.rs`
- Create: `thought-khoral-room-gateway/tests/facilitator_port_test.rs`
- Test: port contract, proposal validation, provenance, and no-transition behavior

**Interfaces:**
- Consumes: graph/provenance output and the private mediated event path from Phases 2–3.
- Produces: `Vec<DraftProposal>` through the existing gateway facilitator port only.

- [x] **Step 1: Write failing port tests**

Tests must assert that a memory-derived candidate returns a draft carrying the same `roomId` and source event IDs, that a candidate with cross-room provenance is rejected, that no candidate can call `decision.transition`, and that the gateway records any accepted result as `decision.proposed` with draft status.

Include a structural test or source-level assertion that the memory adapter does not expose a client-visible `decision.propose` handler and that the gateway has one facilitator dispatch point.

- [x] **Step 2: Run the focused tests**

Run: `cargo test --test facilitator_port_test`

Expected: FAIL until the memory-engine adapter and gateway-side validation are present.

- [x] **Step 3: Implement the adapter without activating it yet**

Implement `RoomMemoryPort::propose` behind the existing facilitator boundary. It may return zero or more validated draft candidates, but it must not persist gateway events, transition decisions, or write active context. Keep the deterministic `Decision:` implementation as the configured active provider while this adapter is tested.

- [x] **Step 4: Run focused tests and the retained live-parser regression**

Run:

```bash
cargo test --test facilitator_port_test
cd ../thought-khoral-room-gateway
cargo test --test facilitator_test --test facilitator_port_test
```

Expected: PASS; the live parser still produces drafts, and the memory adapter is proven to use the same port without being a second active proposer.

- [x] **Step 5: Future commit checkpoint**

Commit the inactive adapter and validation tests only after human approval.
Executed after approval in `thought-khoral-memory-engine` commit `0585b21` and
`thought-khoral-room-gateway` commit `233d535`.

### Task 8: Explicitly switch the active facilitator implementation

**Files:**
- Modify: `thought-khoral-room-gateway/src/facilitator.rs` and its provider/configuration module
- Modify: `thought-khoral-room-gateway/tests/facilitator_test.rs`
- Modify: `thought-khoral-room-gateway/tests/facilitator_port_test.rs`
- Modify: `thought-khoral-memory-engine/tests/facilitator_port_test.rs`
- Modify: `thought-khoral-platform/compose.yaml` and any phase-gated smoke script
- Test: one-provider-only end-to-end governed-decision flow

**Interfaces:**
- Consumes: the proven memory-engine adapter, mediated ingestion, graph/provenance fixtures, and existing gateway transition API.
- Produces: an explicit active-provider selection in which the memory engine occupies the facilitator port; `Decision:` remains covered as a regression implementation but is not concurrently active.

- [ ] **Step 1: Define the switch acceptance test before changing the active provider**

The test must fail unless:

```text
one committed room event -> exactly one configured facilitator provider
memory proposal -> gateway decision.proposed draft
human Confirm/Edit/Dismiss -> existing decision.transition path
active collective memory -> only gateway-active decisions
```

It must also assert that an ordinary chat event, extraction failure, malformed memory proposal, and duplicate delivery cannot create an unauthorized active decision.

- [ ] **Step 2: Run the baseline provider tests**

Run the existing gateway facilitator and room-flow tests with the deterministic provider. Expected: PASS before the provider switch is attempted.

- [ ] **Step 3: Switch the provider explicitly**

Change the gateway’s provider selection so exactly one provider is active for a given process. The memory-engine provider becomes active only in the explicitly configured Phase 4 test profile after all prior evidence and tests pass. Do not leave both providers subscribed to committed events or emitting drafts.

Retain the `Decision:` parser implementation and regression tests as an inactive/fallback implementation; do not run it concurrently with the memory provider in the same room flow.

- [ ] **Step 4: Run the switch and governance tests**

Run:

```bash
cd ../thought-khoral-room-gateway
cargo fmt --check
cargo test --test facilitator_test --test facilitator_port_test --test room_flow_test --test authorization_test
cd ../thought-khoral-memory-engine
cargo test --test facilitator_port_test --test room_isolation_test --test provenance_test --test extraction_failure_test
```

Expected: PASS; memory-derived drafts enter through the existing port, only humans transition them, and the retained parser remains a tested non-concurrent implementation.

- [ ] **Step 5: Future commit checkpoint**

Commit the explicit provider switch only after a human reviews the evidence, integration output, and rollback behavior. Do not execute now.

### Task 9: Run the complete POC verification and document the deferred boundary

**Files:**
- Create: `thought-khoral-memory-engine/docs/poc-verification.md`
- Modify: `thought-khoral-memory-engine/.ai/specs/README.md` only if additional approved local specs were created
- Modify: `thought-khoral-platform/scripts/smoke-memory-engine.sh`
- Test: complete memory-engine, gateway, contract, and platform verification suites

**Interfaces:**
- Consumes: all phase outputs and the unchanged released `n2n.room.v1` contract.
- Produces: reproducible evidence for room isolation, provenance, failure isolation, one facilitator path, human-only activation, and credential separation.

- [x] **Step 1: Add the release-gate assertions**

The gate must prove all of the following:

1. Two rooms never return each other’s derived facts, embeddings, lineage, or drafts.
2. Every derived item and draft has source-event provenance and the correct `roomId`.
3. No `conversationId`, project identifier, new JSON-RPC method, new contract version, or persisted event field was introduced.
4. The gateway remains the only writer of `decision.proposed` and active-context transitions.
5. No second independent `decision.propose` path or racing agent exists.
6. Extraction failure does not block chat, the selected facilitator’s draft path, or human transitions.
7. Collective memory contains only active gateway decisions.
8. Active decision specifications are separated from supporting context, and only active specifications are normative.
9. Human and agent context projections are room-scoped, and agent projections are read-only.
10. The memory engine has no gateway event-log database credentials.
11. The live `Decision:` parser remains a passing regression implementation, even after the explicit provider switch.

- [x] **Step 2: Run all focused and integration tests**

Run the exact commands established by the repository toolchains, including:

```bash
cd thought-khoral-memory-engine
cargo fmt --check
cargo test
cd ../thought-khoral-room-gateway
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cd ../thought-khoral-contracts
npm test
cd ../thought-khoral-platform
bash scripts/smoke-memory-engine.sh
bash scripts/validate-kube.sh
```

Expected: PASS without changing the contract repository or requiring gateway database credentials in the memory-engine environment.

- [x] **Step 3: Record evidence and rollback conditions**

In `docs/poc-verification.md`, record exact dependency evidence, source revisions, test commands, room fixtures, provider selection, failure-injection results, credential inspection, and the condition that reverts the active provider to deterministic `Decision:` without changing `n2n.room.v1`.

Execution note: the release gate passes with the deterministic provider retained;
the optional memory-provider switch is deferred because Cognee adoption and a
proposal-generating runtime provider are not yet proven.

- [x] **Step 4: Future commit checkpoint**

Commit verification documentation independently in each affected repository only after human approval.
Executed in the Task 9 verification checkpoint; the deterministic facilitator
remains active and the Phase 4 switch is deferred.

## Spec coverage review

| Approved requirement | Plan coverage |
| --- | --- |
| Room remains one conversation; all derived memory is room-scoped | Global Constraints; Tasks 3, 4, 7, and 9 |
| Project/topic hierarchy and conversation identifiers are excluded | Global Constraints; Tasks 3 and 9 |
| Cognee lives in the memory-engine process, not the gateway | Architecture; Tasks 1, 2, 4, and 6 |
| Facilitator is the only draft-proposal port | Interfaces; Tasks 5, 7, 8, and 9 |
| Live `Decision:` stays until an explicit later switch | Global Constraints; Tasks 4, 7, and 8 |
| Humans Confirm/Edit/Dismiss; active-only collective memory | Tasks 7, 8, and 9 |
| Active decisions are normative specs; derived context is supporting | Global Constraints; Tasks 3, 4, 7, 8, and 9 |
| License/maturity/compatibility evidence precedes pinning | Phase 1, Tasks 1–2 |
| Extraction failure cannot block governed chat or transitions | Tasks 4, 5, 6, 8, and 9 |
| No gateway event-log database credentials in memory engine | Global Constraints; Tasks 6 and 9 |
| No platform composition before a phase requires it | Global Constraints; Task 6 |
| No contract change | Global Constraints; Tasks 3, 5, and 9 |

## Deliberate non-goals and follow-ons

This plan does not define project/topic memory, nested conversations, a new
wire contract, UI product changes, remote A2A/MCP agents, model hosting,
production OpenShift topology, or a generalized memory platform. If the
facilitator switch is not justified by the evidence or integration tests, the
approved outcome is a graph-only memory-engine capability with the
deterministic gateway `Decision:` facilitator retained; a later plan must
explicitly revisit the port with a new approved decision or How update if the
boundary changes.
