# Prompt Knowledge Source Governance v0.2

## Status

Status: docs-only governance contract.
Scope: `raw_layer` and `wiki_layer` mapping for Prompt knowledge source intake,
quarantine, wiki drafting, reviewed wiki promotion, and QA acceptance.

This document is for `hope-kb` Prompt knowledge governance only. It does not
modify seed JSON, import maps, migrations, snapshot SQLite files, Hope
main-thread code, desktop UI, image generation, video generation, runtime
ingestion, or default runtime selection.

`raw_layer` and `wiki_layer` are governance, compile, and QA surfaces. They are
not runtime-maintained stores. They must not be scanned at query time, used as a
fallback when the verified snapshot fails, or written from user sessions.

## Relationship To v0.2 QA Contracts

This contract extends the v0.2 QA hardening boundary:

- unreviewed sources stay outside active manifests, snapshots, indexes, runtime
  selection, eval truth, payloads, logs, telemetry, and shadow records
- source records must normalize `source_id`, `source_type`, `url_or_path`,
  `content_hash`, `review_status`, and `effective_confidence` before they can
  support future active runtime artifacts
- quarantine zones remain isolated until a future human-reviewed promotion gate
  rebuilds and verifies the active seed bundle, snapshot, and selection index
- allowed disclosure remains summary-only; raw prompt bodies, raw KB rows, raw
  source text, full source registers, local paths, and secrets are denied

The first v0.2 use of an LLM Wiki architecture is therefore limited to offline
governance, compilation assistance, conflict detection, and QA review support.
It is not a default runtime auto-maintenance path.

## Layer Mapping

### `raw_layer`

`raw_layer` is the intake and isolation layer for Prompt knowledge materials.
Its purpose is to preserve provenance, review state, and risk classification
before any material can influence a reviewed wiki page or future seed proposal.

Allowed raw-layer zones:

- `quarantine_raw`: original untrusted or unreviewed material, stored only in
  controlled local governance surfaces with a stable hash and reason code
- `quarantine_labeled`: reviewed-enough-to-label material with source metadata,
  candidate type, risk notes, and extraction notes, still not runtime-eligible
- `seed_candidate`: a human-review-ready proposal for future seed work, still
  excluded from active manifests, snapshots, indexes, eval truth, and runtime
  selection until a later approved seed/snapshot gate

Raw-layer entries may be counted, hashed, classified, rejected, or promoted to
wiki draft. They must not be emitted into chat, payloads, telemetry, logs, or
runtime context as raw text.

### `wiki_layer`

`wiki_layer` is the reviewed knowledge synthesis layer for Prompt governance.
It turns source-backed claims into compact, auditable wiki pages that humans can
review and future validators can compile against.

Allowed wiki-layer states:

- `wiki_draft`: LLM-assisted or human-authored synthesis awaiting review
- `reviewed_wiki`: human-reviewed knowledge page with source coverage,
  review status, effective confidence, open questions, and schema links

The wiki layer may support QA compilation, duplicate detection, source coverage
review, conflict review, and seed-candidate planning. It does not directly
write seed files, mutate snapshots, or expand runtime selection.

## Prompt Knowledge Types

The first governance pass recognizes these Prompt knowledge entity types:

- `prompt_sample`: source-backed example or pattern sample for future
  few-shot or benchmark-facing work
- `prompt_rule`: reusable rule about storyboard, animation, camera,
  continuity, structure, or prompt composition
- `prompt_failure_pattern`: known failure mode, negative default, ambiguity,
  leakage risk, schema-mismatch pattern, or degraded-input pattern
- `prompt_repair_strategy`: source-backed or team-reviewed repair move that
  maps a failure pattern to a safer prompt construction

Each entity must keep source provenance separate from synthesized guidance.
Style experience may be recorded, but it must be marked as team distillation or
limited confidence when it is not an authority-backed rule.

## Required Fields

### Source Record

Every source considered for Prompt knowledge governance must carry:

```yaml
source_id: stable unique identifier
source_type: official_rule | official_product | team_distillation | internal_example | other_reviewed
url_or_path: canonical source URL, reviewed local path, or opaque controlled handle
content_hash: algorithm-prefixed content hash, for example sha256:<digest>
review_status: quarantined | labeled_candidate | draft | review_requested | reviewed | accepted_limited | rejected | superseded
effective_confidence: high | medium | low | rejected | unknown
```

`content_hash` is required before a source can leave `quarantine_raw`.
`review_status` and `effective_confidence` are required before a source can
support `reviewed_wiki` or a future `seed_candidate`.

### Source Field Normalization Mapping

The current v0.2 seed source register can be adapted to the normalized source
record shape without changing seed JSON:

```yaml
path: url_or_path
sha256: content_hash
content_hash_format: sha256:<digest>
review_status: future normalized adapter or activation gate field only
effective_confidence: future normalized adapter or activation gate field only
```

