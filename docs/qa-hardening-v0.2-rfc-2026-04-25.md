# QA Hardening v0.2 RFC 2026-04-25

## Status

Status: controller RFC and validation plan.
Plan name: `hope-kb-QA`.
Repo: `E:\codex\hope-kb`.
Branch read by control: `codex/contracts-freeze`.
HEAD read by control: `d5b319c data: absorb V148 safe-plus capability`.
Known pre-existing dirty file at start: `AGENTS.md`.

This package keeps `hope` and `hope-kb` separate. It does not edit
`E:\codex\hope`, does not expand Hope desktop into a knowledge-management
product, and does not open default runtime GraphRAG, hybrid/rerank, network
fetching, automatic ingestion, or runtime LLM summarization.

## Current Verified Baseline

The current v0.2 seed bundle is the 152-row V120/V148 safe-plus package:

- `manifest.content_hash`:
  `bundle-sha256:b16bb3719d73c7246eef674838130bc671879263f5889075cfbc4fbe1fff049e`
- seed counts: `golden_sample_library=152`,
  `golden_sample_field_coverage_rules=5`,
  `golden_sample_failure_mapping=152`,
  `golden_sample_repair_mapping=152`,
  `golden_sample_sources=17`,
  `golden_sample_provenance_entries=5`
- reserve boundary: 32 `GS120-CAND-*` rows remain `reserve / No`
- schema boundary: 23 fields, no `director_style_ref`

Control verification on 2026-04-25:

- `validate-v0-2-seed-bundle.ps1 -RepoRoot E:\codex\hope-kb`: passed
- `snapshots/hope-kb-v0.2.rebuilt-5.sqlite3` SQLite `quick_check`: `ok`
- rebuilt-5 table counts: `152 / 5 / 152 / 152 / 17 / 5`
- rebuilt-5 internal meta: `v0.2`,
  `hope-kb-golden-sample-library-v0.2`,
  `bundle-sha256:b16bb3719d73c7246eef674838130bc671879263f5889075cfbc4fbe1fff049e`
- rebuilt-5 file SHA-256:
  `4FE4C77E1617F5DA613668A0FEB3833136D1497F92B9F788F9A22DFD05980F25`

Observed risk: `snapshots/hope-kb-v0.2.sqlite3` currently has a different
SHA-256,
`8C766D9E5575D5ADDE296FAEAF2DC4A1FD4E522CDF6E7F2C4FA7F041AB87775A`.
No consumer should treat the standard ignored path as the active v0.2 runtime
snapshot until an install/switch verification explicitly chooses and binds it.

## Worker Packets

This RFC integrates four bounded worker packets:

- `docs/qa-hardening-v0.2-snapshot-source-integrity-2026-04-25.md`
- `docs/qa-hardening-v0.2-router-eval-gates-2026-04-25.md`
- `docs/qa-hardening-v0.2-ephemeral-prompt-safety-2026-04-25.md`
- `docs/qa-hardening-v0.2-validation-telemetry-leakage-2026-04-25.md`

The packets are docs-only contracts. They intentionally do not modify seed
records, snapshots, validator scripts, Hope runtime code, or desktop UI.

## v0.2 Contract Decisions

### Snapshot and Source Integrity

v0.2 activation must bind three release/install artifacts:

- `manifest_hash`
- `snapshot_hash`
- `index_hash`

`manifest.content_hash` remains the current seed bundle hash. A runtime
activation descriptor must additionally bind the selected snapshot and selected
runtime selection index before any consumer treats the KB as active.

Verification timing:

- install or snapshot switch: full verification of manifest, snapshot, index,
  metadata, counts, source minimum fields, quarantine exclusion, and health
- app startup: light check of the already active descriptor/snapshot/index
- runtime query: no manifest, snapshot, index, source-file, or full-table
  rehash

Failure behavior:

- keep current verified active snapshot if healthy
- otherwise activate a verified `last_known_good_snapshot`
- otherwise fail closed with `no_kb_context`
- never full-table scan, network-rescue, load unreviewed temporary data, or
  promote quarantine/candidate data

Source minimum fields for future runtime activation:

- `source_id`
- `source_type`
- `url_or_path`
- `content_hash`
- `review_status`
- `effective_confidence`

The current v0.2 register has `path` and `sha256`; future schema or adapter work
may normalize them to `url_or_path` and `content_hash`. Missing
`review_status` or `effective_confidence` excludes the source from new runtime
activation.

Isolation zones:

- `quarantine_raw`
- `quarantine_labeled`
- `seed_candidate`

These zones must not enter active manifest bundles, active snapshots, active
indexes, runtime selection views, or runtime KB context until reviewed,
promoted through Git, rebuilt, and verified.

### Router and Eval Gates

BM25 remains the v0.2 baseline router pool mechanism. Required fields:

