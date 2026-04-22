# Golden Sample Intake Dispatch 2026-04-22

## Dispatch Scope

This dispatch is for the existing `hope-kb` thread:

- thread id: `019da89f-49a2-7e11-bd2f-c0138165fdd9`
- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before dispatch: `6617434` (`docs: align KB live progress after desktop stable stop`)
- worktree state before dispatch: clean and aligned with
  `origin/codex/contracts-freeze`

This does not open a new KB thread. It gives the existing KB thread a bounded
KB-only work package.

## Source Package

The user provided two desktop CSV exports from the Notion golden sample library.
They are byte-identical, so the repo carries one canonical copy.

Original local files:

- `C:\Users\Administrator\Desktop\Hope-kb · 黄金样本库 642bd7876e0d4649814f35fb133b67f3.csv`
- `C:\Users\Administrator\Desktop\Hope-kb · 黄金样本库 642bd7876e0d4649814f35fb133b67f3_all.csv`

Canonical committed source:

- `docs/source-exports/golden-sample-library-642bd7876e0d4649814f35fb133b67f3-2026-04-22.csv`

Source hash:

```text
sha256:2D0CE22F8C85CA59DE6ABF35A48239AADEF9A2BD01BB3F7B4685BF80E06BF843
```

CSV summary:

- rows: 40
- duplicate `sample_id`: none
- fields:
  `sample_title`, `core`, `covered_points`, `created_at`, `director_voice`,
  `empty_words_detected`, `genre`, `missed_points`, `sample_id`, `scene_tag`,
  `shot_type`, `source_cut`, `teaching_note`, `tier`, `updated_at`,
  `usable_for_fewshot`, `word_count`
- `core` coverage:
  `visual_scene_core=8`, `motion_performance_core=8`,
  `camera_directing_core=8`, `audio_directing_core=8`,
  `continuity_lock_core=8`
- `tier` coverage: `优=10`, `好=10`, `中=10`, `差=10`
- `usable_for_fewshot`: `Yes=30`, `No=10`

## Task For The KB Thread

In the existing KB thread, run a bounded golden-sample intake package:

1. Treat the committed CSV as the source of truth.
2. Preserve every CSV row and field during normalization; do not invent missing
   content.
3. Decide the storage target inside `hope-kb` without changing `hope`:
   - preferred first step: create a normalized KB-side source/staging artifact
     that preserves all golden-sample fields losslessly
   - only append selected rows to `seed/v0.1/classic_case_examples.json` if the
     mapping is lossless enough for the current v0.1 schema and validation gates
   - do not add a new runtime snapshot table or change migrations/import map
     without a separate v0.2 scope gate
4. If seed files change, update:
   - `seed/v0.1/source_register.json`
   - `seed/v0.1/manifest.json` counts and bundle hash
   - `docs/live-progress.md`
5. Run the KB validation chain before pushing any seed-bundle change:
   - `powershell -ExecutionPolicy Bypass -File E:\codex\hope-kb\scripts\validate-seed-bundle.ps1`
   - `python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb`

## Boundaries

Allowed:

- KB-only source capture, normalization, mapping, docs, validation, and snapshot
  rebuild
- v0.2 planning notes for the golden-sample schema if the current v0.1
  `classic_case_examples` shape cannot store the fields safely

Prohibited:

- do not modify `E:\codex\hope`
- do not merge `hope-kb` into `hope`
- do not alter Hope RC criteria
- do not change desktop or intake packages
- do not promote V3/V4 fields to Hope engineering implementation from this
  package
- do not flatten the CSV into the v0.1 classic-case schema if that loses
  validator, few-shot, or negative-sample meaning

## Acceptance Criteria

The KB thread should report back with:

- final branch and commit
- exact files changed
- whether the CSV was staged only, normalized, or imported into seed files
- record counts before and after any seed import
- bundle hash before and after any seed import
- validation and snapshot-build results
- any blocked fields that require a future v0.2 schema gate

## Message To Existing KB Thread

```text
Use the existing hope-kb thread `019da89f-49a2-7e11-bd2f-c0138165fdd9`.

Pull `E:\codex\hope-kb` on `codex/contracts-freeze`. Start from
`docs/golden-sample-intake-dispatch-2026-04-22.md`.

Execute the bounded KB-only golden-sample intake package from the committed CSV
at `docs/source-exports/golden-sample-library-642bd7876e0d4649814f35fb133b67f3-2026-04-22.csv`.

Do not touch `E:\codex\hope`, desktop, or intake. Do not merge KB into Hope.
If the v0.1 classic-case schema cannot preserve the golden-sample fields
losslessly, stop at normalized KB-side staging plus a v0.2 schema note instead
of forcing the data into the wrong table.

Report back with changed files, counts/hash, validation result, snapshot result,
and any fields requiring a future v0.2 gate.
```
