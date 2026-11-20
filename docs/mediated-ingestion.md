# Mediated committed-event ingestion

## Current workspace context — 2026-11-20

This component remains the incubating in-memory room/provenance and private mediated-ingestion proof at runtime baseline `7b46518`. The published contract and prepared Codex consumers/profile do not enable memory retrieval/writes, an active facilitator, Cognee or durable storage. Existing fixture/receipt/error-handling proof limits and human activation authority remain as specified. No new memory runtime test or implementation is claimed by this documentation refresh.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

## Status and sources

Maintained description of the incubating Phase 3 proof, derived from the approved
[What](../.ai/specs/what/poc-room-scoped-memory.md),
[How](../.ai/specs/how/poc-room-scoped-memory.md), and
[implementation plan](../.ai/specs/how/poc-room-scoped-memory-implementation-plan.md).
Root [Decision 008](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/008-slash-decisions-and-facilitator-boundary.md)
supersedes the plan's former live-parser baseline: `Decision:` text is ordinary
chat, and no automatic prefix or memory-derived proposer is active. The inactive
draft adapter does not activate a provider. The earlier execution evidence is
preserved in the [historical POC verification record](poc-verification.md).

## Decision

The memory engine receives committed room events through a private HTTP
endpoint owned by the memory-engine process. The gateway uses a
gateway-owned post-commit dispatcher to send a JSON `CommittedRoomEvent`
envelope to `POST /internal/v1/ingest` over the private service network.

This is an internal deployment interface. It is not part of `n2n.room.v1`, is
not exposed through the browser WebSocket, and does not create a client-visible
`decision.propose` method. The gateway remains the only owner of the
facilitator port, proposal persistence, and decision transitions.

The initial implementation uses the existing Rust HTTP/Tokio direction already
used by the gateway, with a bounded in-process queue and a per-deployment
shared authentication secret supplied through process configuration. The
secret is sent only in the private request's `Authorization` header; it is
never included in event payloads, logs, URLs, or the `n2n.room.v1` protocol.
TLS termination or mTLS may be added at the private service boundary later,
but is not required to define this POC interface. The memory engine must still
reject requests that are unauthenticated, malformed, or missing a room scope.

## Permitted options and rejected alternatives

| Option | Decision | Reason |
| --- | --- | --- |
| Gateway-owned private HTTP forwarding after commit | **Selected** | Fits the existing Axum/Tokio service direction, is straightforward to exercise with an in-process test server, and gives the gateway an explicit timeout, authentication, and queue boundary. |
| Gateway-owned private transport using a different internal RPC protocol | Deferred | It would add another protocol and dependency surface without evidence that it improves this POC's correctness or failure isolation. Reconsider only if the HTTP boundary cannot meet measured requirements. |
| Client-visible `decision.propose` request | Rejected | Bypasses the gateway's facilitator ownership and would create a second proposal path. |
| Authenticated agent racing the gateway facilitator | Rejected | Creates competing proposal authors and makes deterministic facilitator ownership ambiguous. |
| Memory engine reading the gateway event-log database | Rejected | Violates the process and credential boundary, couples the engine to gateway persistence, and prevents an independently testable ingestion failure path. |

## Designed internal envelope

The request body contains exactly one normalized committed event:

```json
{
  "roomId": "<UUID>",
  "eventId": "<UUID>",
  "eventType": "room.message",
  "occurredAt": "<RFC3339 timestamp>",
  "payload": {}
}
```

The implementation uses Rust field names internally and camelCase fields at
the HTTP boundary. The approved design requires:

- `roomId` is present and is the sole partition scope for the operation;
- `eventId`, `eventType`, `occurredAt`, and `payload` are present and valid;
- derived objects retain the event ID and timestamp as provenance;
- a duplicate event is handled idempotently rather than producing duplicate
  graph writes; and
- no request can provide a second room scope or source an event from another
  room.

The success response is:

```json
{"status":"accepted","roomId":"<UUID>","eventId":"<UUID>"}
```

The current handler returns `unauthorized`, `scope_mismatch`,
`malformed_request`, or `graph_rejected` for service-level errors. Axum rejects
invalid JSON, missing required fields, or oversized bodies before the handler;
those framework responses do not use the same JSON error envelope. Safe error
codes are the design boundary; raw room text, titles, summaries, access tokens,
and database credentials must not be logged.

