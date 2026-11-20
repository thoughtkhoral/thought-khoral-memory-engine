# Memory engine documentation requirements

## Status and authority

Approved documentation scope, refreshed by the maintainer's 2026-11-20 request
to update documentation specifications before derived pages at workspace and
individual-project level. This supersedes stale scaffold/MVP-only documentation
criteria, without changing approved runtime requirements or acceptance gates.

Read the [local specification index](../README.md), [component requirements](poc-room-scoped-memory.md)
and [implementation design](../how/poc-room-scoped-memory.md) first. Root
[documentation requirements](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/what/open-source-documentation.md)
and [current workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md)
apply. Documentation cannot approve implementation, publication or activation.

## Maintained documentation set

| Page | Required role |
| --- | --- |
| [Repository README](../../../README.md) | Purpose, maturity, prerequisites, valid commands, compatibility and source links. |
| [Project documentation index](../../../docs/README.md) | Local documentation entrypoint mapping maintained pages to their governing specs and checkout applicability. |
| [Private ingestion](../../../docs/mediated-ingestion.md) | Gateway-mediated committed-event input and provenance. |
| [Proof verification](../../../docs/poc-verification.md) | Retained execution evidence and unclosed proof requirements. |

## Acceptance criteria

- Update applicable documentation specs and accepted evidence first, then
  maintain these authored Markdown pages. CI validates; it does not generate
  prose or rewrite specifications.
- Map each maintained page to its governing local What/How, and link from the
  repository README to this documentation specification and the project index.
- Identify canonical runtime versus prepared-branch applicability. Documentation
  HEADs, published artifacts and frozen runtime acceptance inputs are distinct;
  do not repin a tested source because a prose-only followup advanced HEAD.
- Describe the implemented incubating in-memory room/provenance and private-ingestion proof without implying durable or production memory.
- Preserve gateway-owned normative decisions and human activation authority; no automatic or Codex memory integration is enabled by the current preparation.
- State fixture-only extraction and remaining proof limits from the POC implementation plan; do not turn missing validation into accepted behavior.
- Keep Cognee adoption, durable storage, facilitator activation and production operation subject to their own approved scope.
- Preserve historical evidence and existing anchors. Attribute results to exact
  source/image/contract inputs and distinguish implemented, verified, published,
  enabled and production-ready states.
- Keep private configuration, provider keys, human tokens, native sessions and
  room contents outside docs and fixtures. Preserve licenses and legal notices.
- Direct contributions through the issue/specification workflow and private
  security-reporting policy. Validate links, source references and whitespace;
  run any additional checks required by this project's agent instructions.

## Verification boundary

Documentation checks establish discoverability and consistency, not runtime,
network, browser or provider acceptance. Fresh verification must name what was
actually run; retained results keep their original source identity. Current
state comes from approved component/root evidence, not the age of this file.
