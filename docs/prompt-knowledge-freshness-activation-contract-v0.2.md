# Prompt Knowledge Freshness Activation Contract v0.2

## Status

Status: docs-only contract.
Scope: `hope-kb` v0.2 freshness, verified activation, atomic snapshot switch,
last-known-good fallback, stale blocking, and sanitized refresh observability.
Thread: HopePrompt KB v0.2 total control.

This document does not modify seed JSON, source registers, snapshot files,
validators, migrations, Hope runtime code, desktop UI, image generation, video
generation, runtime GraphRAG, runtime network fetch, runtime auto-ingest,
hybrid search, rerank, or runtime LLM summarization.

The project remains a Hope support surface. It is not an independent product.
Hope runtime may consume only a verified activated snapshot and verified
runtime selection index.

## Purpose

The goal is timely data refresh without weakening the v0.2 QA boundary. Fresh
source material can enter governance quickly, but it must not become runtime
context until it passes review, validation, leakage gates, eval freshness
checks, and activation binding.

The safe update path is:

```text
source_delta
  -> governance review
  -> candidate schema/snapshot/index build
  -> full validation and leakage gates
  -> verified activation descriptor
  -> atomic active pointer switch
  -> runtime reads activated snapshot/index only
```

If any step fails, runtime remains on the current verified active snapshot. If
that is unavailable, runtime may use the verified last-known-good descriptor.
If neither is available, runtime must return deterministic `no_kb_context`.

## Non-Goals

- Do not perform runtime network fetch.
- Do not perform runtime auto-ingest.
- Do not query raw, wiki, full schema, seed JSON, source files, source register
  dumps, quarantine stores, or local provenance paths at request time.
- Do not use runtime LLM summarization as a default freshness path.
- Do not expose raw KB rows, raw `prompt_body`, full `source_register`, overlay
  JSON, raw graph payloads, local paths, source raw text, provider config,
  provider headers, API keys, tokens, secrets, or credential references.
- Do not use image or video generation as part of freshness, query, QA, or
  activation.
- Do not make GraphRAG, hybrid search, rerank, query rewriting, vector
  expansion, or live model expansion the v0.2 default path.
- Do not promote `FutureQACandidate` records into eval truth without human or
  controller review, Git-visible promotion, rebuild, and validation.

## Canonical Status Enums

### `freshness_status`

Use this enum everywhere freshness is represented:

```text
fresh
stale_source
stale_index
stale_eval
stale_snapshot
activation_failed
unknown
blocked
```

Meanings:

- `fresh`: source digest, snapshot, index, eval truth, and activation binding
  are all current for the active descriptor.
- `stale_source`: a source delta exists or source eligibility changed before a
  verified rebuilt snapshot is active.
- `stale_index`: runtime selection index no longer matches the selected
  snapshot or selection predicates.
- `stale_eval`: eval artifacts no longer match snapshot, index, intent set,
  model, judge, threshold, or truth-set hashes.
- `stale_snapshot`: selected snapshot is not verified active or no longer
  matches activation hashes.
- `activation_failed`: candidate activation failed validation, leakage,
  integrity, eval, approval, or atomic switch checks.
- `unknown`: freshness cannot be proven from the descriptor.
- `blocked`: freshness is explicitly blocked by controller policy or a hard
  gate.

### `activation_status`

Use this enum for activation descriptors:

```text
candidate
validating
verified
activated
failed
rolled_back
superseded
```

`no_kb_context` is not an `activation_status`. It is a deterministic
`fallback_reason_code` used when no verified active or last-known-good context
can be used.

### `stale_reason_code`

Canonical stale reason codes:

```text
source_delta_pending_review
source_delta_rejected
snapshot_rebuild_required
index_hash_mismatch
index_predicate_mismatch
eval_truth_stale
eval_model_or_judge_stale
snapshot_hash_mismatch
activation_descriptor_invalid
activation_policy_blocked
unknown_freshness
```

### Runtime fallback reason codes

Canonical runtime fallback reason codes:

```text
on_empty_pool
on_low_score
on_index_miss
on_stale_index
on_stale_snapshot
on_eval_stale
on_activation_failed
no_kb_context
```