`path -> url_or_path` is a compile-time normalization. A source-register path
may identify a reviewed offline provenance locator, but the absolute path value
must not enter runtime payloads, runtime logs, UI disclosure, telemetry, shadow
records, rollback records, or chat-visible summaries.

`sha256 -> content_hash` is also compile-time normalization. The normalized
value must be algorithm-prefixed as `sha256:<digest>` before it is used by a
future adapter, activation descriptor, or QA report.

`review_status` and `effective_confidence` are not retroactive v0.2 seed
requirements. They are future normalized-adapter or activation-gate fields for
deciding whether a source may support a future active manifest, snapshot, or
runtime selection surface. Adding those fields requires a separate approved
schema, adapter, or activation-gate implementation task.

### Raw Layer Record

Raw-layer records should carry:

```yaml
raw_layer: quarantine_raw | quarantine_labeled | seed_candidate
source_id: source identifier or pending opaque intake ID
source_type: controlled source category when known
url_or_path: canonical source locator or controlled handle
content_hash: algorithm-prefixed hash
review_status: raw quarantine or candidate review state
effective_confidence: review-derived confidence, unknown until reviewed
candidate_types:
  - prompt_sample
  - prompt_rule
  - prompt_failure_pattern
  - prompt_repair_strategy
risk_flags:
  - unreviewed
  - low_confidence
  - source_missing
  - possible_prompt_body
  - possible_secret
  - licensing_unknown
quarantine_reason: short controlled reason code
```

Raw records may point to controlled storage. They must not duplicate raw prompt
body text into governance summaries, chat, telemetry, or runtime payloads.

### Wiki Layer Record

Wiki-layer records should carry:

```yaml
wiki_layer: wiki_draft | reviewed_wiki
wiki_id: stable wiki identifier
source_ids:
  - source_id
wiki_title: compact human-readable title
wiki_type: prompt_sample | prompt_rule | prompt_failure_pattern | prompt_repair_strategy | source_policy_note
claims:
  - compact source-backed claim or reviewed team-distillation claim
open_questions:
  - unresolved source, schema, confidence, or application question
linked_schema_ids:
  - schema field, table, rule, failure mapping, repair mapping, or validation gate ID
review_status: draft | review_requested | reviewed | accepted_limited | rejected | superseded
effective_confidence: high | medium | low | rejected
```

`claims` must be concise, source-attributed, and reviewable. A draft may contain
open questions; a reviewed wiki entry must either resolve them, keep them
explicit, or mark the entry as limited.

## First Version Ingest Flow

1. Register source intent.
   Record `source_id`, `source_type`, `url_or_path`, initial intake owner,
   expected Prompt knowledge types, and whether the source is public,
   internal, team-distilled, or rejected-on-arrival.

2. Capture into `quarantine_raw`.
   Store only controlled raw material outside runtime paths. Record
   `content_hash`, `quarantine_reason`, rough source type, and risk flags. This
   state is never runtime-selectable.

3. Screen and label into `quarantine_labeled`.
   Confirm the source can map to Prompt governance fields. Add candidate types,
   source notes, review blockers, confidence draft, and linked schema targets.
   Remove or reject anything that appears to contain secrets, local paths, raw
   prompt bodies not allowed for retention, unsupported claims, or licensing
   uncertainty.

4. Create `wiki_draft`.
   A human or LLM may summarize labeled material into compact claims,
   `open_questions`, and `linked_schema_ids`. LLM output is assistance only.
   It cannot change `review_status` to reviewed, cannot create verified seed,
   cannot write snapshots, and cannot approve runtime eligibility.

5. Human review.
   Reviewers check source coverage, source type, claim support, confidence,
   schema links, failure/repair mapping fit, leakage risk, and whether the page
   should be accepted, accepted with limits, rejected, or kept as draft.

6. Promote to `reviewed_wiki`.
   Promotion requires explicit `review_status`, `effective_confidence`,
   source IDs, resolved or retained open questions, and no denied raw content.
   Reviewed wiki entries can inform QA and future seed proposals, but they
   still do not enter runtime selection by themselves.

7. Prepare `seed_candidate` only under a future gate.
   A reviewed wiki page may produce a seed-candidate packet with linked schema
   IDs, proposed table targets, acceptance evidence, and validation needs. It
   remains outside active seed bundles and snapshots until Git-reviewed seed
   changes, rebuild, and validation are explicitly approved.

8. Compile and QA.
   Compile-time tools may check duplicates, conflicts, missing source fields,
   confidence gaps, stale sources, unresolved open questions, and leakage
   boundaries. Runtime must continue using only the verified active snapshot
   and index selected by the approved activation descriptor.

## Runtime Selection Boundary

The following are hard rules:

