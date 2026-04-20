# hope-kb Runtime Consume Integration Packet v0.1

This packet is the `hope-kb` side of the runtime handoff to `hope`.

It does not implement `hope` main-thread code.
It freezes the runtime-consumer contract that `hope` should trust when it reads the validated read-only snapshot.

## Purpose

- keep `hope-kb` as a support surface for `hope`
- prevent runtime-consumer drift from turning into ad-hoc local field guesses
- make `snapshot -> export sheet -> validator gate` expectations inspectable in Git

## Seed Asset

- `seed/v0.1/runtime_consume_contracts.json`

Each record declares:

- `consumer_surface`
- `contract_scope`
- `required_snapshot_tables`
- `required_export_sheets`
- `required_column_sources`
- `required_validator_gates`
- `blocked_local_overrides`
- `handoff_note`

## Current Surfaces

1. `snapshot_bootstrap`
   Trust `snapshot_meta`, `export_template`, `failure_pattern`, `degraded_input_example`, and `runtime_consume_contract` before runtime bootstrap.
2. `segment_and_cut_projection`
   Read `RenderSegments` and `Cuts` only through snapshot taxonomy, alias normalization, and continuity-safe export columns.
3. `handoff_projection`
   Read `HandoffZones` with `transition_type`, `buffer_cut_ids`, and `continuity_notes`; do not collapse handoff payloads to pair labels.
4. `prompt_package_projection`
   Read `PromptPackage` directly from the snapshot contract; do not leak aliases, validation text, or stale prompt schemas.
5. `validation_feedback_projection`
   Keep validator payloads replayable and blocking-aware; do not drop `message` or `related_cut_or_segment`.

## Validation Gate

`scripts/validate-seed-bundle.ps1` now treats `runtime_consume_contracts.json` as a hard gate:

- every surface must exist exactly once
- every required table must exist in the snapshot import map
- every required sheet and column source must exist in `export_templates`
- every required validator gate must already exist in the KB failure / degraded-input chain
- every surface must explicitly block the local override patterns that caused earlier support-facing drift

## Hope Boundary

The next product-side step belongs to `hope`, not to this repo:

- `hope` should consume the validated snapshot and trust these contracts
- `hope-kb` should keep evolving the contract, validator, and support cases
- no thread merge is implied by this packet
