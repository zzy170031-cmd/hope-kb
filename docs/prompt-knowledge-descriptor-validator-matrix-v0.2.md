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

## Cross-Descriptor Review Decisions 2026-04-26

- The next Rust implementation slice remains offline JSON fixture validation.
  It still must not read runtime artifacts, snapshot SQLite files, raw KB rows,
  full source registers, or Hope runtime state.
- Implement static descriptor rules before collection-level binding rules:
  activation descriptors, active pointers, last-known-good descriptors,
  rollback pointers, runtime flags, auto-switch rules, eval artifacts, query
  results, retrieval traces, and FutureQA candidates.
- `descriptor_hash` is the logical descriptor identity hash. The target
  definition is a canonical JSON descriptor digest that excludes the
  `descriptor_hash` field itself. The immediate validator may check hash shape
  and cross-descriptor literal consistency; recomputing canonical digests is a
  later gate.
- The immediate static hash shape accepts `sha256:<token>` and
  `bundle-sha256:<token>` with a non-empty fixture token made from
  alphanumeric characters, `.`, `_`, or `-`, and still rejects raw paths,
  source locators, seed paths, source-register names, and snapshot SQLite
  paths. Strict 64-hex digest enforcement is deferred to the canonical digest
  gate.
- `manifest_hash` canonical recomputation is deferred. The immediate validator
  checks presence, hash shape, no path/locator leakage, and literal binding
  consistency across fixture descriptors.
- `on_index_miss`, `on_stale_index`, and `on_stale_snapshot` use strict mode in
  the first validator: `selected_sample_ids` and `selected_kb_rules` must be
  empty. A verified fallback exception is deferred until fallback descriptor
  fields are standardized.
- If a verified fallback exception is opened later, the minimum proof fields are
  `fallback_descriptor_id`, `fallback_descriptor_hash`,
  `fallback_snapshot_hash`, and `fallback_index_hash`; those fields still must
  not expose raw paths or source locators.
- `auto_switch_allowed=true` requires `freshness_status=fresh`,
  `all_gates_pass=true`, a non-empty `controller_approval_id`, and all
  runtime/media/expensive path flags set to `false`. Any stale, blocked,
  unknown, activation-failed, or missing gate state forces
  `auto_switch_allowed=false`.
- `controller_approval_id` is the required approval marker for v0.2. A separate
  `approval_status=approved` field may be added later, but it is not required
  for the first implementation.
- `FutureQACandidate` must never mutate itself into eval truth. If promotion is
  represented later, it must reference an external eval truth artifact via
  `promoted_eval_artifact_id` and `promoted_eval_artifact_hash`, while keeping
  `is_eval_truth=false` and `enters_eval_truth_by_default=false`.
- `LastKnownGoodDescriptor` first implementation accepts only
  `lkg_activation_status=activated` and `lkg_freshness_status=fresh`. Accepted
  previous-good states such as `superseded` or `rolled_back` are deferred until
  an explicit historical-activation descriptor contract exists.

## Descriptor Fixture Gate Decisions 2026-04-26

- The next gate adds canonical JSON fixtures before cross fixture binding. It
  covers the implemented static validator rules only.
- Fixtures live under `tools/descriptor-validator/fixtures/` and are split by
  expected result and lane-owned topic:

```text
tools/descriptor-validator/fixtures/pass/lane2_activation_pointer/
tools/descriptor-validator/fixtures/fail/lane2_activation_pointer/
tools/descriptor-validator/fixtures/pass/lane3_router_eval_futureqa/
tools/descriptor-validator/fixtures/fail/lane3_router_eval_futureqa/
tools/descriptor-validator/fixtures/pass/lane4_safety_observability/
tools/descriptor-validator/fixtures/fail/lane4_safety_observability/
```

- Lane 1 source-delta fixtures stay deferred until `SourceDeltaBatch` static
  validator rules are opened.
- The current CLI validates JSON files in one provided fixture directory. Until
  a recursive fixture runner is opened, each leaf directory is validated
  independently with `descriptor-validator <fixture-dir>`.
- Passing fixture directories must produce `status=passed`. Failing fixture
  directories must produce `status=failed` and sanitized diagnostics only.