`on_index_miss` means the verified index cannot be found or opened.
`on_stale_index` means an index exists but does not match the verified
activation boundary.

## Source Delta and Freshness Metadata

`source_delta` exists only in governance and activation planning. It is not
runtime prompt context.

Allowed `source_delta` fields:

```text
source_id
source_type
source_updated_at
ingested_at
content_hash
previous_content_hash
review_status
effective_confidence
provenance_locator
freshness_status
stale_reason_code
```

Rules:

- `content_hash` uses the normalized `sha256:<digest>` shape.
- `source_updated_at` and `ingested_at` are governance timestamps. Runtime
  observability may expose only timestamp buckets.
- `review_status` and `effective_confidence` are future normalized adapter and
  activation-gate fields. They do not retroactively rewrite the current v0.2
  seed files.
- `provenance_locator`, `url_or_path`, `path`, `source_register[].path`, and
  local absolute paths are offline locators only.
- Offline locators must not enter runtime payload, runtime logs, UI disclosure,
  telemetry, shadow records, rollback records, or chat-visible summaries.

`source_delta` may contribute only aggregate activation bindings:

```text
source_delta_batch_hash
source_delta_count
source_freshness_digest
```

The activation descriptor must never include per-source locators, full
source-register entries, raw source text, or source-register path values.

## Activation Descriptor

`activation_descriptor` is the local, non-model-visible artifact that binds the
verified runtime KB surface. It is not a provenance dump, source-register
export, prompt debug envelope, raw graph export, or prompt payload.

Minimum fields:

```text
artifact_class=activation_descriptor
schema_version
seed_bundle_hash
manifest_hash
snapshot_hash
index_hash
snapshot_version
snapshot_name
record_counts
runtime_selection_predicates
quarantine_exclusion_predicates
candidate_exclusion_predicates
source_delta_batch_hash
source_delta_count
source_freshness_digest
activation_status
freshness_status
verification_status
active_pointer
last_known_good_snapshot
rollback_pointer
activation_verified_at_bucket
controller_approval_id
```

Hash meanings:

- `seed_bundle_hash`: `seed/v0.2/manifest.json` `content_hash`.
- `manifest_hash`: digest of the activation manifest descriptor.
- `snapshot_hash`: digest of the selected verified snapshot artifact.
- `index_hash`: digest of the verified runtime selection surface.
- `source_delta_batch_hash`: digest of the reviewed source-delta batch that led
  to this activation, or an empty value when no source delta participated.
- `source_freshness_digest`: compact digest over source freshness state, not a
  source-register dump.

Pointer rules:

- `active_pointer` points to a verified activation descriptor identity and
  descriptor hash. It must not point to a raw snapshot path.
- `last_known_good_snapshot` points to a previously verified activation
  descriptor identity and descriptor hash.
- `rollback_pointer` may point only to a verified last-known-good descriptor.
  It must not point to raw snapshot paths, source paths, seed files, full
  source registers, or unverified candidate descriptors.

Denied in `activation_descriptor`:

- raw KB rows
- raw graph objects
- raw `prompt_body`
- full `source_register`
- source-register entries
- source locators or paths
- raw source text
- raw wiki text
- overlay JSON
- quarantine raw text
- provider config
- provider headers
- API keys, tokens, secrets, credential values, credential references, or env
  var names
- prompt payloads, request bodies, response bodies, or debug prompt envelopes

## Atomic Active Pointer Switch

The activation switch must be atomic from the runtime-consumer perspective.

Required sequence:

1. Prepare a candidate activation descriptor with `activation_status=candidate`.
2. Mark `activation_status=validating` while all integrity, schema, leakage,
   eval, freshness, and approval checks run.
3. Verify `seed_bundle_hash`, `manifest_hash`, `snapshot_hash`, `index_hash`,
   `source_delta_batch_hash`, and `source_freshness_digest`.
4. Verify quarantine and candidate exclusion predicates.
5. Verify runtime selection predicates and record counts.
6. Verify no raw rows, raw prompt bodies, full source registers, overlay JSON,
   raw graph objects, local paths, or secrets appear in runtime-exposed
   artifacts.
7. Verify eval freshness and router gates. Any stale eval uses
   `stale_action=block_auto_switch`.
