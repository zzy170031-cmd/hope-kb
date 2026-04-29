# Source Governance Design Intake

Intake date: 2026-04-30

Source thread: `(running) HopePrompt KB - source acquisition and knowledge governance`

Original dispatch: `source-governance-dispatch.md`

Repository: `E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb`

Live Git reported by lane:

- branch: `codex/contracts-freeze`
- HEAD: `9391519`
- status: tracked files clean; untracked alignment artifacts `.codex/` and `openspec/`

## Intake Result

Status: accepted for total-control decision.

The lane satisfied the design-only dispatch:

- confirmed target path and Git anchor
- reviewed the required source governance, freshness activation, descriptor matrix, and OpenSpec files
- produced drafts for source acquisition boundaries, SourceDeltaBatch descriptor fields, fixture matrix, quality scoring rubric, sanitized diagnostics, aggregate-only rules, and the later Rust-first validator implementation gate
- did not edit files, stage, commit, push, run runtime fetch, auto-ingest, or request seed/snapshot/runtime/UI changes

## Total-Control Decisions

1. Approve opening a narrow Lane 1 docs-only document update gate.
2. Keep SourceDeltaBatch Rust implementation deferred.
3. Keep fixture file creation deferred.
4. Keep source acquisition implementation deferred.
5. Keep source quality scoring as review rubric only for v0.2; do not use it as activation or auto-promotion logic.
6. Allow future SourceDeltaBatch cross-descriptor participation only through aggregate hash, aggregate count, and source freshness digest; no per-source locator or evidence fields may flow to activation/runtime surfaces.
7. Defer the fixed `cargo +1.95.0` wrapper environment issue to a later environment or Rust implementation gate unless total-control opens a separate tooling gate.

## Accepted Design Content

### Source Acquisition Boundary

Accepted as design input:

- offline governance intake
- human-controlled source registration
- source intent review
- quarantine classification
- hash/digest requirements
- review status flow
- confidence rubric
- rejection reason taxonomy
- seed-candidate planning

Rejected from this gate:

- runtime fetch
- auto-ingest
- user-session writes
- runtime fallback source lookup
- query-time raw/wiki maintenance
- direct seed mutation
- snapshot mutation
- runtime selection expansion

Layering rule accepted:

- intake material remains governance-only
- labeled material may support wiki draft
- reviewed wiki may support future seed-candidate planning
- nothing becomes runtime-selectable without a later approved seed/snapshot/index rebuild and activation verification

### SourceDeltaBatch Descriptor Design

Accepted as design input:

- `descriptor_type=SourceDeltaBatch`
- governance descriptor artifact class
- aggregate-only source delta batch hash
- source delta count
- source freshness digest
- freshness status and stale reason codes
- review status and effective confidence summaries
- status counts for accepted/rejected/quarantined/limited source deltas
- required review completion flags
- normalized hash flags
- runtime exclusion flags
- activation request and activation blocked reason codes
- `leakage_count=0`

Boundary accepted:

- SourceDeltaBatch may support governance and future activation evidence.
- SourceDeltaBatch must not be copied into runtime payloads, logs, UI, or telemetry.
- Unknown fields should fail closed in the future validator.

### Fixture Matrix Design

Accepted as future fixture-design input only:

- minimal reviewed batch pass case
- mixed review status batch pass case
- activation-ready aggregate-only pass case
- missing required fields fail case
- bad hash shape fail case
- count mismatch fail case
- activation without review fail case
- denied locator/source payload fail case using synthetic sentinel fields only
- artifact class mismatch fail case
- nonzero leakage fail case
- unknown field fail-closed case

Fixture files are not authorized in the current gate.

### Source Quality Scoring Rubric

Accepted for docs-only rubric design:

- categorical labels: `high`, `medium`, `low`, `rejected`, `unknown`
- dimensions: authority, provenance completeness, content support, freshness, licensing/retention, leakage risk, conflict risk, applicability
- rubric output: categorical label plus summaries and bucket counts

Explicit boundary:

- quality scoring is a review aid only in this gate
- it must not auto-promote source, wiki, seed candidate, eval truth, snapshot, index, active pointer, or runtime selection

### Sanitized Diagnostics And Aggregate-Only Rules

Accepted design direction:

- diagnostics may include descriptor type, descriptor ID, field path, denied class, and rule ID
- diagnostics must not include matched values, raw source excerpts, source locator values, file path details, credentials, request/response bodies, provider details, or raw prompt/source/graph content
- activation/runtime-facing descriptors may use only aggregate source delta hash, count, and freshness digest
- matrix summaries remain leaf-level and count-only
- fail fixtures must use synthetic placeholders only

### Later Rust-First Validator Implementation Gate

Accepted as later gate outline only:

- add SourceDeltaBatch static rule module
- register SourceDeltaBatch in validator dispatch
- validate required fields, artifact class, freshness/stale enums, hash shapes, count consistency, review/confidence summaries, activation review preconditions, leakage zero, runtime exclusion, and unknown-field fail-closed behavior
- reuse global denied-field scan
- add Lane 1 synthetic pass/fail fixtures only after fixture write scope opens
- add unit tests for the new SourceDeltaBatch rule

Explicitly not included:

- source acquisition
- source storage
- source fetching
- seed mutation
- snapshot rebuild
- activation switch
- runtime consumption
- UI
- telemetry export
- canonical digest recomputation
- verified fallback exceptions

## Core Challenger Notes

- The lane report supports opening a docs-only update gate, not an implementation gate.
- The next docs must not imply that SourceDeltaBatch validator rules or fixtures already exist.
- The source quality rubric must not become activation logic without review, promotion, rebuild, validation, and activation binding gates.
- Aggregate binding must be preserved; per-source evidence must not spread into activation or runtime surfaces.

## Audit Specialist Notes

- Current next gate may edit only whitelisted docs.
- It must not touch runtime, UI, seed, snapshot, fixtures, exports, Rust code, staging, commit, or push.
- It must not output raw source text, per-source locator values, source register details, matched values, credentials, local path details, request/response bodies, or provider config.

## Next OpenSpec Action

Prepare an OpenSpec-reviewed dispatch for Lane 1 docs-only document updates.
