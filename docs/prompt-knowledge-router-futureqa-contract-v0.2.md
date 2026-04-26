# Prompt Knowledge Router and Future QA Contract v0.2

Status: docs-only contract.
Scope: `hope-kb` v0.2 Prompt Query, Future QA candidate review, and router/eval gates.
Boundary: this document does not modify seed files, snapshots, migrations, validator scripts, Hope runtime code, desktop UI, image generation, video generation, runtime GraphRAG, network fetching, or runtime LLM summarization.

## Purpose

The first v0.2 priority is storyboard prompt quality. Query and Future QA are support surfaces for measuring and improving Prompt KB routing quality. They do not trigger image or video generation, do not create automatic eval truth, and do not expand the KB into a runtime knowledge-management product.

This contract adds the first-version boundary for:

- `governance_query`
- `runtime_query`
- `future_qa_candidate`
- intent routing quality gates
- per-intent eval gates
- stale eval blocking
- QA candidate review and promotion rules

## Non-Goals

- Do not call image generation or video generation from a Query.
- Do not treat Query as a media-generation trigger.
- Do not turn Query into default runtime GraphRAG.
- Do not perform runtime network fetching.
- Do not use runtime LLM summarization by default.
- Do not expose raw KB rows, raw `prompt_body`, full `source_register`, overlay JSON, raw graph data, local paths, API keys, tokens, or provider secrets.
- Do not promote `future_qa_candidate` records into eval truth without review, Git promotion, rebuild, and validation.
- Do not auto-switch router behavior on stale eval artifacts.

## Query Types

### `governance_query`

`governance_query` is an offline or controller-facing query used to inspect Prompt KB quality and router readiness. It exists for governance, QA design, candidate generation, and review workflows.

Allowed uses:

- evaluate whether prompt intents are covered by reviewed KB samples
- reproduce router misses, low-score pools, or per-intent failures
- assemble candidate QA items for human review
- audit sanitized router traces and fallback reasons
- support docs, validator, or gate design before runtime consumption

Disallowed uses:

- trigger image or video generation
- modify seed records, snapshots, import maps, manifests, or source registers
- write candidate items into eval truth automatically
- fetch new network data
- summarize raw KB rows with a runtime LLM
- expose raw KB records or full prompt bodies in output

Required output boundary:

- may include `selected_sample_ids`
- may include `selected_kb_rules`
- may include sanitized `kb_context_summary`
- may include sanitized `retrieval_trace`
- must not include full seed records, full source register entries, full provenance dumps, raw `prompt_body`, raw graph internals, or overlay JSON

### `runtime_query`

`runtime_query` is a future runtime-consumer request for a bounded Prompt KB routing decision. It may support prompt-package assembly or prompt quality checks, but it is not the media generation request itself.

Allowed uses:

- resolve a supported prompt intent before candidate retrieval
- choose reviewed, eligible few-shot sample IDs within the active snapshot boundary
- return compact KB rules and a summary-only context package
- emit deterministic fallback fields when routing cannot safely select samples
- provide structured evidence for downstream QA gates

Disallowed uses:

- call an image provider or video provider
- call hybrid search, rerank, query rewriting, vector expansion, or live model expansion as a v0.2 default path
- query a stale or unverified index
- load reserve holdouts or negative fixtures as positive few-shot examples
- inject raw KB rows as fallback payload
- auto-summarize raw KB content with a runtime LLM

Required output boundary:

```json
{
  "selected_sample_ids": [],
  "selected_kb_rules": [],
  "kb_context_summary": "",
  "retrieval_trace": {
    "query_type": "runtime_query",
    "intent": "",
    "candidate_pool_size": 0,
    "fallback_reason": "on_empty_pool",
    "full_kb_rows_included": 0,
    "raw_prompt_body_included": false,
    "source_register_included": false,
    "overlay_json_included": false
  }
}
```

`full_kb_rows_included` must remain `0`.

## Intent Routing Contract

Every Query that enters router evaluation must resolve one explicit intent before candidate retrieval. Intent is a gate, not a hint.

Required behavior:

- filter the candidate pool by resolved intent before BM25 normalization
- count unknown, ambiguous, or unsupported intent as an eval case
- use deterministic fallback for unresolved intent
- avoid expensive model/provider paths on short, empty, unsupported, or ambiguous input
- evaluate overall and per-intent quality separately

Minimum structured fields:

- `intent`
- `intent_confidence`
- `intent_routing_accuracy`
- `candidate_pool_size`
- `raw_bm25_score`
- `max_in_pool`
- `normalized_score`
- `fallback_reason`

