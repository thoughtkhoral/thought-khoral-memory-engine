# Deferred memory engine

## Planned responsibility

`thought-khoral-memory-engine` is reserved for a future, independently approved collective-memory capability. This repository records the project boundary and identity only; it does not authorize an implementation.

## Compatibility boundary

Any future implementation must consume `n2n.room.v1` without changing its wire values and must preserve existing database identifiers and persisted values unless a separate migration decision is accepted.

## Explicit exclusions

Runtime code, packages, binaries, database schemas, migrations, deployment artifacts, and service integrations are excluded from this specification-only repository.
