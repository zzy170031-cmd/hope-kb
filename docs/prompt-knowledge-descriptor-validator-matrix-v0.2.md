# Prompt Knowledge Descriptor Validator Matrix v0.2

## Status

Status: docs-only descriptor matrix.
Scope: first machine-checkable descriptor plan for `hope-kb` v0.2 Prompt
knowledge governance, freshness activation, router/FutureQA gates, and safety
lint records.
Thread: HopePrompt KB v0.2 total control.

This document does not implement validators. It does not modify seed JSON,
source registers, snapshots, migrations, runtime code, Hope main-thread code,
desktop UI, image generation, video generation, runtime GraphRAG, runtime
network fetch, runtime auto-ingest, hybrid search, rerank, or runtime LLM
summarization.

## Controller Decisions

- Future descriptor fixtures use JSON objects and JSON rule tables as the
  lowest-dependency machine-checkable shape.
- This matrix is the docs-only source of truth before implementation.
- The first validator implementation gate, when opened, should validate offline
  descriptor fixtures only. It must not read runtime artifacts, raw KB rows,
  source registers, snapshot SQLite files, or Hope runtime state.
- `descriptor_type`, `descriptor_version`, `descriptor_hash`, `artifact_class`,
  `schema_version`, and a timestamp bucket or equivalent review time bucket are
  required unless a descriptor-specific exception is explicitly listed here.
- Unknown fields are fail-closed in v0.2 descriptor fixtures.
- `artifact_class_allowlist` and `denied_fields` are centralized in this
  matrix. Lane-specific docs should not maintain separate incompatible lists.
- `source_ids` and `content_hashes` may appear in governance-only
  `SourceDeltaBatch` descriptors. They must not appear in runtime prompt
  payloads or activation descriptors as per-source locators.
- `last_known_good_snapshot` is a compatibility field name. Its canonical
  meaning is `last_known_good_activation_descriptor`, not a raw snapshot file
  path.
- `descriptor_id + descriptor_hash` is required for active and rollback pointer
  targets.
- `auto_switch_allowed` defaults to `false`. It may be `true` only when
  freshness is `fresh`, all gates pass, and controller approval is present.
- Stale query results may contain selected IDs only when those IDs come from a
  verified fallback path and a sanitized fallback reason is present. They must
  never select from a stale or mismatched index.

## Canonical Artifact Classes

| artifact_class | Visibility | Allowed role |
| --- | --- | --- |
| `prompt_payload` | model-visible | Summary-only runtime prompt context |
| `retrieval_trace_log_telemetry_shadow_rollback` | non-model-visible | Sanitized trace, local telemetry, shadow, purge, and rollback records |
| `activation_descriptor` | non-model-visible | Hashes, counters, statuses, logical descriptor pointers |
| `governance_descriptor` | governance-only | Source delta batches and controller review descriptors |
| `eval_descriptor` | governance-only | Eval artifacts and truth-set binding descriptors |
| `future_qa_descriptor` | governance-only | Future QA candidate review descriptors |

Unknown `artifact_class` values fail closed.

## Canonical Enums

### `freshness_status`

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

### `activation_status`

```text
candidate
validating
verified
activated
failed
rolled_back
superseded
```

### `stale_reason_code`

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

### `fallback_reason_code`

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

## Global Denied Fields

The first descriptor validator should recursively scan object fields, arrays,
generic containers, `metadata`, `debug`, `trace`, `context`, `attachments`,
`payload`, and serialized JSON strings.

Denied names and content classes:

```text
raw_kb_rows
raw rows
raw_prompt_body
prompt_body
full_prompt
story_input
accepted_ephemeral_context
selected_sample_excerpts
source_original_text
raw_source_text
source_register
full_source_register
source_register[].path
sources[].path
provenance_locator
path
url_or_path
local_path
absolute_path
overlay_json
overlay JSON
raw_graph
graph_neighborhood
provider_config
provider_headers
request_body
response_body
api_key
api_key_ref
secret
secret_ref
credential
credential_ref
token
env_var_name
matched_value
matched_value_snippet
```

On a denied match, the validator may report only:

```text
descriptor_type
descriptor_id
field_path
denied_class
rule_id
```

It must not record matched values.

## Descriptor Matrix

### SourceDeltaBatch

Artifact class: `governance_descriptor`

Purpose: governance-only source freshness batch. This descriptor may support
review and activation planning, but it must not become runtime prompt context
or runtime telemetry.

Required fields:

