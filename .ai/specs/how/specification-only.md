# Specification-only boundary

## Status

Historical boundary. The approved
[POC implementation plan](poc-room-scoped-memory-implementation-plan.md)
authorizes task-gated graph and ingestion proof work, but not Cognee or
production deployment.

Follow the root [ThoughtKhoral identity migration design](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/thoughtkhoral-identity-migration.md), [Decision 003](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/003-thoughtkhoral-product-identity.md), and [Decision 005](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md).

The accepted local [ThoughtKhoral identity decision](../decisions/002-thoughtkhoral-identity.md) establishes the repository name. The accepted local [POC room-scoped memory decision](../decisions/003-poc-room-scoped-memory.md) establishes product intent for the first Cognee integration.

The approved implementation plan and its tests govern the current limited
runtime proof. Dependency license/compatibility review remains required before
any new Cognee adoption under the [POC How](poc-room-scoped-memory.md).

The `n2n.room.v1` wire value, database identifiers, and persisted values remain unchanged. Project-level and topic-level memory are excluded from this repository until a later root decision authorizes them.
