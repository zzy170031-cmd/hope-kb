# QA Hardening v0.2 Snapshot Source Integrity 2026-04-25

## Scope

This packet defines the v0.2 snapshot/source integrity contract at RFC level.
It is implementation-neutral and does not open runtime retrieval design,
desktop UI work, Hope mainline changes, or LLM summarization.

Current tree anchors inspected for this packet:

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- HEAD: `d5b319c`
- v0.2 manifest: `seed/v0.2/manifest.json`
- v0.2 active bundle hash:
  `bundle-sha256:b16bb3719d73c7246eef674838130bc671879263f5889075cfbc4fbe1fff049e`
- v0.2 counts: `golden_sample_library=152`,
  `golden_sample_sources=17`, `golden_sample_provenance_entries=5`

## Integrity Hash Contract

The active runtime KB set is a three-part contract:

1. `manifest_hash`
2. `snapshot_hash`
3. `index_hash`

All three values are release/install artifacts, not per-query calculations.

### `manifest_hash`

`manifest_hash` is the SHA-256 digest of the canonical active manifest
descriptor. The descriptor binds at minimum:

- `snapshot_version`
- `snapshot_name`
- seed bundle hash, currently stored as `manifest.content_hash`
- `bundle_order`
- record counts
- source register digest or equivalent source-register binding
- expected `snapshot_hash`
- expected `index_hash`

The current v0.2 `manifest.content_hash` remains the seed bundle hash. It is
not, by itself, a complete runtime integrity descriptor until it also binds
the selected snapshot and selected runtime index.

### `snapshot_hash`

`snapshot_hash` is the SHA-256 digest of the immutable snapshot artifact that
will be opened by a runtime consumer. For the current SQLite snapshot path,
this means the selected SQLite file bytes or another deterministic snapshot
artifact format approved by the KB integration owner.

The snapshot must also carry enough internal metadata to prove that it was
built from the same manifest version and seed bundle hash. A snapshot whose
internal metadata disagrees with the active manifest descriptor is invalid
even if the file exists.

### `index_hash`

`index_hash` is the SHA-256 digest of the runtime selection index or lookup
descriptor derived from the active snapshot. This does not imply GraphRAG,
hybrid search, reranking, vectors, or any model-side default. The index may be
a primary-key lookup map, eligibility filter table, source metadata view, or a
minimal deterministic descriptor over those runtime-selection surfaces.

If a consumer uses the snapshot directly and has no separate index file, it
still needs a deterministic index descriptor whose digest records:

- selected snapshot identity
- tables/views used for runtime selection
- primary key fields
- row counts
- quarantine and candidate exclusion predicates
- few-shot/runtime eligibility predicates

Any change to the runtime selection surface changes `index_hash` and requires
full verification before activation.

## Verification Timing

### Install or Snapshot Switch: Full Verification

Full verification is required before installing a new snapshot or switching
the active snapshot pointer. It must verify:

- `manifest_hash`
- `snapshot_hash`
- `index_hash`
- manifest record counts against the snapshot/import surface
- snapshot internal metadata against the active manifest descriptor
- source register minimum fields
- quarantine/candidate exclusion from runtime selection
- basic snapshot health, for example SQLite integrity checks when SQLite is
  the chosen artifact

Activation succeeds only after the entire active KB set verifies as one
coherent unit.

### Startup: Light Active Check

Startup should perform a light check of the already activated KB set:

- active descriptor exists and is readable
- active snapshot path exists
- active index/descriptor path exists if applicable
- stored hash values match the active descriptor
- snapshot metadata still names the expected version and seed bundle hash
- cheap file identity checks, such as size/mtime fingerprints, have not drifted
  from the last verified activation record

Startup should not rebuild the snapshot, scan all rows, or perform source-file
rehashing. If the light active check fails, runtime KB context is unavailable
unless a verified `last_known_good_snapshot` can be activated.

### Runtime Query: No Rehash

Runtime queries must never rehash the manifest, snapshot, index, source files,
or full tables. Query-time work may read only the already verified active
snapshot/index and return bounded KB context according to the active selection
contract.

## Failure Behavior

On any install/switch verification failure:

- do not activate the candidate snapshot
- keep the current verified active snapshot if it is still healthy
- otherwise activate a verified `last_known_good_snapshot`
- if no verified fallback exists, fail closed with `no_kb_context`

On startup active-check failure:

- try a verified `last_known_good_snapshot` only if its descriptor and hashes
  were previously verified
- otherwise fail closed with `no_kb_context`

The failure path must not:

- scan the full seed or snapshot table to reconstruct context
- fetch network rescue data
- load unreviewed temporary files
- promote quarantine or candidate data
- silently continue with a mismatched manifest, snapshot, or index

