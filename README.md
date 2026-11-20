# ThoughtKhoral memory engine

[Project documentation](docs/README.md) · [Documentation requirements](.ai/specs/what/public-documentation.md)

## Current workspace context — 2026-11-20

This component remains the incubating in-memory room/provenance and private mediated-ingestion proof at runtime baseline `7b46518`. The published contract and prepared Codex consumers/profile do not enable memory retrieval/writes, an active facilitator, Cognee or durable storage. Existing fixture/receipt/error-handling proof limits and human activation authority remain as specified. No new memory runtime test or implementation is claimed by this documentation refresh.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

`thought-khoral-memory-engine` is the incubating implementation and
specification home for collective-memory, provenance, and temporal lineage.
The first capability is an incubating room-scoped POC: a room remains one
conversation, and Cognee is a later implementation of the gateway facilitator
port.

## Status

Phase 3 mediated-ingestion proof / incubating. This repository contains the
backend-neutral in-memory graph, provenance, context model, and a private
authenticated HTTP ingestion process. An inactive draft adapter validates
room scope and provenance; it does not generate live proposals or activate
decisions. There is no Cognee integration, durable storage, or production
deployment procedure.

The shared [Codex conversation status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md) treats this repository as a separate, incubating memory lane; Codex memory integration is not part of milestone one.

The private ingestion endpoint accepts gateway-mediated committed room events;
it is not a retained-v1 browser protocol implementation, a direct gateway
database consumer, or an active decision authority. The gateway
`Decision:` prefix parser is retired under root
[Decision 008](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/008-slash-decisions-and-facilitator-boundary.md);
no automatic prefix or memory proposer is active. Activating a later facilitator
implementation requires its own approved plan and contract review.

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

This README and the current ingestion description are maintained from the
approved local What/How and root Decision 008. The verification record preserves
the earlier POC checkpoint as historical evidence; it is not a current runtime
verification claim.
