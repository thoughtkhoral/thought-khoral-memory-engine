# Specification-only boundary

Follow the root [ThoughtKhoral identity migration design](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/thoughtkhoral-identity-migration.md), [Decision 003](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/003-thoughtkhoral-product-identity.md), and [Decision 005](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md).

The accepted local [ThoughtKhoral identity decision](../decisions/002-thoughtkhoral-identity.md) establishes the repository name. The accepted local [POC room-scoped memory decision](../decisions/003-poc-room-scoped-memory.md) establishes product intent for the first Cognee integration.

No runtime implementation begins until a dedicated implementation plan, tests, and dependency license/compatibility evidence are approved against the [POC How](poc-room-scoped-memory.md).

The `n2n.room.v1` wire value, database identifiers, and persisted values remain unchanged. Project-level and topic-level memory are excluded from this repository until a later root decision authorizes them.
