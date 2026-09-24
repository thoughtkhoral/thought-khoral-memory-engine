# 003 — POC room-scoped memory

## Status

Accepted

## Context

Root [decision 005](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/005-room-scoped-poc-memory.md) scopes the first Cognee capability to one room as one conversation and excludes project/topic memory. This repository previously recorded only a specification-only placeholder.
Root [decision 008](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/008-slash-decisions-and-facilitator-boundary.md)
later superseded only decision 005's live `Decision:` parser requirement.

## Decision

This project implements root decision 005 locally. The authoritative What is
[POC room-scoped memory](../what/poc-room-scoped-memory.md). The placeholder
[deferred memory engine](../what/deferred-memory-engine.md) document is
superseded. Runtime code remains unauthorized until a dedicated
implementation plan is approved.

The `n2n.room.v1` wire value, database identifiers, and persisted values remain
unchanged. The facilitator is the draft-proposal port. Cognee occupies that
port as a later implementation and must not invoke `decision.transition` or sit
beside another derived-draft implementation as a second independent proposer.
The retired `Decision:` parser must not be restored as part of this POC.

## Alternatives considered

Recorded in root decision 005; this project adds no local alternative.

## Consequences

- Local specifications now describe POC product intent and design constraints
  without authorizing packages, binaries, or deployments.
- A later project/topic memory model requires a new root decision and local
  specification update; it is out of scope here.

## Overrides

None.
