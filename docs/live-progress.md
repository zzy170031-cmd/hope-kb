# hope-kb Live Progress

Update this file for online sync after every 10-20 minute work package.

## Current Focus

- current thread state: `codex/contracts-freeze`, `hope-kb` only, no merge with `hope`
- current task: V120 CN war / SLG candidate pool one-shot ingest is being finalized into the v0.2 seed package, with the prior 120-row V120 package preserved as the base and Hope runtime promotion still closed
- owner / lane: KB integration owner, coordinating candidate-pool dry-run, v0.2 seed rebuild, validation, snapshot rebuild, and boundary sync with `hope`
- last updated: 2026-04-24 13:40:00 +08:00

## Latest Completed

- latest pushed commit before this ingest: `91fb18c` (`data: ingest V120 full KB package`)
- latest local change: added the V120 CN war / SLG candidate pool dry-run + ingest scripts, generated `docs/v120-cnwar-slg-one-shot-dry-run-2026-04-24.md`, and rebuilt `seed/v0.2/golden_sample_library.json`, `golden_sample_field_coverage_rules.json`, `golden_sample_failure_mapping.json`, `golden_sample_repair_mapping.json`, `source_register.json`, and `manifest.json` for the new 32-row one-shot add-on
- accepted ingest status: `V120 = primary KB ingest source`; `V108 = comparison baseline`; current v0.2 package now carries 152 rows / 23 source fields with the new 32 rows held as reserve-only sequence samples, while Hope runtime import, product-ready external references, and `reference_control_core` remain closed
- before / after package counts: `golden_sample_library 120 -> 152`, `golden_sample_field_coverage_rules 5 -> 5`, `golden_sample_failure_mapping 120 -> 152`, `golden_sample_repair_mapping 120 -> 152`, `golden_sample_sources 13 -> 14`, `golden_sample_provenance_entries 3 -> 4`
- candidate-pool source evidence status: 152 rows total, `official = 108`, `reserve = 44`, `single_shot = 72`, `sequence_shot = 80`, `usable_for_fewshot = Yes 97 / No 55`; the new 32 rows are `GS120-CAND-001 .. GS120-CAND-032`, grouped into `CNWARSEQ01-04` and `SLGSEQ01-04`, all `reserve` / `No`
- dry-run note: raw workbook structure passed, but the new 32 `prompt_body` fields carried label / IP leakage (`国漫`, `架空三国`) and were safety-normalized on ingest only for those new 32 rows; the other 22 source fields remained unchanged
- boundary handling: no Hope repo edits, no desktop edits, no intake edits, no Qwen / Seedance calls, no live Hope runtime import of all 120 rows, no product-ready external references, no `reference_control_core`
- latest validation command for this package: `powershell -ExecutionPolicy Bypass -File E:\codex\hope-kb\scripts\validate-v0-2-seed-bundle.ps1 -RepoRoot E:\codex\hope-kb` passed; `python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb --version v0.2` passed and rebuilt snapshot at `snapshots/hope-kb-v0.2.rebuilt-4.sqlite3`

## Next Up

- next 30 minutes: commit and push the V120 CN war / SLG candidate pool package, then report dry-run / counts / hash / snapshot result back to main control
- next 60-90 minutes: return to KB standby unless main control opens a bounded follow-up on runtime promotion, external reference normalization, or validator/export gate work

## Blockers / Risks

- blocker: no active KB repo blocker; this package is validated and snapshotted, but any live Hope runtime import, product-ready external reference handles, or `reference_control_core` still requires a later main-control gate
- unresolved V120 gaps kept closed by design: full ingest does not equal Hope runtime readiness; reserve rows remain holdout-only; external reference handles remain source text only; no product handle normalization or runtime promotion is opened in this package
- risk to `hope` separation: none in implementation scope; keep Hope product-side runtime hookup, desktop, intake, Qwen, Seedance, and product import behavior out of this repo
- merge-readiness status: closed, not reopened
