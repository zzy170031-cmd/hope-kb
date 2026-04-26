# LLM Wiki Governance Model v0.2

Status: total-control baseline, pending lane integration.

## Model Summary

The LLM Wiki model separates knowledge creation from runtime consumption. The
repo may hold richer governance evidence, but Hope only receives compact,
verified, read-only snapshot products.

The four layers are:

- `raw`: evidence, imports, quarantines, and candidate material
- `wiki`: reviewed explanations and human-readable governance notes
- `schema`: machine-readable contracts, IDs, gates, and relationships
- `snapshot`: immutable verified consumption artifact

The four operations are:

- `Ingest`: accept and review material
- `Query`: retrieve bounded, summary-only knowledge
- `Lint`: block unsafe, incomplete, stale, or leaky material
- `Future QA`: record later experiments without changing v0.2 defaults

## Layer Rules

### raw

The raw layer is for controlled intake and audit. It can hold candidate source
metadata and quarantine records, but runtime consumers must not scan it or use
it as fallback context.

Promotion out of raw requires review status, effective confidence, normalized
source fields, Git-visible seed or schema changes, rebuild, and verification.

### wiki

The wiki layer makes the knowledge understandable to reviewers and branch
threads. It can explain prompt intent, scene logic, failure patterns, repair
choices, source rationale, and unresolved decisions.

Wiki text is not a substitute for schema gates. A claim in wiki form becomes a
runtime-relevant rule only after it is represented in schema or snapshot
validation.

### schema

The schema layer carries contracts that automation can check:

- stable IDs and joins
- source minimum fields
- manifest and bundle counts
- snapshot activation descriptor fields
- router scoring and fallback fields
- eval freshness and per-intent thresholds
- structural leakage deny-lists
- telemetry and rollback allowed fields

Schema expansion remains centralized. Local lane threads should report proposed
fields to control instead of improvising downstream contract changes.

### snapshot

The snapshot layer is the release handoff surface. A snapshot is active only
after the selected manifest, snapshot artifact, and runtime selection index are
bound by hash and pass validation.

If activation fails, the consumer should keep the current verified snapshot,
use a verified last-known-good snapshot, or fail closed with `no_kb_context`.

## Operation Rules

### Ingest

Ingest must be explicit and reviewable. It never means runtime auto-ingest.

Minimum ingest evidence:

- source identifier
- source type
- source path or URL handle
- content hash or equivalent digest
- review status
- effective confidence
- promotion or quarantine decision

### Query

Query must respect intent, eligibility, and leakage boundaries.

Allowed v0.2 query output is compact:

- selected sample IDs
- selected KB rule IDs
- fallback reason code
- bounded payload size
- compressed `kb_context_summary`

Disallowed query output includes raw rows, full prompt bodies, raw graph data,
full source register entries, overlay JSON, local paths, and credential values.

### Lint

Lint is the first protection layer for both governance and runtime handoff. A
v0.2 lint gate should fail on:

- missing source minimum fields
- manifest, snapshot, or index hash mismatch
- candidate or quarantine leakage into active selection
- stale eval artifact
- per-intent sample insufficiency
- payload structural leakage
- short-input expensive path
- telemetry purge residue

### Future QA

Future QA is allowed to collect controlled questions for v0.2.1+, but it must
not silently become default runtime behavior.

Recorded future topics include signatures, provenance attestation, shadow-only
hybrid/rerank, compact evidence packs, maintenance review UI, cloud telemetry
opt-in, token multiplier calibration, and rollback threshold tuning.

## Control Decisions

- The main control thread is the only integrator for cross-lane contract
  changes.
- Lane reports must include branch/HEAD, git status, modified files,
  validation results, risks, and whether they recommend commit/push.
- Control stages only an explicit whitelist and should not stage `AGENTS.md`,
  snapshot SQLite files, or build side effects unless the user explicitly
  reopens that boundary.
