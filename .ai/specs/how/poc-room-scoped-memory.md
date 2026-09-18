# POC room-scoped memory — implementation design

## Status

Approved

## Governing specifications

- Governing What document: [../what/poc-room-scoped-memory.md](../what/poc-room-scoped-memory.md)
- Governing decision: root [005 — Room-scoped POC memory](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md)
- Compatibility: retained [`n2n.room.v1`](https://github.com/thoughtkhoral/thought-khoral-contracts/blob/main/protocol.md)
- Identity: [../decisions/002-thoughtkhoral-identity.md](../decisions/002-thoughtkhoral-identity.md)

This How records design constraints for the POC. It is not an implementation
plan and does not authorize runtime code, dependency adoption, or deployment.

## Components and responsibilities

`thought-khoral-room-gateway` remains the sole mediator of persisted room events
and active-context updates. It owns the facilitator **port**: after a committed
room event, it asks the port for zero or more draft proposals, persists
`decision.proposed`, and never lets that path invoke `decision.transition`.

The live implementation of that port stays the deterministic `Decision:`
parser in the gateway. `thought-khoral-memory-engine` is the intended later
implementation of the same port. It derives room-scoped graph facts and, when
an approved plan enables it, returns draft proposals to the gateway through
that port. It must not open a second independent `decision.propose` path.

`thought-khoral-workspace-ui` continues to render drafts and the active-only
collective-memory ledger. `thought-khoral-contracts` remains the compatibility
authority; this POC adds no method, field, or contract version.

The room context has two semantic layers. The gateway-owned active decision
ledger contains room-scoped decision specifications and is normative: an
active human-approved decision is the source of truth for the conversation.
The memory engine supplies non-authoritative supporting context—derived facts,
entities, relationships, embeddings, temporal lineage, and retrieval hints.
Derived context cannot activate, overwrite, or supersede a decision
specification without the existing human transition path.

Cognee-RS is the intended extraction engine, subject to later license,
maturity, and protocol compatibility evidence. Until that evidence exists, no
crate, service, or embedding pipeline may be added. Cognee must not be embedded
as an unmediated model call inside the gateway.

## Interfaces and data flow

1. The gateway persists an immutable room event.
2. The gateway asks the facilitator port for drafts derived from that event.
3. The live `Decision:` implementation may return one draft when the message
   text matches; otherwise it returns none.
4. The memory engine, when enabled behind the same port, consumes committed
   events for one `roomId`, stores derived nodes and edges partitioned by that
   `roomId`, and may return draft proposals with title, summary, and
   `sourceEventIds`. It may also provide a read-only supporting-context
   projection for that room; the projection is not authoritative.
5. The gateway persists any returned drafts as `decision.proposed`.
6. Humans Confirm, Edit, or Dismiss through the existing UI. Only then does
   collective memory show the decision.
7. A mediated context reader composes active gateway decision specifications
   with room-matched supporting derived context. Human and agent projections
   remain room-scoped; agent projections are read-only and add no `n2n.room.v1`
   method or field.

Event ingestion transport for a memory-engine implementation is not selected
here. Permitted later options are mediated forwarding of committed events from
the gateway into the facilitator-port implementation, never gateway event-log
database credentials in the memory engine. A racing authenticated agent that
calls `decision.propose` outside the port is not the target architecture.

Graph-only memory-engine work may precede a live Cognee facilitator
implementation. Drafts from memory still enter only through the port.

## Failure and security behavior

- Extraction failure must not block chat, join, the live `Decision:`
  implementation, or human transitions.
- The memory engine must not receive gateway database credentials, filesystem
  shell access, or unmediated tools.
- Rejected or malformed extraction output is dropped or surfaced as a
  structured error; it must not write active context.
- Derived context is informative only. Only an active gateway decision
  specification is normative room context; stale, superseded, dismissed, and
  cross-room items are excluded from active projections.
- Logs must not contain access tokens, raw room message text, titles, or
  summaries.
- A later project/topic hierarchy must not require un-mixing a global
  unpartitioned graph; `roomId` is the partition key for this POC.

## Verification

An implementation plan for this How must include:

- License and compatibility evidence for Cognee-RS before the dependency is
  pinned.
- Deterministic fixtures proving provenance, `roomId` partitioning, and that
  extraction cannot activate a decision.
- Regression proof that the live `Decision:` facilitator implementation still
  produces drafts, until a later plan switches implementations.
- Proof that no second independent propose path exists beside the facilitator
  port.
- Proof that collective memory still lists only active decisions.
- Proof that active decision specifications are distinct from supporting
  derived context and are the only normative room context.
- Proof that mediated human and agent context projections cannot cross rooms or
  let an agent activate or revise a decision specification.
- An explicit test that no conversation or project identifier is introduced
  on the `n2n.room.v1` wire.
