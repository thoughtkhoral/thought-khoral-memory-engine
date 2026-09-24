# POC room-scoped memory

## Status

Approved

## Purpose

Define the first Cognee-backed collective-memory capability for ThoughtKhoral
without changing the governed-room MVP. A room remains one conversation.
Derived facts and draft decisions are scoped to that room. Humans continue to
activate context through Confirm, Edit, or Dismiss; a human may also delete a
current decision while its audit event remains immutable.
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
- Do not restore the retired `Decision:` prefix parser. An approved later
  plan must activate any memory-engine implementation behind the existing
  gateway facilitator port.
- Emit drafts, if any, only as facilitator-port proposals that the gateway
  records as `decision.proposed`.
- Leave `decision.transition` and active collective memory to the gateway and
  human participants.

Governing documents: root [solution architecture](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/what/n2n-solution-architecture.md),
[decision 005](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md),
and its limited parser supersession in
[decision 008](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/008-slash-decisions-and-facilitator-boundary.md).

## Exclusions

- Project-level memory, topic-level memory, nested conversations, and any
  conversation or parent-project identifier.
- Deleting the facilitator port, restoring the retired `Decision:` parser,
  or activating a Cognee implementation before its separate approval.
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
3. The facilitator remains the only path for drafts derived from room events.
   The retired `Decision:` parser produces no drafts; a later approved plan
   must activate a memory-engine implementation behind the same port.
4. A Cognee-derived draft, when authorized, enters through the facilitator port
   and remains `draft` until a human Confirm, Edit, or Dismiss.
5. Collective memory continues to show only gateway-active decisions.
6. Active decisions are normative room specifications; derived graph and
   retrieval context remains supporting and non-authoritative.
7. Human and agent context projections are room-scoped, and agent projections
   are read-only without adding a new `n2n.room.v1` method or field.
8. The approved graph and private-ingestion proof does not activate Cognee or
   a memory-derived draft proposer; those require a later approved plan.