Fallback reasons:

| Field | Trigger | Required action |
| --- | --- | --- |
| `on_empty_pool` | Index lookup succeeds but no candidates remain after intent filtering. | Return no positive samples, emit a safe no-sample summary, and count the case in eval. |
| `on_low_score` | Candidates exist but top `normalized_score < 0.15`. | Return no positive samples and keep only allowed intent-level guidance. |
| `on_index_miss` | Index is missing, stale, unreadable, or hash-mismatched. | Do not query guessed or stale data; block auto-switch and use deterministic no-sample fallback. |

## Router Output Contract

Router outputs must be summary-only and structurally testable.

Allowed:

- `selected_sample_ids`
- `selected_kb_rules`
- `kb_context_summary`
- `retrieval_trace`
- snapshot/version/hash identifiers
- normalized scores and fallback reason codes

Denied:

- raw KB rows
- raw `prompt_body`
- full `source_register`
- overlay JSON
- raw graph internals
- quarantine raw text
- local file paths
- API keys, tokens, provider headers, credential values, or secret references

`retrieval_trace` must be safe for logs and telemetry. It may explain why routing succeeded or fell back, but it must not become a raw data dump.

## Machine-Checkable Descriptor Drafts

These descriptors are first-version contract shapes for a later validator or
consumer test gate. They are not seed changes, snapshot changes, runtime
implementations, or eval truth records.

### `QueryResult`

`QueryResult` is the bounded output shape for `governance_query` and
`runtime_query`.

```yaml
descriptor_type: QueryResult
descriptor_version: prompt_knowledge_query_result.v0.2
query_type: governance_query | runtime_query
resolved_intent: string
intent_confidence: number
intent_routing_status: resolved | ambiguous | unsupported | empty
selected_sample_ids: list<string>
selected_kb_rules: list<string>
kb_context_summary: string
retrieval_trace_ref: string
fallback_reason: none | on_empty_pool | on_low_score | on_index_miss | unresolved_intent
full_kb_rows_included: 0
raw_prompt_body_included: false
full_source_register_included: false
overlay_json_included: false
raw_graph_included: false
local_paths_included: false
secrets_included: false
media_generation_triggered: false
image_generation_triggered: false
video_generation_triggered: false
graphrag_used: false
hybrid_search_used: false
rerank_used: false
runtime_llm_summarize_used: false
expensive_path_used: false
```

Required validation rules:

- `query_type` must be `governance_query` or `runtime_query`.
- `full_kb_rows_included` must equal `0`.
- all `*_included` leakage booleans must be `false`.
- all media, graph, hybrid, rerank, summarization, and expensive-path booleans
  must be `false` for v0.2 default behavior.
- if `intent_routing_status` is not `resolved`, `selected_sample_ids` must be
  empty and `fallback_reason` must be deterministic.

### `RetrievalTrace`

`RetrievalTrace` is safe operational evidence for routing, logs, telemetry,
shadow records, and review. It is not a raw evidence packet.

```yaml
descriptor_type: RetrievalTrace
descriptor_version: prompt_knowledge_retrieval_trace.v0.2
trace_id: string
query_type: governance_query | runtime_query
resolved_intent: string
candidate_pool_size: integer
raw_bm25_score: number | null
max_in_pool: number | null
normalized_score: number | null
min_candidate_score: 0.15
fallback_reason: none | on_empty_pool | on_low_score | on_index_miss | unresolved_intent
selected_sample_ids: list<string>
selected_kb_rules: list<string>
snapshot_version: string
snapshot_hash: string
index_hash: string
full_kb_rows_included: 0
raw_prompt_body_included: false
full_source_register_included: false
overlay_json_included: false
raw_graph_included: false
local_paths_included: false
secrets_included: false
expensive_path_used: false
```

Required validation rules:

- trace fields may contain IDs, counts, hashes, scores, and reason codes only.
- `retrieval_trace` must not inline source records, prompt bodies, source text,
  graph neighborhoods, overlay JSON, local paths, provider config, credential
  refs, API key refs, tokens, or secrets.
- `on_index_miss` must block auto-switch.
- `on_empty_pool`, `on_low_score`, and `unresolved_intent` must keep
  `selected_sample_ids` empty unless a later approved contract defines a safe
  non-positive guidance surface.

### `FutureQACandidate`

`FutureQACandidate` is a candidate review record. It is not eval truth and
does not count toward eval-set coverage until promoted through an approved
Git-visible eval dataset path and validated.