```text
descriptor_type=SourceDeltaBatch
descriptor_version
descriptor_hash
artifact_class=governance_descriptor
schema_version
source_delta_batch_hash
source_delta_count
source_freshness_digest
freshness_status
stale_reason_code
source_ids
content_hashes
previous_content_hashes
review_status_summary
effective_confidence_summary
ingested_at_bucket
```

Allowed fields:

```text
controller_review_id
reviewed_at_bucket
review_status_counts
confidence_bucket_counts
rejected_source_delta_count
accepted_source_delta_count
```

Must equal or match:

- `content_hashes[*]` and `previous_content_hashes[*]` use
  `sha256:<digest>`.
- `source_delta_count` equals the number of source deltas represented by the
  batch.
- `source_delta_batch_hash` and `source_freshness_digest` are aggregate
  digests, not serialized source-register content.

Reject if:

- Any hash is missing or malformed.
- `review_status_summary` or `effective_confidence_summary` is missing while
  activation is requested.
- Any source locator, path, URL/path alias, source-register path, raw source
  text, credential, or secret appears.
- The descriptor is copied into runtime payload, runtime logs, UI disclosure,
  telemetry, shadow, rollback, or chat-visible summaries.

Future validator name: `Validate-SourceDeltaBatchDescriptor`.

### ActivationDescriptor

Artifact class: `activation_descriptor`

Purpose: local non-model-visible descriptor binding a verified runtime KB
surface.

Required fields:

```text
descriptor_type=ActivationDescriptor
descriptor_version
descriptor_id
descriptor_hash
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
last_known_good_activation_descriptor
rollback_pointer
activation_verified_at_bucket
controller_approval_id
```

Must equal or match:

- `seed_bundle_hash` equals `seed/v0.2/manifest.json` `content_hash`.
- `manifest_hash` binds seed bundle, snapshot, index, record counts, and
  predicates.
- `snapshot_hash` matches the selected verified snapshot artifact digest.
- `index_hash` matches the verified runtime selection surface.
- `activation_status` is a canonical activation status.
- `freshness_status` is a canonical freshness status.
- `activation_status=activated` requires `freshness_status=fresh`.
- `source_delta_batch_hash` and `source_freshness_digest` are aggregate
  digests only.

Reject if:

- Any binding hash is missing or mismatched.
- Runtime predicates or exclusion predicates are missing.
- `active_pointer`, `last_known_good_snapshot`, or `rollback_pointer` points to
  a raw snapshot path, source path, seed file, full source register, or
  unverified descriptor.
- The descriptor contains raw KB rows, raw prompt bodies, full source register,
  overlay JSON, raw graph, local paths, provider config, credential refs, or
  secrets.
- Runtime prompt payload contains activation descriptor internals.

Future validator name: `Validate-ActivationDescriptor`.

### ActivePointer

Artifact class: `activation_descriptor`

Purpose: atomic logical pointer to the verified active activation descriptor.

Required fields:

```text
descriptor_type=ActivePointer
descriptor_version
descriptor_id
descriptor_hash
artifact_class=activation_descriptor
schema_version
pointer_type=active_pointer
pointer_id
target_activation_descriptor_id
target_activation_descriptor_hash
target_seed_bundle_hash
target_manifest_hash
target_snapshot_hash
target_index_hash
pointer_status=active
atomic_switch_txn_id
previous_pointer_id
switched_at_bucket
controller_approval_id
```

Must equal or match:

- Target descriptor has `verification_status=passed`.
- Target descriptor has `activation_status=verified` or `activated`.
- Target hash bindings match the target activation descriptor.
- Pointer switch updates descriptor, snapshot, index, and hash binding as one
  atomic logical transition.

Reject if:

- Pointer target is a raw snapshot path, source path, seed file, full source
  register, or unverified descriptor.
- Partial pointer update is represented as active.
- `previous_pointer_id` is missing.

Future validator name: `Validate-ActivePointerDescriptor`.

### LastKnownGoodDescriptor

Artifact class: `activation_descriptor`

Purpose: verified fallback descriptor for last-known-good runtime selection.

Required fields:

```text
descriptor_type=LastKnownGoodDescriptor
descriptor_version
descriptor_id
descriptor_hash
artifact_class=activation_descriptor
schema_version
lkg_descriptor_id
lkg_descriptor_hash
lkg_seed_bundle_hash
lkg_manifest_hash
lkg_snapshot_hash
lkg_index_hash
lkg_verified_at_bucket
lkg_activation_status=activated
lkg_freshness_status
lkg_reason_code
```

Must equal or match:

- `lkg_descriptor_id` and `lkg_descriptor_hash` identify a previously verified
  activation descriptor.
