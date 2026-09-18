# Mediated committed-event ingestion

## Decision

The memory engine will receive committed room events through a private HTTP
endpoint owned by the memory-engine process. The gateway will use a
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

## Internal envelope

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

The implementation may use the Rust field names internally, but the wire
mapping must remain explicit and versioned as an internal interface. The
memory engine validates that:

- `roomId` is present and is the sole partition scope for the operation;
- `eventId`, `eventType`, `occurredAt`, and `payload` are present and valid;
- derived objects retain the event ID and timestamp as provenance;
- a duplicate event is handled idempotently rather than producing duplicate
  graph writes; and
- no request can provide a second room scope or source an event from another
  room.

The response is one of:

```json
{"status":"accepted","roomId":"<UUID>","eventId":"<UUID>"}
```

or a typed error with a stable safe code, for example
`unauthorized`, `malformed_request`, `scope_mismatch`, `duplicate_event`, or
`backend_unavailable`. Responses and logs contain only safe room/event IDs and
error codes. Raw room text, titles, summaries, access tokens, and database
credentials are not logged.

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
HTTP error, malformed response, or memory-engine outage produces a safe
diagnostic and drops or retries the derived-memory job according to the later
bounded retry policy; it never rejects the already committed chat event. The
existing deterministic `Decision:` facilitator remains available in the
gateway throughout this phase.

The dispatcher must be shut down with the gateway and must not outlive the
request/runtime that owns it. It must not retry indefinitely, replay an event
without an idempotency key, or turn a memory-engine response into a gateway
decision transition.

## Authentication and boundary checks

The gateway sets the private endpoint and shared secret through separate
configuration values. The memory engine does not receive `DATABASE_URL` or any
gateway event-log credential. The endpoint binds only to the private service
interface, rejects requests without the configured credential, and applies a
request body limit before deserialization.

The integration test must prove that an authenticated request can be accepted
without changing active decisions, while an unauthenticated request, a
cross-room provenance request, and a malformed request are rejected before any
graph write. Test fixtures must use non-sensitive text and identifiers.

## Task 6 interface and sequence test plan

Before adding runtime composition, write failing tests for:

1. an authenticated committed event reaching the memory engine only after the
   gateway persistence step;
2. the envelope preserving one `roomId`, event ID, type, timestamp, and
   normalized payload;
3. an ingestion receipt acknowledging derived graph work without creating an
   active decision or invoking `decision.transition`;
4. an unavailable, slow, malformed, or queue-saturated memory engine leaving
   chat completion and the deterministic gateway facilitator unaffected;
5. rejection of missing/cross-room provenance without partial graph writes;
6. absence of the gateway event-log database credential from the memory-engine
   process configuration; and
7. bounded shutdown and no unbounded retry/replay behavior.

The test process may use an in-process HTTP server for the first contract
tests. A separate memory-engine process and platform composition are added only
in Task 6, when the cross-process failure-isolation test requires them.
