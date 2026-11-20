# Memory engine documentation

This index is maintained from the [documentation requirements](../.ai/specs/what/public-documentation.md).
Use the [repository README](../README.md) for prerequisites, valid commands and
checkout context; use the [specification index](../.ai/specs/README.md) for
requirements and evidence authority. The page/source map below follows the
current documentation contract, not a new runtime or release decision.

## Checkout applicability

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

The [workspace guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/workspace-status.md)
identifies canonical mains, prepared branches, published artifacts and exact
preserved inputs. Documentation-only HEADs do not replace frozen acceptance
commits or enable a prepared command in a checkout that lacks its implementation.

## Maintained pages and sources

| Page | Read it for | Governing specification |
| --- | --- | --- |
| [Repository README](../README.md) | Purpose, maturity, prerequisites, valid commands, compatibility and source links. | [Source](../.ai/specs/what/public-documentation.md) |
| [Private ingestion](mediated-ingestion.md) | Gateway-mediated committed-event input and provenance. | [Source](../.ai/specs/how/poc-room-scoped-memory.md) |
| [Proof verification](poc-verification.md) | Retained execution evidence and unclosed proof requirements. | [Source](../.ai/specs/how/poc-room-scoped-memory-implementation-plan.md) |

## Authoring and verification

1. Read the [component requirements](../.ai/specs/what/poc-room-scoped-memory.md) and
   [implementation design](../.ai/specs/how/poc-room-scoped-memory.md). Update applicable specs and
   accepted evidence before changing their documentation.
2. Update the README, this index and affected guides with checkout-specific
   applicability. Keep historical anchors and exact source/image/contract IDs.
3. Validate local/source links and whitespace, and follow any project agent
   instructions for additional checks. Documentation validation does not prove
   runtime, real-browser, network, provider or production acceptance.

These are authored Markdown pages. CI validates their structure/links and does
not generate prose or rewrite specifications. Keep credentials, private state
and user content out of documentation; preserve licensing and legal notices.