```yaml
descriptor_type: FutureQACandidate
descriptor_version: prompt_knowledge_future_qa_candidate.v0.2
candidate_id: string
source_query_type: governance_query | runtime_query | manual_review | shadow_observation
source_intent: string
candidate_question: string
candidate_expected_behavior: string
candidate_failure_mode: string
selected_sample_ids: list<string>
selected_kb_rules: list<string>
kb_context_summary: string
retrieval_trace_ref: string
review_status: candidate | needs_review | accepted_for_eval_draft | promoted_by_git | rejected
reviewer: string | null
created_at: datetime
judged_against_model: string
judged_at: datetime
stale_if_model_changes: true
stale_action: block_auto_switch
is_eval_truth: false
enters_eval_truth_by_default: false
requires_human_review: true
requires_git_promotion: true
requires_rebuild_and_validation: true
raw_kb_rows_included: false
raw_prompt_body_included: false
full_source_register_included: false
overlay_json_included: false
raw_graph_included: false
local_paths_included: false
secrets_included: false
```

Required validation rules:

- `is_eval_truth` must remain `false` until the candidate is represented in an
  approved eval truth surface outside this descriptor.
- `promoted_by_git` is the only state that may point to eval truth, and only
  after human/controller review, dedupe, Git promotion, rebuild, and validation.
- model-generated or shadow-observed candidates may suggest coverage gaps, but
  cannot become truth by themselves.
- candidate records must be leakage-clean and summary-only.

### `EvalArtifact`

`EvalArtifact` is the machine-checkable routing/eval gate record for a proposed
router or snapshot-selection behavior.

```yaml
descriptor_type: EvalArtifact
descriptor_version: prompt_knowledge_eval_artifact.v0.2
eval_artifact_id: string
snapshot_version: string
snapshot_hash: string
index_hash: string
intent_label_set_hash: string
eval_truth_set_hash: string
intent_routing_accuracy: number
pass_rate_overall: number
pass_rate_per_intent: map<string, number>
tail_failure_rate: number
max_tail_failure_rate_absolute: number
min_samples_per_intent_in_eval_set: 20
max_per_intent_drop_pp: 0.05
judged_against_model: string
judged_at: datetime
stale_if_model_changes: true
stale_action: block_auto_switch
auto_switch_allowed: false
blocked_reason_codes: list<string>
future_qa_candidates_counted_as_truth: false
media_generation_triggered: false
graphrag_used: false
hybrid_search_used: false
rerank_used: false
runtime_llm_summarize_used: false
expensive_path_used: false
```

Required validation rules:

- missing per-intent sample minimums fail compile.
- missing `max_tail_failure_rate_absolute` fails the eval gate.
- `stale_action` must be `block_auto_switch`.
- any stale model, judge, snapshot, index, intent label set, eval truth set,
  fallback behavior, or threshold change must set `auto_switch_allowed=false`.
- Future QA candidates must not be counted as eval truth.
- no image/video generation, GraphRAG, hybrid search, rerank, runtime LLM
  summarization, or expensive default path may be used as v0.2 eval approval
  evidence.

## Future QA Candidate Contract

`future_qa_candidate` is a candidate review object. It is not eval truth.

Allowed sources:

- `governance_query` results from reviewed audit runs
- runtime shadow observations that contain only sanitized IDs and metrics
- manually written QA ideas from controller or lane review
- per-intent tail failures and router misses
- stale eval review notes after model or judge changes

Disallowed sources:

- unreviewed raw user input promoted directly into seed
- runtime network fetches
- raw KB rows or full prompt bodies
- model-generated truth accepted without human review
- quarantined material promoted without Git review

Minimum candidate fields:

```json
{
  "candidate_id": "",
  "source_query_type": "governance_query",
  "source_intent": "",
  "candidate_question": "",
  "candidate_expected_behavior": "",
  "candidate_failure_mode": "",
  "selected_sample_ids": [],
  "selected_kb_rules": [],
  "kb_context_summary": "",
  "retrieval_trace": {},
  "review_status": "candidate",
  "reviewer": "",
  "created_at": "",
  "judged_against_model": "",
  "judged_at": "",
  "stale_if_model_changes": true,
  "stale_action": "block_auto_switch"
}
```

Candidate states:

| State | Meaning | May enter eval truth |
| --- | --- | --- |
| `candidate` | Captured for possible future QA review. | No |
| `needs_review` | Needs human/controller judgment and dedupe. | No |
| `accepted_for_eval_draft` | Accepted as an eval draft, but not yet truth. | No |
| `promoted_by_git` | Reviewed, committed through the approved eval dataset path, and validated. | Yes, only after rebuild and validation |
| `rejected` | Not suitable for eval truth. | No |