## Current proof limits

These limits derive from the owning plan's
[current implementation checkpoint](../.ai/specs/how/poc-room-scoped-memory-implementation-plan.md#known-implementation-limits--unclosed-proof-requirements).
They remain unclosed proof requirements; the checkpoint does not waive the
approved design or authorize runtime changes.

The running proof uses `DeterministicFixtureExtractor`: `payload.extraction`
contains fixture output. The empty `payload` above illustrates envelope shape,
not a successful extraction request. Ordinary committed chat without fixture
output returns an extraction failure; this is not live semantic extraction.
The graph is in memory and disappears on process restart.

Duplicate event IDs are rejected before applying graph writes, rather than
acknowledged as an earlier successful ingestion. The rejection currently maps
through the extraction error to `malformed_request`; there is no dedicated
`duplicate_event` HTTP response. The gateway worker makes one bounded send
attempt and checks HTTP success status; it does not validate the receipt body
or implement retries. These observations describe the existing proof and do
not close the design's broader response/replay evidence requirements.

Sources for this scope are [ingestion](../src/ingestion.rs),
[extractor](../src/extractor.rs), [graph](../src/graph.rs), and the gateway-owned
[dispatcher](https://github.com/thoughtkhoral/thought-khoral-room-gateway/blob/main/src/memory_engine_client.rs).
No runtime checks were rerun for this documentation reconciliation.

## Non-blocking sequence

The gateway's authoritative request path is ordered as follows:

```text
gateway validates and commits room event
  -> gateway publishes the committed event normally
  -> gateway attempts to enqueue one bounded memory-ingestion job
  -> worker sends the private authenticated HTTP request with a short timeout
  -> memory engine validates room/provenance and derives graph data
  -> gateway records no active context and no second proposal path
```

The room-event transaction and the chat response do not await extraction or a
successful memory-engine response. A full queue, connection failure, timeout,
HTTP error, malformed response, or memory-engine outage must not reject the
already committed chat event. The design requires safe diagnostics; the current
worker logs elapsed timeouts and drops failed jobs without retry. Human
`/decisions` actions and decision transitions remain governed by the gateway;
the retired `Decision:` parser is not a fallback.

The dispatcher must be shut down with the gateway and must not outlive the
request/runtime that owns it. It must not retry indefinitely, replay an event
without an idempotency key, or turn a memory-engine response into a gateway
decision transition.

## Authentication and boundary checks

The gateway sets the private endpoint and shared secret through separate
configuration values. The memory engine does not receive `DATABASE_URL` or any
gateway event-log credential. Private exposure is a deployment boundary: the binary defaults to
`0.0.0.0:43121`, configurable with `THOUGHT_KHORAL_MEMORY_LISTEN_ADDRESS`.
`THOUGHT_KHORAL_MEMORY_SHARED_SECRET` supplies the credential sent as
`Authorization: Bearer <secret>`. The handler rejects requests without that
credential and uses Axum's default JSON body limit. This proof provides no
production TLS or deployment procedure.

The integration test must prove that an authenticated request can be accepted
without changing active decisions, while an unauthenticated request, a
cross-room provenance request, and a malformed request are rejected before any
graph write. Test fixtures must use non-sensitive text and identifiers.

## Task 6 interface and sequence test plan (implementation checklist)

The approved Task 6 plan established these tests before runtime composition.
The checklist describes that proof boundary; current runtime evidence is not
regenerated by this documentation update. It covers:

1. an authenticated committed event reaching the memory engine only after the
   gateway persistence step;
2. the envelope preserving one `roomId`, event ID, type, timestamp, and
   normalized payload;
3. an ingestion receipt acknowledging derived graph work without creating an
   active decision or invoking `decision.transition`;
4. an unavailable, slow, malformed, or queue-saturated memory engine leaving
   chat completion and governed human decision actions unaffected;
5. rejection of missing/cross-room provenance without partial graph writes;
6. absence of the gateway event-log database credential from the memory-engine
   process configuration; and
7. bounded shutdown and no unbounded retry/replay behavior.

The initial contract tests use an in-process HTTP server. Task 6 added the
separate memory-engine process and phase-gated platform composition for the
cross-process failure-isolation proof. This does not authorize production
deployment or a facilitator-provider switch.
