# POC room-scoped memory

## Status

Approved

## Purpose

Define the first Cognee-backed collective-memory capability for ThoughtKhoral
without changing the governed-room MVP. A room remains one conversation.
Derived facts and draft decisions are scoped to that room. Humans continue to
activate context through Confirm, Edit, or Dismiss.
Decisions made in the room are conversation-scoped specifications: an active
human-approved decision is the normative source of truth, while derived memory
supplies supporting shared context without becoming authoritative by itself.

## Scope

`thought-khoral-memory-engine` owns room-scoped graph derivation, provenance,
temporal lineage, and a later implementation of the gateway facilitator port.
This What specifies product outcomes for a POC integration of Cognee-RS. It
does not authorize runtime code by itself. The approved, task-gated
[implementation plan](../how/poc-room-scoped-memory-implementation-plan.md)
permits the current graph and private-ingestion proof; it does not authorize
Cognee integration or production deployment.

The capability must:

- Consume already-persisted room events for a single `roomId`.
- Partition all derived facts, embeddings, and lineage by that `roomId`.
- Preserve provenance through source-event identifiers and timestamps.
- Maintain a room-scoped distinction between authoritative active decision
  specifications and non-authoritative derived context such as facts,
  relationships, embeddings, and retrieval hints.
- Make the same room-scoped context available through mediated human and agent
  projections; agents receive read-only context and cannot activate or revise a
  decision specification.
- Occupy the facilitator draft-proposal port rather than sit beside it or
  replace the port.
- Leave the live `Decision:` implementation in place until an approved
  implementation plan switches or adds a memory-engine implementation.
- Emit drafts, if any, only as facilitator-port proposals that the gateway
  records as `decision.proposed`.
- Leave `decision.transition` and active collective memory to the gateway and
  human participants.

Governing documents: root [solution architecture](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/what/n2n-solution-architecture.md) and [decision 005](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md).

## Exclusions

- Project-level memory, topic-level memory, nested conversations, and any
  conversation or parent-project identifier.
- Deleting the facilitator port or replacing the live `Decision:`
  implementation before Cognee is proven.
- A second independent propose path that bypasses the facilitator port.
- Invoking `decision.transition`, writing active room context, or bypassing
  human Confirm / Edit / Dismiss.
- Treating a derived fact, embedding, retrieval result, or extraction output as
  an active decision specification without a human transition.
- Changing `n2n.room.v1` wire values, database identifiers, or persisted event
  fields.
- Granting Cognee or the memory engine gateway database credentials,
  filesystem shell access, or unmediated side-effecting tools.
- Embedding Cognee inside the gateway process as an unmediated model call.
- Remote third-party A2A/MCP agents, local model hosting as a product feature,
  and production OpenShift topology.
- Runtime packages, binaries, migrations, or deployment artifacts before an
  approved How design, implementation plan, license review, and compatibility
  evidence.

## Acceptance criteria

1. Derived memory for the POC is partitioned by `roomId` and does not introduce
   a conversation identifier.
2. Project/topic memory hierarchy is documented as an explicit non-goal of this
   capability.
3. The facilitator remains the only draft-creation path from room events, and
   the live `Decision:` implementation still produces drafts from matching chat
   messages until a later plan switches implementations.
4. A Cognee-derived draft, when authorized, enters through the facilitator port
   and remains `draft` until a human Confirm, Edit, or Dismiss.
5. Collective memory continues to show only gateway-active decisions.
6. Active decisions are normative room specifications; derived graph and
   retrieval context remains supporting and non-authoritative.
7. Human and agent context projections are room-scoped, and agent projections
   are read-only without adding a new `n2n.room.v1` method or field.
8. No runtime implementation exists until an implementation plan for this What
   and How is approved.