Promotion rules:

- a candidate must be reviewed by a human/controller gate before promotion
- accepted candidates must be deduplicated by intent and failure mode
- accepted candidates must preserve the prompt-quality purpose of v0.2
- promotion must happen through explicit Git-visible dataset or contract changes
- promotion must run the relevant validator/eval gate before any auto-switch
- model-generated candidates may suggest coverage but may not become truth by themselves

## Per-Intent Eval Gate

Eval artifacts must be layered. A later quality result cannot compensate for a failed earlier gate.

Required layers:

1. Snapshot compile gate.
2. Intent routing gate.
3. Candidate pool gate.
4. Per-intent quality gate.
5. Tail failure gate.
6. Auto-switch gate.

Required eval metrics:

```json
{
  "intent_routing_accuracy": 0.0,
  "pass_rate_overall": 0.0,
  "pass_rate_per_intent": {},
  "tail_failure_rate": 0.0,
  "max_tail_failure_rate_absolute": 0.0,
  "min_samples_per_intent_in_eval_set": 20,
  "max_per_intent_drop_pp": 0.05,
  "judged_against_model": "",
  "judged_at": "",
  "stale_if_model_changes": true,
  "stale_action": "block_auto_switch"
}
```

Hard rules:

- missing `min_samples_per_intent_in_eval_set >= 20` fails compile
- missing `max_tail_failure_rate_absolute` fails the eval gate
- overall pass rate alone cannot approve router changes
- any per-intent drop greater than `max_per_intent_drop_pp` blocks auto-switch
- any model or judge version change makes the eval stale when `stale_if_model_changes = true`
- stale eval must use `stale_action = block_auto_switch`
- `on_index_miss` blocks auto-switch
- candidate QA coverage does not count as truth coverage until promoted and validated

## Stale Blocking Contract

`stale_action = block_auto_switch` is mandatory for v0.2 Query and Future QA eval artifacts.

An eval artifact is stale if:

- `judged_against_model` changes
- judge version or scoring rubric changes
- active snapshot hash changes
- active index hash changes
- intent label set changes
- eval truth set changes
- fallback behavior changes
- per-intent thresholds change

Required stale behavior:

- do not auto-switch router behavior
- keep the last verified active behavior if healthy
- otherwise fall back to deterministic no-KB or no-sample behavior
- require fresh eval before promotion or switch
- report the stale reason through structured fields

## Acceptance Tests

### Query Does Not Trigger Media Generation

Given a `governance_query` or `runtime_query`, when routing runs, then no image-generation provider, video-generation provider, media job queue, or media export path is invoked.

### Future QA Candidate Is Not Eval Truth

Given a generated `future_qa_candidate`, when the candidate is created, then it remains outside eval truth until human/controller review, Git promotion, rebuild, and validation have all passed.

### Per-Intent Regression Blocks Auto-Switch

Given `pass_rate_overall` passes but one intent drops by more than `0.05`, then auto-switch is blocked and the failing intent is reported.

### Stale Model Blocks Auto-Switch

Given an eval artifact judged against model A, when the judge or target model changes to model B, then the artifact is stale and `stale_action = block_auto_switch` prevents router auto-switch.

### Empty Pool Is Deterministic

Given `on_empty_pool`, then the router returns no positive samples, emits safe summary-only context, and does not widen retrieval, call GraphRAG, call a runtime LLM summarizer, or expose raw rows.

### Low Score Is Deterministic

Given `on_low_score`, then the router returns no positive samples, emits the low-score reason, and keeps only allowed intent-level guidance.

### Index Miss Blocks

Given `on_index_miss`, then the router refuses stale or guessed indexes, blocks auto-switch, and returns deterministic no-sample fallback behavior.

### Summary-Only Trace

Given any Query result, then `retrieval_trace` may include safe IDs, counts, hashes, scores, and reason codes, but must not include raw KB rows, raw `prompt_body`, full `source_register`, overlay JSON, raw graph data, local paths, or secrets.

## v0.2 Landing

This document is a first-version contract for Prompt Query and Future QA positioning. It intentionally lands as docs-only work.

The next implementation gate, if opened by control, should be bounded to machine-checkable schemas/tests for:

- Query result shape
- Future QA candidate review state
- per-intent eval artifact fields
- stale eval detection
- structural leakage checks for `retrieval_trace`
- explicit no-media-generation assertions
