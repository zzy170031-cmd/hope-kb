# QA Hardening v0.2 Router Eval Gates 2026-04-25

Status: QA contract packet only.
Scope: `hope-kb` v0.2 router and eval gates for future runtime consumption.
Boundary: this packet does not implement runtime retrieval, hybrid search, rerank, model calls, seed edits, or `hope` code.

## Current Anchor

- Current seed bundle: `seed/v0.2/manifest.json`.
- Current snapshot version: `v0.2`.
- Current manifest counts: `golden_sample_library=152`, `golden_sample_field_coverage_rules=5`, `golden_sample_failure_mapping=152`, `golden_sample_repair_mapping=152`.
- Current validator: `scripts/validate-v0-2-seed-bundle.ps1` enforces manifest counts, primary-key uniqueness, bundle hash, coverage totals, and reserve/No status for the 32 new `GS120-CAND-*` rows.
- This packet defines the additional router/eval gates that must exist before any runtime consumer may auto-switch to a new router behavior.

## Router Pool Gate

BM25 remains the v0.2 baseline router pool mechanism.

Required candidate fields:

- `raw_bm25_score`
- `max_in_pool`
- `normalized_score`
- `candidate_pool_size`
- `fallback_reason`

Normalization rules:

- `max_in_pool` is the maximum positive BM25 score in the candidate pool after index lookup and intent filtering, before truncation and before the minimum-score gate.
- `normalized_score = raw_bm25_score / max_in_pool`.
- `normalized_score` must be clamped to `0.0..1.0`.
- `min_candidate_score = 0.15`.
- Candidates with `normalized_score < 0.15` are not eligible for positive few-shot selection.
- If the pool is empty, or if `max_in_pool <= 0`, no candidate normalization may be treated as valid.

Hard behavior:

- The router must not promote reserve holdouts or negative fixtures into positive few-shot selection.
- The router must not use raw full rows as fallback payload.
- The router must preserve the summary-only consumption boundary: selected IDs and compact summaries are allowed; full KB row injection is not.

## Fallback Gate

Every fallback must be explicit and deterministic. Fallback must not silently widen retrieval scope.

| Condition | Trigger | Required behavior |
| --- | --- | --- |
| `on_empty_pool` | Index lookup succeeds but returns zero candidates after intent filtering. | Return no positive few-shot samples, emit `fallback_reason=on_empty_pool`, and use the safe no-sample router summary. |
| `on_low_score` | Candidates exist but top `normalized_score < 0.15`. | Return no positive few-shot samples, emit `fallback_reason=on_low_score`, and keep only intent-level guidance that is already allowed by the snapshot contract. |
| `on_index_miss` | Index is missing, stale, unreadable, or does not match the active snapshot/version hash. | Do not query a guessed or stale index. Emit `fallback_reason=on_index_miss`, block auto-switch, and use deterministic no-sample fallback behavior. |

Fallback outputs must be testable by structured fields, not by human-readable wording alone.

## Intent Gate

Router calls must resolve an explicit intent before candidate retrieval.

Hard rules:

- Intent is a gate, not a hint.
- Candidate pools must be filtered by the resolved intent before BM25 normalization.
- Unknown, ambiguous, or unsupported intent must not enter expensive retrieval or model-expansion paths.
- A request that fails intent resolution must use deterministic fallback and must be counted in eval.
- Overall pass rate is not enough; every configured intent must pass its own gate.

The intent labels are contract labels for the router/eval layer. This packet does not require a seed schema change.

## Layered Eval Gate

The v0.2 router must be evaluated in layers. A later layer cannot compensate for a failed earlier layer.

Required layers:

1. Snapshot compile gate: manifest, import map, bundle hash, record counts, and eval-set sample coverage all pass.
2. Intent routing gate: intent classification is judged before retrieval quality.
3. Candidate pool gate: BM25 normalization, thresholding, and fallback behavior are judged before downstream output quality.
4. Per-intent quality gate: each intent is judged independently.
5. Tail failure gate: severe or repeated tail failures are bounded even if average pass rate looks acceptable.
6. Auto-switch gate: router behavior may auto-switch only if all gates pass and the eval artifact is not stale.

## Required Eval Metrics

Each eval artifact must include these fields:

- `intent_routing_accuracy`
- `pass_rate_overall`
- `pass_rate_per_intent`
- `tail_failure_rate`
- `max_tail_failure_rate_absolute`
- `min_samples_per_intent_in_eval_set = 20`
- `max_per_intent_drop_pp = 0.05`
- `judged_against_model`
- `judged_at`
- `stale_if_model_changes = true`
- `stale_action = block_auto_switch`

Metric rules:

- `max_tail_failure_rate_absolute` is a required configured ceiling; missing ceiling fails the eval gate.
- `max_per_intent_drop_pp = 0.05` means a drop greater than 5 percentage points for any configured intent blocks auto-switch.
- `judged_against_model` must identify the model/version used for judging or target-output evaluation.
- `judged_at` must be machine-readable and comparable.
- Any model or judge version change makes the eval artifact stale when `stale_if_model_changes = true`.

## Hard Blocking Rules

- Missing `min_samples_per_intent_in_eval_set >= 20` fails snapshot compile.
- Model or judge version change makes the eval stale and blocks auto-switch.
- Per-intent regression blocks auto-switch.
- Overall pass rate alone cannot approve router changes.
- `on_index_miss` blocks auto-switch.
- A candidate below `min_candidate_score = 0.15` must not be used as a positive few-shot sample.
- Hybrid search and rerank are not v0.2 defaults.

## Acceptance Tests

### Eval Stale Blocks Auto-Switch

Given an eval artifact with `judged_against_model=model-A`, `judged_at` set, and `stale_if_model_changes=true`, when the target model or judge changes to `model-B`, then the eval artifact is stale and `stale_action=block_auto_switch` must prevent auto-switch until eval is rerun.

### Sample Insufficiency Fails Compile

Given an eval set where any configured intent has fewer than `20` samples, when snapshot compile or router-gate validation runs, then the gate fails before runtime consumption and reports the insufficient intent.

### Per-Intent Regression Blocks Auto-Switch

Given an eval run where `pass_rate_overall` passes but one configured intent drops by more than `0.05` against its accepted baseline, then auto-switch is blocked and the failing intent must be reported.

### No Short-Input Expensive Path

Given an empty, very short, or intent-ambiguous input, when BM25 returns an empty pool, low-score pool, or unresolved intent, then the router must use deterministic fallback and must not call hybrid search, rerank, vector expansion, query rewriting, or live model expansion. The acceptance assertion must prove `expensive_path_used=false` or equivalent structured evidence.

## Version Boundary

v0.2 must not implement hybrid or rerank as default behavior.

Hybrid/rerank exploration may only appear later as `v0.2.1+` shadow-only work:

- no default traffic
- no auto-switch
- no mutation of v0.2 snapshot semantics
- no positive few-shot promotion from reserve or negative fixtures
- no runtime-consumer dependency until the eval gates above pass