- Fixture files must be UTF-8 JSON. Each file may contain one descriptor object
  or an array of descriptor objects.
- Failing fixtures must use synthetic placeholder values only. Do not put real
  raw KB rows, raw prompt bodies, source-register dumps, overlay JSON, raw
  graph payloads, local absolute paths, API keys, tokens, provider headers, or
  matched snippets into fixtures. Use safe sentinel strings that trigger field
  name or shape checks without exposing sensitive content.
- Fixture work may add JSON files only. It must not change Rust rules, seed
  JSON, snapshots, runtime code, migrations, or Hope main-thread code unless
  total control opens a separate gate.

## Cross-Descriptor Fixture-Set Binding Gate Decisions 2026-04-26

- This gate opens after `f1a63d3`. It implements offline fixture-set binding
  only. It must not read runtime artifacts, snapshot SQLite files, raw KB rows,
  full source registers, source raw text, Hope runtime state, or network data.
- The first implementation target is a descriptor-set index over explicitly
  supplied JSON fixtures. The index may recurse inside a named fixture-set
  directory, but it still reads only `.json` descriptor fixtures.
- Descriptor identity for this gate is literal `descriptor_id +
  descriptor_hash`. Canonical JSON digest recomputation remains deferred.
- `manifest_hash`, `snapshot_hash`, `index_hash`, `seed_bundle_hash`,
  `source_delta_batch_hash`, and `source_freshness_digest` are checked for
  literal consistency across related descriptors. They are not recomputed in
  this gate.
- `ActivePointer.target_activation_descriptor_id +
  target_activation_descriptor_hash` must resolve to an `ActivationDescriptor`
  in the same fixture set. The target must have `verification_status=passed`
  and `activation_status=verified` or `activation_status=activated`.
- Active pointer target hashes for seed bundle, manifest, snapshot, and index
  must exactly match the referenced `ActivationDescriptor`.
- `LastKnownGoodDescriptor.lkg_descriptor_id + lkg_descriptor_hash` must
  resolve to a verified last-known-good `ActivationDescriptor` in the same
  fixture set. The first implementation allows only `activation_status=activated`
  and `freshness_status=fresh`.
- `RollbackPointer.target_lkg_descriptor_id + target_lkg_descriptor_hash` must
  resolve to either a `LastKnownGoodDescriptor` or the activation descriptor it
  binds to. `RollbackPointer.current_failed_descriptor_id +
  current_failed_descriptor_hash` must resolve to a failed activation candidate
  in the same fixture set.
- Cross-binding diagnostics remain sanitized: report descriptor type, descriptor
  id, field path, denied class, and rule id only. Do not emit matched values,
  raw paths, raw source text, source registers, prompt bodies, or secrets.
- Verified fallback exceptions for `QueryResult` and `RetrievalTrace` remain
  closed. `on_index_miss`, `on_stale_index`, and `on_stale_snapshot` still
  require empty selected IDs/rules in this gate.
- SourceDeltaBatch implementation remains deferred. Lane 1 may review whether
  future source-delta descriptors should participate in cross binding, but no
  source-delta Rust rules or fixtures are opened here.

## Duplicate Descriptor Identity Gate Decisions 2026-04-26

- This gate opens after Lane 3 and Lane 4 read-only review of
  `cross_descriptor_binding`. It remains an offline JSON fixture validator gate.
- The validator must fail closed when the same literal `descriptor_id +
  descriptor_hash` identity appears more than once in a supplied fixture set.
- Duplicate detection applies across all descriptor types and all JSON files
  loaded from the explicitly supplied fixture directory.
- Duplicate diagnostics must remain sanitized and structural. They may report
  descriptor type, descriptor id, field path, denied class, and rule id only.
  They must not print the duplicate descriptor hash, matched values, file paths,
  raw source text, raw prompt bodies, source-register rows, provider material,
  or secrets.
- The initial implementation may keep the existing non-recursive fixture
  directory reader. Recursive fixture-set runner remains a later gate.
- Canonical digest recomputation remains closed. Duplicates are detected by the
  literal identity already present in the fixture descriptors.