- `raw_bm25_score`
- `max_in_pool`
- `normalized_score`
- `candidate_pool_size`
- `fallback_reason`

`normalized_score = raw_bm25_score / max_in_pool`, clamped to `0.0..1.0`.
`min_candidate_score = 0.15`. Below-threshold candidates are not eligible for
positive few-shot selection.

Required fallback reasons:

- `on_empty_pool`
- `on_low_score`
- `on_index_miss`

Fallback must be explicit, deterministic, and summary-only. It must not widen
retrieval, use raw rows, or promote reserve/negative fixtures.

Layered eval is a hard gate. Required metrics:

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

Hard rules:

- missing minimum samples for any current intent fails snapshot/router compile
- judge/model version change makes old eval stale and blocks auto-switch
- per-intent regression over threshold blocks auto-switch
- overall pass rate alone cannot approve a router switch
- hybrid/rerank is not a v0.2 default path

### Ephemeral Context and Prompt Safety

Ephemeral context is optional, session-scoped, and untrusted.

Required prompt placement:

1. validated `kb_context_summary`
2. accepted ephemeral context block
3. `selected_sample_excerpts`

Rules:

- `max_ephemeral_tokens = 1200`
- use the exact model-visible prefix defined in the ephemeral worker packet
- high-risk content is rejected from the prompt
- suspicious but usable content is wrapped only as untrusted context
- no default runtime LLM summarization
- ephemeral context does not write Git, enter snapshots, join default KB
  selection, enter eval truth, or override verified KB rules

Minimum safety checks:

- empty input
- over token limit
- obvious secret or API key
- local path
- mojibake or binary-looking content
- HTML or script
- ignore-rule request
- system prompt leak request
- API key leak request
- KB rule override request
- source register, raw graph, raw KB row, or full prompt-body request

### Validation, Telemetry, Leakage, Shadow, Rollback

Payload leakage tests must parse structured fields and paths. String scanning
may be secondary, but it is not sufficient.

Denied in payloads, logs, telemetry, shadow records, and rollback records:

- prompt full text
- `prompt_body`
- source original text
- user local paths
- API keys, tokens, provider headers, or credential values
- raw graph
- raw KB rows
- full source register
- quarantine raw original text

Allowed disclosure stays bounded to sanitized summaries and identifiers such as
`snapshot_version`, `snapshot_hash`, `selected_sample_ids`,
`selected_kb_rules`, `payload_bytes`, `fallback_reason_code`, and
`kb_context_summary`.

Telemetry is local by default. Purge must remove aggregate files or truncate
them to empty state; no default cloud telemetry sync is part of v0.2.

Shadow runs observe selected IDs, payload size, fallback rate, leakage result,
purge result, and short-input behavior. Threshold breach blocks auto-switch.
Rollback records must contain only sanitized incident metadata.

## Acceptance Matrix

The v0.2 QA/hardening gate is not closed until these contract tests are
represented in validator/test plans or downstream consumer tests:

- tampered manifest, snapshot, or index refuses candidate activation
- active snapshot failure uses verified last-known-good or `no_kb_context`
- short input does not trigger network, Git, full graph traversal, runtime LLM
  summarization, or expensive model/provider paths
- ephemeral injection cannot override KB rules
- ephemeral leak requests cannot reveal prompt, API key, source register, raw
  graph, or raw KB rows
- stale eval blocks auto-switch
- intent eval sample insufficiency blocks snapshot/router compile
- per-intent regression above `0.05` blocks auto-switch
- shadow selected IDs, payload size, or fallback-rate threshold breach blocks
  auto-switch
- telemetry purge leaves local aggregate files empty or removed
- payload structural leakage count is exactly zero
- `quarantine_raw` and `quarantine_labeled` never enter runtime selection

## v0.2.1 Deferrals

These are recorded only and must not become v0.2 default runtime behavior:

- signature chain, public key verification, provenance attestation
- hybrid/rerank shadow beyond observation
- compact neighborhood or compact evidence pack, compile-time only
- hope-kb maintenance UI for review queues, source freshness, risk scores, or
  failed-case eval candidates
- telemetry opt-in cloud sync
- Chinese ephemeral token multiplier
- catastrophic rollback threshold calibration from stable-period variance

## Integration Decision

The minimal landing for this package is docs/RFC plus worker contract packets.
No manifest JSON, seed row, snapshot file, validator script, Hope runtime, or
desktop UI change is included in this commit-sized work package.

Next implementation gate, if approved by control, should be a bounded
validator/contract task that adds machine-checkable descriptors for:

- runtime activation descriptor shape
- `manifest_hash` / `snapshot_hash` / `index_hash`
- source minimum field normalization
- eval artifact schema and stale blocking
- structural leakage deny-list tests