8. After all gates pass, set `activation_status=verified`.
9. Atomically switch `active_pointer` to the verified descriptor.
10. Set `activation_status=activated` and `freshness_status=fresh`.
11. Preserve `last_known_good_snapshot` and `rollback_pointer`.
12. Emit only sanitized observability records.

Failure behavior:

- Candidate failure sets `activation_status=failed` and
  `freshness_status=activation_failed` or `blocked`.
- Runtime remains on the current verified active descriptor.
- If the current descriptor cannot be used, runtime may use the verified
  `last_known_good_snapshot`.
- If neither verified descriptor is usable, runtime returns deterministic
  `no_kb_context`.
- Runtime must not fall back to raw, wiki, seed JSON, source register, local
  files, runtime network fetch, runtime auto-ingest, or runtime LLM
  summarization.

## Runtime Consumption Boundary

Runtime may read only:

```text
verified activation_descriptor
verified activated snapshot
verified runtime selection index
```

Runtime prompt payload may include only:

```text
resolved_intent
selected_sample_ids
selected_kb_rules
selected_object_ids
kb_context_summary
fallback_reason_code
full_kb_rows_included=0
```

Runtime prompt payload must not include:

- `source_delta`
- `source_delta_batch_hash`
- `source_freshness_digest`
- `source_updated_at`
- `ingested_at`
- `provenance_locator`
- source paths
- source register entries
- activation descriptor internals
- raw rows
- raw graph objects
- raw `prompt_body`
- raw source text
- overlay JSON
- local paths
- secrets or credential references

## Stale Blocking Rules

### `stale_source`

`stale_source` exists at governance or activation time. It must not trigger
runtime source lookup.

Rules:

- Candidate activation is blocked until the source delta is reviewed and a new
  verified snapshot/index is built.
- Runtime continues to use current verified active, verified
  last-known-good, or deterministic `no_kb_context`.
- Prompt payload must not include stale-source details.
- Retrieval trace may include only sanitized `freshness_status` and
  `stale_reason_code`.

### `stale_index`

`stale_index` blocks auto-switch and sample selection from that index.

Rules:

- `auto_switch_allowed=false`.
- Use `fallback_reason_code=on_stale_index` when an index exists but no longer
  matches the activation boundary.
- Use `fallback_reason_code=on_index_miss` when the verified index is missing
  or unavailable.
- `selected_sample_ids` must be empty or derive from a verified fallback path.
- Do not guess with an old or mismatched index.

### `stale_eval`

`stale_eval` keeps the existing router eval hardening behavior.

Rules:

- `stale_action=block_auto_switch`.
- `auto_switch_allowed=false`.
- `FutureQACandidate` may record the stale condition as a candidate issue, but
  it remains `is_eval_truth=false`.
- Do not promote candidates into eval truth without review, Git-visible
  promotion, rebuild, and validation.

### `stale_snapshot`

`stale_snapshot` means the selected snapshot is not verified active for the
current descriptor.

Rules:

- Runtime must not read raw, wiki, schema, seed JSON, source register, or
  source files as fallback.
- Runtime may use current verified active, verified last-known-good, or
  deterministic `no_kb_context`.
- `fallback_reason_code=on_stale_snapshot` is allowed in sanitized traces.

## Query, Trace, Future QA, and Eval Extensions

### `QueryResult`

Allowed freshness fields:

```text
freshness_status
activation_status
fallback_reason_code
snapshot_hash
index_hash
selected_sample_ids
selected_kb_rules
selected_object_ids
kb_context_summary
full_kb_rows_included=0
expensive_path_used=false
media_generation_triggered=false
image_generation_triggered=false
video_generation_triggered=false
graphrag_used=false
hybrid_search_used=false
rerank_used=false
runtime_llm_summarize_used=false
```

Denied:

- source deltas
- source locators
- raw rows
- raw `prompt_body`
- full source register
- overlay JSON
- raw graph
- local paths
- secrets or credential references

### `RetrievalTrace`

`RetrievalTrace` belongs to
`retrieval_trace_log_telemetry_shadow_rollback`. It is non-model-visible.

Allowed freshness fields:

```text
freshness_status
activation_status
stale_reason_code
fallback_reason_code
snapshot_hash
index_hash
source_delta_count
selected_sample_ids
selected_kb_rules
candidate_pool_size
normalized_score
payload_bytes
full_kb_rows_included=0
```

It must not include source locators, full source-register entries, raw source
text, raw KB rows, raw graph objects, raw prompt bodies, overlay JSON, local
paths, provider configs, request bodies, response bodies, secrets, or credential
references.

### `FutureQACandidate`

Freshness fields:

```text
source_freshness_status
freshness_status
freshness_reason_code
stale_reason_code
candidate_source=router_or_activation_observation
is_eval_truth=false
```

Rules:

- A stale or freshness failure can create a candidate question or review item.
- It does not become eval truth.
- It does not override per-intent eval gates.
- It does not allow auto-switch.

### `EvalArtifact`

Required freshness and binding fields:

```text
snapshot_hash
index_hash
eval_truth_set_hash
judged_against_model
judged_at_bucket
freshness_checked_at_bucket
freshness_status
stale_reason_code
stale_action=block_auto_switch
auto_switch_allowed=false when stale
```

Any change to snapshot hash, index hash, eval truth set hash, intent coverage,
fallback rules, threshold, model, judge, or approval policy makes the eval
artifact stale until regenerated and reviewed.

## Refresh Telemetry, Purge, Shadow, and Rollback

All refresh telemetry, purge records, shadow records, and rollback records
belong to `retrieval_trace_log_telemetry_shadow_rollback`.

Allowed fields:

```text
snapshot_hash
index_hash
activation_status
freshness_status
stale_reason_code
fallback_reason_code
source_delta_count
payload_bytes
selected_sample_ids
selected_kb_rules
lint_rule_ids
leakage_count
purge_status
bytes_remaining
raw_event_backups_remaining
reconstructable_sensitive_content_remaining
rollback_trigger_code
sanitized_incident_id
timestamp_bucket
remediation_state
```

Denied fields:

- raw KB rows
- raw `prompt_body`
- full `source_register`
- source-register paths
- `path`
- `url_or_path`
- provenance locators
- overlay JSON
- raw graph
- source raw text
- local paths
- provider config
- provider headers
- request body
- response body
- credential values
- credential references
- `api_key_ref`
- `credential_ref`
- `secret_ref`
- `env_var_name`
- matched-value snippets

Purge acceptance:

```text
purge_status=passed
bytes_remaining=0
raw_event_backups_remaining=0
reconstructable_sensitive_content_remaining=0
```

Rollback records may contain only trigger code, sanitized incident ID,
timestamp bucket, disabled logical path, previous logical path, and remediation
state. They must not contain original requests, prompts, source rows, raw
provider payloads, credential material, matched snippets, or local paths.

## Acceptance Checklist

A freshness activation package is acceptable only if all of the following are
true:

- Source deltas stay in governance.
- Activation descriptor contains hashes, counters, status, and logical
  pointers only.
- `source_delta_batch_hash` and `source_freshness_digest` are aggregate
  digests, not provenance dumps.
- `rollback_pointer` points only to a verified last-known-good descriptor.
- Runtime reads only verified activation descriptor, activated snapshot, and
  verified selection index.
- `full_kb_rows_included=0` is preserved for prompt payload, QueryResult, and
  RetrievalTrace.
- Stale source, index, eval, or snapshot blocks auto-switch.
- Future QA candidates remain non-truth until review and Git-visible promotion.
- Refresh telemetry, shadow, purge, and rollback records are sanitized
  observability only.
- No image generation, video generation, GraphRAG default, hybrid default,
  rerank default, runtime network fetch, runtime auto-ingest, or runtime LLM
  summarize is introduced.

## Future Machine-Checkable Gate Candidates

Future implementation gates may define machine-checkable descriptors for:

- `SourceDeltaBatch`
- `ActivationDescriptor`
- `ActivePointer`
- `QueryResult`
- `RetrievalTrace`
- `FutureQACandidate`
- `EvalArtifact`
- `RefreshTelemetryRecord`
- `PurgeDescriptor`
- `RollbackDescriptor`

Those implementation gates remain closed until total control explicitly opens
them.