- SourceDeltaBatch implementation and verified fallback exceptions remain
  closed.

Duplicate gate implementation ownership:

```text
Lane 2: implement duplicate identity detection in DescriptorSet or the nearest
        fixture-set indexing module, plus one pass-preserving and one failing
        duplicate fixture.
Lane 4: optional read-only review if diagnostics shape changes.
Lane 3: standby unless router/eval fixtures are unexpectedly affected.
Lane 1: standby.
```

## Recursive Fixture Matrix Runner Gate Decisions 2026-04-26

- This gate opens after duplicate descriptor identity implementation and Lane 4
  safety review. It remains an offline JSON fixture validator gate.
- The goal is operational ergonomics only: run all known fixture leaf
  directories in one command and verify expected pass/fail outcomes.
- The runner must read only `.json` descriptor fixtures under the explicitly
  supplied fixture root. It must not read runtime artifacts, snapshot SQLite
  files, raw KB rows, full source registers, source raw text, Hope runtime
  state, or network data.
- The first implementation should support the existing fixture layout:

```text
tools/descriptor-validator/fixtures/pass/<leaf>/
tools/descriptor-validator/fixtures/fail/<leaf>/
```

- Every `pass` leaf must produce `status=passed` and exit successfully. Every
  `fail` leaf must produce `status=failed`; expected failing leaves should not
  make the matrix runner itself fail.
- The matrix runner must fail if a pass leaf fails, a fail leaf passes, a leaf
  cannot be read, a leaf has invalid JSON, or an unknown expectation directory
  is encountered.
- The matrix summary may report fixture root, expectation, leaf name,
  descriptors checked, status, and sanitized diagnostics count. It must not
  print matched values, descriptor hashes, raw paths, source text, prompt
  bodies, source-register rows, provider material, or secrets.
- This gate must not change descriptor validation semantics. It only adds a
  runner over existing per-leaf validation.
- Canonical digest recomputation, SourceDeltaBatch implementation, verified
  fallback exceptions, and runtime/snapshot activation remain closed.

Recursive runner implementation ownership:

```text
Lane 4: primary Rust implementation for the matrix runner and safety-preserving
        summary output.
Lane 2: standby unless fixture-set indexing needs a small adapter.
Lane 3: standby unless router/eval fixture expectations are affected.
Lane 1: standby.
```

Cross binding gate implementation ownership:

```text
Lane 2: primary Rust implementation for descriptor_set / fixture_set indexing
        and ActivationDescriptor / ActivePointer / LastKnownGoodDescriptor /
        RollbackPointer binding.
Lane 3: read-only review of router/eval interaction; no verified fallback
        exception, no auto-switch relaxation.
Lane 4: read-only review of sanitized diagnostics and denied-field coverage;
        no raw values in binding diagnostics.
Lane 1: read-only SourceDeltaBatch participation review only; no source-delta
        implementation in this gate.
```

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
descriptor_id
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
quarantined_source_delta_count
limited_source_delta_count
all_required_reviews_present
all_hashes_normalized
all_sources_locator_sanitized
all_sources_runtime_excluded
activation_requested
activation_blocked_reason_codes
leakage_count=0
notes_summary_ref
```

Must equal or match:

- `content_hashes[*]` and `previous_content_hashes[*]` use
  `sha256:<digest>`.
- `source_delta_count` equals the number of source deltas represented by the
  batch.
- accepted, rejected, quarantined, and limited status counts must not exceed
  `source_delta_count`; when review is complete, their sum should equal
  `source_delta_count`.
- `source_delta_batch_hash` and `source_freshness_digest` are aggregate
  digests, not serialized source-register content.
- `freshness_status` and `stale_reason_code` use the canonical enums in this
  matrix.
- `activation_requested=true` requires `all_required_reviews_present=true`.
- `all_sources_runtime_excluded=true` is required.
- `leakage_count` is required when present and must be `0`.

Reject if:

- Any hash is missing or malformed.
- `review_status_summary` or `effective_confidence_summary` is missing while
  activation is requested.
- `source_delta_count` does not match represented source IDs or status counts.
- Required review completion is false while activation is requested.
- Unknown fields appear in a v0.2 descriptor fixture.
- Any source locator, path, URL/path alias, source-register path, raw source
  text, credential, or secret appears.
- The descriptor is copied into runtime payload, runtime logs, UI disclosure,
  telemetry, shadow, rollback, or chat-visible summaries.

Future validator name: `Validate-SourceDeltaBatchDescriptor`.

Future fixture matrix, when total control opens fixture write scope:

```text
pass/lane1_source_delta_batch/01_minimal_reviewed_batch.json
pass/lane1_source_delta_batch/02_mixed_review_status_batch.json
pass/lane1_source_delta_batch/03_activation_ready_aggregate_only.json
fail/lane1_source_delta_batch/01_missing_required_fields.json
fail/lane1_source_delta_batch/02_bad_hash_shape.json
fail/lane1_source_delta_batch/03_count_mismatch.json
fail/lane1_source_delta_batch/04_activation_without_review.json
fail/lane1_source_delta_batch/05_denied_locator_or_source_payload.json
fail/lane1_source_delta_batch/06_runtime_artifact_class_mismatch.json
fail/lane1_source_delta_batch/07_leakage_count_nonzero.json
fail/lane1_source_delta_batch/08_unknown_field_fail_closed.json
```

Fixture design rules:

- pass fixtures use synthetic reviewed batches, valid aggregate hashes, matched
  counts, completed summaries, `leakage_count=0`, and runtime exclusion.
- mixed-status pass fixtures may represent accepted, rejected, quarantined, and
  limited counts when activation is not requested or is explicitly blocked.
- activation-ready pass fixtures must be aggregate-only and must not include
  per-source locators, raw source text, source-register rows, or denied fields.
- fail fixtures use synthetic safe sentinel fields only; they must not contain
  real raw KB rows, raw prompt bodies, raw source text, source-register dumps,
  overlay JSON, raw graph payloads, local path details, credentials, provider
  material, request/response bodies, or matched snippets.

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
- `lkg_freshness_status` is `fresh` for the v0.2 first implementation.
  Accepted previous-good states are deferred until an explicit
  historical-activation descriptor contract exists.

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

Source-delta diagnostics and summaries are aggregate-only:

- diagnostics may report only `descriptor_type`, `descriptor_id`,
  `field_path`, `denied_class`, and `rule_id`
- diagnostics must not include matched values, local file details, original
  source excerpts, source locator values, credential-like values,
  request/response bodies, provider details, or raw prompt/source/graph content
- activation descriptors, query results, retrieval traces, telemetry,
  rollback records, UI, logs, and prompt payloads must not include per-source
  source-delta evidence
- matrix summaries may report expectation, leaf name, descriptors checked,
  status, diagnostics count, errors count, and outcome only
- accepted source-delta aggregate bindings are limited to
  `source_delta_batch_hash`, `source_delta_count`, and
  `source_freshness_digest`

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

The smallest future Rust-first SourceDeltaBatch implementation gate should:

1. Add a static `SourceDeltaBatch` rule module only.
2. Register that rule in the first-wave validator dispatcher.
3. Validate required fields, artifact class, canonical freshness/stale enums,
   hash shape, count consistency, review/confidence summaries, activation
   review preconditions, `leakage_count=0`, runtime exclusion, and
   unknown-field fail-closed behavior.
4. Reuse the existing global denied-field scan before descriptor-specific
   checks.
5. Add Lane 1 synthetic pass/fail fixtures only after total control opens
   fixture write scope.
6. Extend the fixture matrix only by adding new pass/fail leaves; do not
   change matrix semantics.
7. Add unit tests for pass descriptor, missing fields, bad hash shape, count
   mismatch, activation without review, denied field, artifact-class mismatch,
   nonzero leakage, and unknown field.

The first validator must not:

- read runtime artifacts
- read snapshot SQLite files
- read raw KB rows
- read full source registers
- call a network
- call a model
- generate images or video
- modify seed, snapshot, migration, runtime, or Hope files
- implement source acquisition, source storage, source fetching, activation
  switching, telemetry export, canonical digest recomputation, or verified
  fallback exceptions
