# hope-kb Live Progress

Update this file for online sync after every 10-20 minute work package.

## Current Focus

- current thread state: `codex/contracts-freeze`, `hope-kb` only, no merge with `hope`
- current task: `hope-kb-QA` v0.2 QA / hardening RFC and four bounded contract packets for snapshot/source integrity, router/eval gates, ephemeral prompt safety, and validation/telemetry/leakage
- owner / lane: KB integration owner as `hope-kb-QA-Control`, coordinating four non-overlapping QA lanes and keeping `hope` runtime / desktop work out of scope
- last updated: 2026-04-25 14:09:23 +08:00

## Latest Completed

- live Git anchor checked by control: `d5b319c` (`data: absorb V148 safe-plus capability`) on `codex/contracts-freeze`; only pre-existing `AGENTS.md` was dirty at start
- four QA worker packets added under `docs/`: snapshot/source integrity, router/eval gates, ephemeral prompt safety, and validation/telemetry/leakage
- control RFC added: `docs/qa-hardening-v0.2-rfc-2026-04-25.md`
- v0.2 mandatory contract coverage now recorded for `manifest_hash` / `snapshot_hash` / `index_hash`, last-known-good / fail-closed behavior, quarantine isolation, source minimum fields, BM25 score normalization, layered eval stale blocking, ephemeral context prompt safety, structural leakage tests, local telemetry purge, shadow thresholds, and rollback triggers
- v0.2.1 deferrals recorded: signature chain / public-key verification / provenance attestation, hybrid/rerank shadow, compact evidence packs, maintenance UI review queues, telemetry cloud sync, Chinese ephemeral token multiplier, and catastrophic rollback threshold calibration
- validation rerun by control: `powershell -ExecutionPolicy Bypass -File E:\codex\hope-kb\scripts\validate-v0-2-seed-bundle.ps1 -RepoRoot E:\codex\hope-kb` passed for the current 152-row bundle
- snapshot check by control: `snapshots/hope-kb-v0.2.rebuilt-5.sqlite3` passed SQLite `quick_check`, table counts `152 / 5 / 152 / 152 / 17 / 5`, and internal metadata matched manifest bundle hash `bundle-sha256:b16bb3719d73c7246eef674838130bc671879263f5889075cfbc4fbe1fff049e`
- observed integrity note: `snapshots/hope-kb-v0.2.rebuilt-5.sqlite3` SHA-256 is `4FE4C77E1617F5DA613668A0FEB3833136D1497F92B9F788F9A22DFD05980F25`; ignored standard `snapshots/hope-kb-v0.2.sqlite3` currently differs and must not be treated as active without install/switch verification

## Next Up

- next 30 minutes: review the five docs as a single RFC package, then decide whether to stage/commit or open a bounded validator/contract implementation gate
- next 60-90 minutes if implementation is approved: add machine-checkable descriptors/tests for runtime activation hashes, source minimum-field normalization, eval stale blocking, and structural leakage deny-list checks
- if no new gate is opened: keep `hope-kb` in QA standby and do not touch `E:\codex\hope`

## Blockers / Risks

- blocker: no active KB repo blocker; current package is docs/RFC only and does not yet implement machine-checkable validator gates for the new QA contracts
- risk: standard ignored `snapshots/hope-kb-v0.2.sqlite3` differs from verified `rebuilt-5`; any future runtime activation must bind a chosen snapshot by hash instead of relying on path name
- risk to `hope` separation: none in current implementation scope; keep Hope product-side runtime hookup, desktop, intake, Qwen, Doubao, Seedance, GraphRAG, hybrid/rerank default, and runtime LLM summarize out of this repo/package
- merge-readiness status: closed, not reopened
