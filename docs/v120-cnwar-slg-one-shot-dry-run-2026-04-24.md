# V120 CN War / SLG Candidate Pool Dry-Run 2026-04-24

## Scope

- source workbook: `E:\codex\outputs\黄金样本库v120究极版_V120专项增强候选池_20260424.xlsx`
- target repo: `E:\codex\hope-kb` on `codex/contracts-freeze`
- write mode: dry-run only for this step; no seed files were modified while generating this report
- ingest assumption: new 32 rows will be appended to v0.2 as reserve sequence samples, and their `prompt_body` will be safety-normalized on ingest to remove label / IP leakage while leaving the other 22 source fields untouched

## Structural Checks

- workbook sheet `V120工作簿` column count: `23`
- workbook headers match `seed/v0.2/golden_sample_library.json` `source_field_order`: `True`
- workbook row count: `152`
- existing 120 rows unchanged versus current v0.2 seed: `True`
- new candidate IDs present: `GS120-CAND-001 .. GS120-CAND-032`

## New 32 Checks

- all new rows are `reserve`: `True`
- all new rows are `usable_for_fewshot=No`: `True`
- all new rows are `sequence_shot`: `True`
- sequence groups: `{"CNWARSEQ01": 4, "CNWARSEQ02": 4, "CNWARSEQ03": 4, "CNWARSEQ04": 4, "SLGSEQ01": 4, "SLGSEQ02": 4, "SLGSEQ03": 4, "SLGSEQ04": 4}`
- core fields non-empty across all 32: `True`
- placeholder residue (`待补` / `TODO` / `TBD` / `template`): `True`
- teaching notes include story-function language: `True`
- SLG rows stay on visible strategy-shot syntax: `True`

## Prompt Safety

- raw workbook prompt-body label/IP hits: `32`
- normalized ingest prompt-body label/IP hits: `0`
- normalization rule 1: rewrite the opening line to `生成一个动漫分镜视频镜头。`
- normalization rule 2: rewrite `国漫战争` to `中式战争`
- normalization rule 3: rewrite `架空三国军政` to `架空古代军政`

## Counts After Planned Ingest

- golden_sample_library: `120 -> 152`
- golden_sample_field_coverage_rules: `5 -> 5`
- golden_sample_failure_mapping: `120 -> 152`
- golden_sample_repair_mapping: `120 -> 152`
- library_status summary after ingest: `{"official": 108, "reserve": 44}`
- sample_type summary after ingest: `{"single_shot": 72, "sequence_shot": 80}`
- quality_grade summary after ingest: `{"优": 54, "差": 11, "中": 54, "好": 33}`
- usable_for_fewshot summary after ingest: `{"Yes": 97, "No": 55}`

## Dry-Run Result

- raw workbook structure passes items 1-5 and 7-9.
- raw workbook `prompt_body` text needs deterministic safety normalization for item 6 before seed write.
- with the normalization above applied only to the new 32 `prompt_body` values, the ingest is ready to proceed.

## Raw Prompt Hits

- `GS120-CAND-001`: `三国, 国漫`
- `GS120-CAND-002`: `三国, 国漫`
- `GS120-CAND-003`: `三国, 国漫`
- `GS120-CAND-004`: `三国, 国漫`
- `GS120-CAND-005`: `三国, 国漫`
- `GS120-CAND-006`: `三国, 国漫`
- `GS120-CAND-007`: `三国, 国漫`
- `GS120-CAND-008`: `三国, 国漫`
- `GS120-CAND-009`: `三国, 国漫`
- `GS120-CAND-010`: `三国, 国漫`
- `GS120-CAND-011`: `三国, 国漫`
- `GS120-CAND-012`: `三国, 国漫`
- `GS120-CAND-013`: `三国, 国漫`
- `GS120-CAND-014`: `三国, 国漫`
- `GS120-CAND-015`: `三国, 国漫`
- `GS120-CAND-016`: `三国, 国漫`
