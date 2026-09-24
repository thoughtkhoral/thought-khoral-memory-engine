# ThoughtKhoral memory engine

`thought-khoral-memory-engine` is the incubating implementation and
specification home for collective-memory, provenance, and temporal lineage.
The first planned capability is room-scoped POC memory: a room remains one
conversation, and Cognee is a later
implementation of the gateway facilitator port rather than a second proposer.

## Status

Phase 3 mediated-ingestion proof / incubating. This repository contains the
backend-neutral in-memory graph, provenance, context model, and a private
authenticated HTTP ingestion process. It has no Cognee integration, durable
storage, or production deployment procedure.

The private ingestion endpoint accepts gateway-mediated committed room events;
it is not a retained-v1 browser protocol implementation, a direct gateway
database consumer, or an active decision authority. A later facilitator
integration requires its own approved plan and contract review.

Run `cargo fmt --check` and `cargo test` to verify this proof. The
[POC verification record](docs/poc-verification.md) and private
[mediated-ingestion design](docs/mediated-ingestion.md) describe its evidence
and trust boundary; it is not a public room-protocol or production API.

Read the [local specification index](.ai/specs/README.md), the
[POC room-scoped memory specification](.ai/specs/what/poc-room-scoped-memory.md),
and the [ThoughtKhoral repository map](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/repository-map.md).

## Contributing

Use an issue to propose scope, contracts, security boundaries, or evidence for
an implementation plan. Further runtime work remains task-gated by that
approved implementation plan. See the
[organization contribution guide](https://github.com/thoughtkhoral/.github/blob/main/CONTRIBUTING.md).