## Source Register Minimum Fields

Every source eligible to support an active runtime snapshot must provide these
minimum normalized fields:

- `source_id`: stable unique source identifier
- `source_type`: controlled source category
- `url_or_path`: canonical local path or URL for the reviewed source
- `content_hash`: algorithm-prefixed source content hash, for example
  `sha256:<digest>`
- `review_status`: controlled review state
- `effective_confidence`: review-derived confidence usable by runtime policy

The current v0.2 register uses `path` and `sha256`; a future schema or adapter
may normalize those into `url_or_path` and `content_hash`. Missing
`review_status` or missing `effective_confidence` must exclude the source from
new runtime activation until the source is reviewed and the active manifest is
rebuilt.

Recommended `review_status` semantics:

- `accepted`: eligible for active snapshot selection
- `accepted_limited`: eligible only under explicit limits recorded in the
  active manifest/index descriptor
- `candidate`: not runtime-selectable
- `quarantined`: not runtime-selectable
- `rejected`: not runtime-selectable

`effective_confidence` must be derived from review outcome, not copied blindly
from source claims. Runtime policy must define the minimum confidence for each
selection surface; absent or unknown confidence excludes the source.

## Quarantine and Candidate Isolation

The following stores are isolation zones, not runtime sources:

- `quarantine_raw`
- `quarantine_labeled`
- `seed_candidate`

Entries in these zones may be hashed and tracked for audit, but they must not
appear in the active manifest bundle, active snapshot, active index, runtime
selection views, or runtime KB context.

Promotion out of an isolation zone requires:

- source minimum fields
- human review outcome
- explicit `review_status`
- explicit `effective_confidence`
- manifest/snapshot/index rebuild
- install or switch full verification

No runtime consumer may scan quarantine or candidate zones as a fallback when
the active snapshot fails.

## v0.2.1 Deferred Items

The following are intentionally deferred to v0.2.1 or later and are not required
for the v0.2 hardening acceptance gate:

- signature chain over manifest, snapshot, and index descriptors
- public key verification
- provenance attestation beyond the current source register and provenance
  entries

These deferred items should extend the same hash contract instead of replacing
it.

## Acceptance Tests

### Tampered Manifest

Given an otherwise valid snapshot package, modify the active manifest descriptor
or seed bundle hash. Full verification must fail before activation. The runtime
must continue with the prior verified active snapshot, activate a verified
`last_known_good_snapshot`, or return `no_kb_context`.

Expected result: no candidate activation, no full-table scan, no temporary data
rescue.

### Tampered Snapshot

Given a valid manifest descriptor, modify the selected snapshot artifact or its
internal metadata. Full verification must fail on install/switch. Startup light
check must also reject an already-active snapshot whose metadata or cheap file
identity no longer matches the last verified activation record.

Expected result: fallback only to a verified `last_known_good_snapshot`, else
`no_kb_context`.

### Tampered Index

Given a valid manifest and snapshot, modify the runtime selection index or
descriptor, for example by adding a reserve/candidate row or changing an
eligibility predicate. The `index_hash` check must fail before activation.

Expected result: runtime selection cannot use the modified index, and no query
may select rows through it.

### Active Snapshot Missing or Unreadable

Point the active descriptor to a missing, locked, or unreadable snapshot. The
startup light active check must fail without rebuilding the snapshot or scanning
seed files.

Expected result: verified `last_known_good_snapshot` if available, otherwise
`no_kb_context`.

### Quarantine and Candidate Exclusion

Place valid-looking records in `quarantine_raw`, `quarantine_labeled`, or
`seed_candidate`. Full verification must confirm that these records are absent
from the active manifest bundle, active snapshot, active index, and runtime
selection surfaces.

Expected result: isolated data remains unavailable to runtime selection until a
future promotion gate rebuilds and verifies the active KB set.

### Source Minimum Field Failure

Remove or blank `source_id`, `source_type`, `url_or_path`, `content_hash`,
`review_status`, or `effective_confidence` from a source that would support an
active runtime snapshot.

Expected result: full verification fails for new activation, or the source is
treated as non-runtime-selectable until reviewed and normalized.

### Query-Time Rehash Guard

Run repeated runtime queries against an already verified active snapshot.
Instrumentation should show no manifest, snapshot, index, source-file, or
full-table rehash in the query path.

Expected result: query-time behavior uses only the verified active
snapshot/index and returns `no_kb_context` if that active handle is unavailable.

## Non-Goals

This packet does not authorize:

- GraphRAG, hybrid search, rerank defaults, or vector index policy
- Hope desktop UI expansion
- Hope mainline runtime changes
- runtime LLM summarization
- network rescue
- unreviewed temporary-data fallback
- promotion of reserve, quarantine, or candidate data without a future gate