- Snapshot and index hashes bind to that descriptor.
- `lkg_freshness_status` is `fresh` or an accepted previous-good state.

Reject if:

- It references source paths, raw snapshot paths, seed file paths, or full
  source registers.
- It cannot bind both snapshot and index hash.

Future validator name: `Validate-LastKnownGoodDescriptor`.

### RollbackPointer

Artifact class: `retrieval_trace_log_telemetry_shadow_rollback`

Purpose: sanitized rollback pointer to a verified last-known-good activation
descriptor.

Required fields:

```text
descriptor_type=RollbackPointer
descriptor_version
descriptor_id
descriptor_hash
artifact_class=retrieval_trace_log_telemetry_shadow_rollback
schema_version
rollback_pointer_id
rollback_trigger_code
current_failed_descriptor_id
current_failed_descriptor_hash
target_lkg_descriptor_id
target_lkg_descriptor_hash
rollback_status
rolled_back_at_bucket
sanitized_incident_id
```

Allowed `rollback_status`:

```text
ready
executed
blocked
```

Reject if:

- Target LKG descriptor is not verified.
- Pointer resolves to a raw snapshot path, source path, seed file, source
  register dump, or unverified candidate descriptor.
- The record contains raw requests, prompts, source rows, provider payloads,
  credential material, matched snippets, or local paths.

Future validator name: `Validate-RollbackPointerDescriptor`.

### QueryResult

Artifact class: `prompt_payload`

Purpose: model-visible bounded result surface for governance or future runtime
query results.

Required fields:

```text
descriptor_type=QueryResult
descriptor_version
descriptor_hash
artifact_class=prompt_payload
schema_version
query_type
resolved_intent
intent_confidence
intent_routing_status
selected_sample_ids
selected_kb_rules
kb_context_summary
retrieval_trace_ref
fallback_reason_code
freshness_status
activation_status
snapshot_hash
index_hash
full_kb_rows_included=0
```

Required safety constants:

```text
raw_prompt_body_included=false
full_source_register_included=false
overlay_json_included=false
raw_graph_included=false
local_paths_included=false
secrets_included=false
media_generation_triggered=false
image_generation_triggered=false
video_generation_triggered=false
graphrag_used=false
hybrid_search_used=false
rerank_used=false
runtime_llm_summarize_used=false
expensive_path_used=false
```

Reject if:

- `query_type` is not `governance_query` or `runtime_query`.
- `full_kb_rows_included` is missing or not `0`.
- Any safety constant is `true`.
- Stale or unresolved query state selects samples from a stale or mismatched
  index.
- Prompt payload contains source deltas, source locators, raw rows, raw prompt
  bodies, full source registers, overlay JSON, raw graph, local paths, or
  secrets.

Verified fallback exception:

- Stale query results may include selected IDs only from a verified fallback
  path, with a sanitized `fallback_reason_code`. They must not use stale index
  selection.

Future validator name: `Validate-QueryResultDescriptor`.

### RetrievalTrace

Artifact class: `retrieval_trace_log_telemetry_shadow_rollback`

Purpose: non-model-visible sanitized trace for retrieval and fallback
observability.

Required fields:

```text
descriptor_type=RetrievalTrace
descriptor_version
descriptor_hash
artifact_class=retrieval_trace_log_telemetry_shadow_rollback
schema_version
trace_id
query_type
resolved_intent
candidate_pool_size
raw_bm25_score
max_in_pool
normalized_score
min_candidate_score=0.15
fallback_reason_code
freshness_status
activation_status
stale_reason_code
snapshot_hash
index_hash
selected_sample_ids
selected_kb_rules
payload_bytes
source_delta_count
full_kb_rows_included=0
expensive_path_used=false
```

Reject if:

- `artifact_class` is not
  `retrieval_trace_log_telemetry_shadow_rollback`.
- Trace is model-visible.
- `normalized_score` is outside `[0, 1]`.
- `on_index_miss` or `on_stale_index` does not block auto-switch.
- Stale status selects from a mismatched index.
- Trace includes source records, locators, prompt bodies, source text, graph
  neighborhoods, overlay JSON, local paths, provider config, credential refs,
  API key refs, tokens, or secrets.
- `full_kb_rows_included` is not `0` or `expensive_path_used=true`.

Future validator name: `Validate-RetrievalTraceDescriptor`.

### FutureQACandidate

Artifact class: `future_qa_descriptor`

Purpose: review candidate for future QA coverage. It is not eval truth.

Required fields:

```text
descriptor_type=FutureQACandidate
descriptor_version
descriptor_id
descriptor_hash
artifact_class=future_qa_descriptor
schema_version
candidate_id
source_query_type
source_intent
candidate_question
candidate_expected_behavior
candidate_failure_mode
selected_sample_ids
selected_kb_rules
kb_context_summary
retrieval_trace_ref
review_status
created_at_bucket
judged_against_model
judged_at_bucket
source_freshness_status
freshness_status
freshness_reason_code
stale_reason_code
stale_if_model_changes=true
stale_action=block_auto_switch
is_eval_truth=false
enters_eval_truth_by_default=false
requires_human_review=true
requires_git_promotion=true
requires_rebuild_and_validation=true
promoted_by_git=false
```

Reject if:

- `is_eval_truth=true`.
- `enters_eval_truth_by_default=true`.
- Future QA candidates are counted as eval truth.
- Review skips human/controller review, Git-visible promotion, rebuild, or
  validation.
- Stale or freshness failure is used to allow auto-switch.
- Raw KB rows, raw prompt bodies, full source registers, overlay JSON, raw
  graph, local paths, or secrets appear.

Promotion rule:

- `promoted_by_git` must not turn this descriptor into truth in place. It may
  only reference an external eval truth artifact after review, Git-visible
  promotion, rebuild, and validation.

Future validator name: `Validate-FutureQACandidateDescriptor`.

### EvalArtifact

Artifact class: `eval_descriptor`

Purpose: machine-checkable eval artifact binding truth-set, model, judge,
snapshot, index, and stale policy.

Required fields:

```text
descriptor_type=EvalArtifact
descriptor_version
descriptor_id
descriptor_hash
artifact_class=eval_descriptor
schema_version
eval_artifact_id
snapshot_hash
index_hash
intent_label_set_hash
eval_truth_set_hash
judged_against_model
judged_at_bucket
freshness_checked_at_bucket
freshness_status
stale_reason_code
stale_if_model_changes=true
stale_action=block_auto_switch
auto_switch_allowed
blocked_reason_codes
intent_routing_accuracy
pass_rate_overall
pass_rate_per_intent
tail_failure_rate
max_tail_failure_rate_absolute
min_samples_per_intent_in_eval_set=20
max_per_intent_drop_pp=0.05
future_qa_candidates_counted_as_truth=false
```

Must equal or match:

- `auto_switch_allowed=false` when stale.
- `auto_switch_allowed=true` only when freshness is `fresh`, all gates pass,
  and controller approval exists.

Reject if:

- Per-intent sample floor is missing.
- `max_tail_failure_rate_absolute` is missing.
- `pass_rate_overall` alone is used to approve.
- Snapshot hash, index hash, eval truth hash, intent set, model, judge,
  threshold, fallback rule, or approval policy changed without stale marking.
- Stale eval has `auto_switch_allowed=true`.
- Future QA candidates are counted as truth.
- Media generation, GraphRAG, hybrid search, rerank, runtime LLM summarize, or
  expensive path evidence is used for v0.2 eval approval.

Future validator name: `Validate-EvalArtifactDescriptor`.

### RefreshTelemetryRecord

Artifact class: `retrieval_trace_log_telemetry_shadow_rollback`

Purpose: local sanitized aggregate telemetry for refresh and activation
observability.

Required fields:

```text
descriptor_type=RefreshTelemetryRecord
descriptor_version
descriptor_id
descriptor_hash
artifact_class=retrieval_trace_log_telemetry_shadow_rollback
schema_version
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
leakage_count=0
timestamp_bucket
```

Allowed structured fields:

```text
purge_status
bytes_remaining
raw_event_backups_remaining
reconstructable_sensitive_content_remaining
rollback_trigger_code
sanitized_incident_id
remediation_state
```

Reject if:

- `artifact_class` is missing or not
  `retrieval_trace_log_telemetry_shadow_rollback`.
- Model-visible text appears.
- `leakage_count` is not `0`.
- Source deltas appear as per-source locators rather than count or digest
  aggregates.
- Any global denied field appears.

Future validator name: `Validate-RefreshTelemetryRecordDescriptor`.

### PurgeDescriptor

Artifact class: `retrieval_trace_log_telemetry_shadow_rollback`

Purpose: prove local aggregate purge completed with zero sensitive residue.

Required fields:

```text
descriptor_type=PurgeDescriptor
descriptor_version
descriptor_id
descriptor_hash
artifact_class=retrieval_trace_log_telemetry_shadow_rollback
schema_version
local_telemetry_only=true
purge_required=true
purge_scope
purge_action
purge_status=passed
files_removed_or_empty=true
bytes_remaining=0
raw_event_backups_remaining=0
reconstructable_sensitive_content_remaining=0
timestamp_bucket
```

