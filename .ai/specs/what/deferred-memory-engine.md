# Deferred memory engine

## Status

Superseded by [POC room-scoped memory](poc-room-scoped-memory.md).

## Planned responsibility

`thought-khoral-memory-engine` was reserved as a specification-only project
boundary and identity. That placeholder is replaced by the POC room-scoped
memory What specification. Historical compatibility rules below remain true
and are restated there.

## Compatibility boundary

Any implementation must consume `n2n.room.v1` without changing its wire values
and must preserve existing database identifiers and persisted values unless a
separate migration decision is accepted.

## Explicit exclusions

This historical placeholder did not authorize runtime code, packages,
binaries, database schemas, migrations, deployment artifacts, or service
integrations. The later approved POC implementation plan authorizes only its
task-gated graph and mediated-ingestion proof; Cognee and production scope
remain deferred.