- `quarantine_raw` must never enter runtime selection
- `quarantine_labeled` must never enter runtime selection
- `seed_candidate` must never enter runtime selection until a future approved
  seed/snapshot/index rebuild and activation verification
- `wiki_draft` must never enter runtime selection
- `reviewed_wiki` may inform future compile-time work, but it is not a runtime
  source until converted through an approved seed/snapshot gate
- runtime consumers must not auto-ingest user session content
- runtime consumers must not scan raw-layer or wiki-layer stores as fallback
- runtime consumers, logs, UI surfaces, payloads, telemetry, shadow records,
  rollback records, and chat-visible summaries must not disclose source
  register absolute paths; such paths are offline provenance locators only
- runtime consumers must not use an LLM to maintain, summarize, or promote the
  wiki by default
- LLM-assisted drafts can never auto-upgrade to verified seed

If the verified active snapshot or index fails, the only allowed runtime
outcomes remain the prior verified active snapshot, a verified
`last_known_good_snapshot`, or `no_kb_context`.

## Source Acceptance Checklist

A source is acceptable for Prompt knowledge governance only when:

- `source_id` is stable and unique
- `source_type` is controlled and not overstated
- `url_or_path` is canonical or represented by a controlled opaque handle
- `content_hash` is present and reproducible
- legacy `path` values have been normalized to `url_or_path` only for
  compile-time governance, and absolute path values remain offline provenance
  locators that are excluded from runtime/log/UI/payload disclosure
- legacy `sha256` values have been normalized to
  `content_hash = sha256:<digest>` before any future adapter, activation gate,
  or QA report relies on them
- `review_status` is explicit
- `effective_confidence` is derived from review, not copied from source claims
- source notes explain why the material is relevant to Prompt governance
- source material can map to at least one Prompt knowledge type
- unsupported, unverifiable, secret-bearing, path-bearing, or licensing-unclear
  content is rejected or kept in quarantine

## Quarantine Acceptance Checklist

A quarantine record is acceptable only when:

- it is in `quarantine_raw` or `quarantine_labeled`, never an active manifest,
  active snapshot, active index, eval truth set, runtime payload, telemetry
  event, or chat response
- it has a `content_hash` or opaque intake ID before detailed handling
- it has a controlled `quarantine_reason`
- it records risk flags without exposing matched raw values
- it records only bounded metadata in logs and review summaries
- it cannot be selected by BM25, fallback retrieval, shadow runs, or
  downstream prompt assembly
- promotion requires human review plus a later manifest/snapshot/index rebuild
  gate

## Wiki Acceptance Checklist

A wiki entry is acceptable for `reviewed_wiki` only when:

- `wiki_id`, `source_ids`, `wiki_title`, and `wiki_type` are present
- every material claim is source-backed or explicitly marked as team
  distillation with limited confidence
- `claims` are compact and do not embed raw prompt bodies or raw source text
- `open_questions` are resolved, retained as explicit limits, or tracked for a
  future review wave
- `linked_schema_ids` point to relevant schema fields, rules, failure patterns,
  repair strategies, validation gates, or source policy IDs
- `review_status` is `reviewed`, `accepted_limited`, `rejected`, or
  `superseded`, not merely draft
- `effective_confidence` is present and justified by review
- leakage-sensitive fields are absent from the wiki record and all emitted
  artifacts

## Compile And QA Checks

Future validators may compile this governance layer into machine-checkable
reports, but only under an approved implementation gate. The reports should
check:

- missing source minimum fields
- duplicate or conflicting `wiki_id` and `source_id` values
- unreviewed sources referenced by reviewed wiki pages
- reviewed wiki pages with unresolved high-risk `open_questions`
- low-confidence claims that are not marked as limited
- `seed_candidate` records without linked schema IDs
- quarantine records present in any active manifest, snapshot, index, eval, or
  runtime-selection surface
- denied field names or content classes in wiki, logs, telemetry, shadow, and
  rollback records

Acceptance result for leakage remains `leakage_count = 0`.

## Non-Goals

This contract does not authorize:

- changing `seed/v0.2/*.json`
- changing migrations
- changing snapshot SQLite files
- changing Hope main-thread runtime or desktop UI
- adding image generation or video generation behavior
- turning Hope desktop into a knowledge-management product
- runtime automatic ingest
- user-session writes to Git
- raw-layer or wiki-layer query-time maintenance
- raw prompt body retention in user-visible payloads
- full source-register disclosure
- API key, token, credential, or provider header disclosure

## Controller Questions

These decisions remain for a future control gate:

- whether `reviewed_wiki` should become a separate tracked artifact directory
  or remain a design contract until validator work opens
- whether `wiki_id` should follow a human-readable namespace or an opaque
  stable ID scheme
- whether `effective_confidence` should stay categorical or compile to a
  numeric threshold for future runtime activation descriptors
- which team role owns final promotion from `reviewed_wiki` to
  `seed_candidate`