Allowed `purge_scope`:

```text
local_aggregate_files
local_shadow_aggregate_files
local_rollback_aggregate_files
sidecar_backup_files
```

Allowed `purge_action`:

```text
remove_file
truncate_to_empty
```

Reject if:

- Any path, URL/path alias, local path, absolute path, raw event, prompt body,
  request body, response body, provider config, API key ref, credential ref,
  secret ref, env var name, non-empty raw backup, or reconstructable sensitive
  content appears.
- Any zero-residue field is non-zero or false.

Future validator name: `Validate-PurgeDescriptor`.

### RollbackDescriptor

Artifact class: `retrieval_trace_log_telemetry_shadow_rollback`

Purpose: sanitized proof that unsafe candidate retrieval/runtime paths were
disabled and previous deterministic or fallback path remains active.

Required fields:

```text
descriptor_type=RollbackDescriptor
descriptor_version
descriptor_id
descriptor_hash
artifact_class=retrieval_trace_log_telemetry_shadow_rollback
schema_version
rollback_trigger
catastrophic_trigger
rollback_trigger_code
sanitized_incident_id
timestamp_bucket
disabled_path_name
previous_path_name
remediation_state
rollback_pointer_id
target_lkg_descriptor_id
target_lkg_descriptor_hash
```

Required actions:

```text
stop_emitting_candidate_payload
disable_candidate_retrieval_runtime_path
keep_previous_deterministic_or_fallback_path_active
require_controller_review_before_reenable
```

Reject if:

- `rollback_pointer` target is not a verified last-known-good activation
  descriptor identity and hash.
- It points to raw snapshot path, source path, seed file, source register dump,
  or unverified candidate descriptor.
- Prompt payload, prompt body, full prompt, story input, accepted ephemeral
  context, source original text, raw KB rows, raw graph, source register, path
  aliases, request body, response body, provider headers, provider config,
  tokens, credentials, secrets, env var names, or matched snippets appear.

Future validator name: `Validate-RollbackDescriptor`.

## Cross-Descriptor Rules

### Auto-Switch

`auto_switch_allowed` defaults to `false`.

It may be `true` only when:

- `freshness_status=fresh`
- activation descriptor is verified or activated
- eval artifact is fresh
- source delta batch is reviewed
- leakage count is zero
- all hash bindings match
- controller approval is present

Any stale source, stale index, stale eval, stale snapshot, activation failure,
unknown freshness, or blocked status forces:

```text
auto_switch_allowed=false
stale_action=block_auto_switch
```

### Full KB Row Guard

`full_kb_rows_included=0` is required in:

- `QueryResult`
- `RetrievalTrace`
- runtime `prompt_payload`

If the field appears in non-model-visible records, it must also be `0`.

Reject if:

- field is missing where required
- field is not `0`
- raw KB rows, raw rows, or full source dumps appear under another field

This check must be structural, not a string-only scan.

### Denied Field Scan

Every descriptor must pass recursive denied-field scanning before descriptor
specific validation is accepted.

Any structural leak is catastrophic. It must reject the descriptor and cannot
be downgraded to a warning.

### Source Delta Visibility

`SourceDeltaBatch` may include `source_ids` and content hashes for governance
review. Activation and runtime-facing descriptors may include only:

```text
source_delta_batch_hash
source_delta_count
source_freshness_digest
```

They must not include source locators, paths, full source-register entries, or
raw source text.

## Suggested Implementation Order

The implementation gate remains closed until total control opens it.

When code implementation is explicitly opened, use Rust first for descriptor
validators and durable governance logic. PowerShell may remain only as a thin
Windows orchestration wrapper, compatibility layer for existing scripts, or an
explicit total-control exception.

When opened, the minimum first validator should:

1. Load JSON descriptor fixtures from an explicitly named offline fixture
   directory.
2. Reject unknown `descriptor_type` and unknown `artifact_class`.
3. Apply the global recursive denied-field scan.
4. Validate required fields and enum values.
5. Validate fixed constants such as `full_kb_rows_included=0`,
   `leakage_count=0`, and `expensive_path_used=false`.
6. Validate pointer targets use `descriptor_id + descriptor_hash` and do not
   use raw paths.
7. Validate stale statuses force `auto_switch_allowed=false`.
8. Validate purge zero-residue fields.
9. Print sanitized failure paths, rule IDs, and denied classes only.

The first validator must not:

- read runtime artifacts
- read snapshot SQLite files
- read raw KB rows
- read full source registers
- call a network
- call a model
- generate images or video
- modify seed, snapshot, migration, runtime, or Hope files
