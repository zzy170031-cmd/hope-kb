# Seedance2 V108 Canonical Object Registry Readiness 2026-04-23

## Route

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- starting anchor: `434c74a`
- Hope control anchor: `91bdcfd`
- package type: KB-only docs readiness review

This is a readiness review only. It does not create a canonical object registry,
product-ready external reference handles, canonical object rows, image URLs,
asset IDs, character appearances, scene designs, or `reference_control_core`.

## Reviewed Inputs

Hope control inputs, read only:

- `E:\codex\hope\docs\seedance2-v108-canonical-object-registry-readiness-dispatch-2026-04-23.md`
- `E:\codex\hope\docs\seedance2-v108-shot-language-selection-contract-review-2026-04-23.md`
- `E:\codex\hope\docs\seedance2-v108-shot-language-selection-contract-candidate-2026-04-23.md`
- `E:\codex\hope\docs\seedance2-v108-validator-evidence-contract-review-2026-04-23.md`
- `E:\codex\hope\docs\seedance2-v108-vnext-schema-decision-2026-04-23.md`

KB inputs:

- `docs/seedance2-v108-import-dry-run-report-2026-04-23.md`
- `docs/seedance2-v108-field-mapping-review-2026-04-23.md`
- `docs/seedance2-v108-v3-alignment-report-2026-04-23.md`
- `docs/seedance2-v108-external-reference-handles-canonical-name-review-2026-04-23.md`
- `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`
- `seed/v0.1/source_register.json`
- `seed/v0.2/source_register.json`
- `seed/v0.2/import_map.json`

## V108 Reference Evidence Summary

The normalized V108 staging artifact still contains 115 rows and remains raw
source evidence only.

| evidence surface | count | readiness meaning |
| --- | ---: | --- |
| total normalized rows | 115 | accepted raw source rows for future dry-run review |
| rows with `reference_bundle` source field | 115 | all rows have a reference-related source surface |
| non-empty `reference_bundle` values | 115 | source values exist, but are not canonical objects |
| `reference_bundle` containing placeholder text | 102 | blocked by placeholder / future-fill surface |
| placeholder-free `reference_bundle` values | 13 | enough for review examples, not enough for product handles |
| `reference_handle_normalization_needed` | 115 | every row still needs normalization before any product use |
| `reference_handle_needs_canonical_name` | 75 | source labels are not accepted object names |
| rows with non-empty candidate stems | 40 | candidate evidence only |
| unique candidate stems | 37 | non-canonical; many are scene/action/media descriptors |
| parsed `reference_bundle` parts | 349 | media-label evidence, not registry rows |
| parsed parts with placeholder text | 246 | blocked |
| parsed parts without placeholder text | 103 | review evidence only |
| product-ready external reference handles | 0 | no current row can enter product reference context |
| `reference_control_core` coverage | 0 | absent and must not be inferred |

Reference-related source fields are broader than `reference_bundle`: all 115
rows also carry `ip_abstraction_note`, `covered_points`, `missed_points`,
`teaching_note`, and `continuity_negative_core` evidence. Those fields can help a
future registry proposal decide risk and provenance, but they do not create
object records.

## Boundary Definitions

| concept | current status | allowed use now | blocked use |
| --- | --- | --- | --- |
| raw `reference_bundle` evidence | source field under the V108 23-header surface | preserve as raw text and gap evidence | direct product use, direct prompt append, asset binding |
| canonical object name | absent | planning name for a future registry schema only | inventing names from titles, prompt prose, media descriptors, or candidate stems |
| product-ready `external_reference_handles` | absent, count is 0 | none in this gate | Qwen context, Seedance prompt, product import, UI/debug output |
| `reference_control_core` | uncovered, count is 0 | no current use | deriving from `reference_bundle`, old V3 assumptions, style lanes, or prompt text |

Future product-facing handles may only point to accepted object names from a
separate registry. They must not copy media labels such as image/video/audio
descriptors, file paths, URLs, asset IDs, or generic labels.

## Current Registry Readiness Risks

- Placeholder risk: 102 rows have placeholder-like `reference_bundle` content,
  and 246 parsed reference parts contain placeholder text.
- Ambiguous source-name risk: candidate stems include generic descriptors such
  as action, crowd, character-face, character-standing-image, chase, stage light,
  and scene-action labels. These are not stable object names.
- IP abstraction risk: all 115 rows carry `ip_abstraction_note`; these notes are
  compliance evidence, not permission to expand character appearance, scene
  design, or external media references.
- Missing registry schema: no accepted KB schema defines canonical object
  identity, category ownership, alias rules, provenance, or product visibility.
- Missing import map: `seed/v0.2/import_map.json` has only golden sample library,
  field coverage, failure mapping, repair mapping, source, and provenance
  entries. It has no canonical object registry table or registry provenance map.
- Missing manifest / hash path: current v0.2 manifest has no record counts,
  bundle order, or content hash path for registry rows.
- V3 alignment gap: V108 `reference_bundle` maps toward future
  `external_reference_handles`; old V3 `reference_control_core`, asset registry,
  image/video/audio ref-assets, and adapter assumptions remain historical
  references only.
- Prompt boundary risk: `prompt_body` may contain internal sections, but those
  sections remain content under `prompt_body` and cannot create reference fields
  or completed handles.

## Registry Readiness Checklist

A future canonical object registry gate is not ready to implement until all
items below are satisfied:

- An explicit docs-only registry schema proposal is accepted by main control.
- The schema defines object categories before any rows exist.
- Canonical object names are supplied by project/story setup or an accepted
  external reference system, not inferred from V108 rows.
- Alias handling is defined for raw candidate stems without treating aliases as
  canonical handles.
- IP / abstraction rules define what may be referenced and what must remain
  blocked.
- Provenance links each object name to an accepted source, not to generated
  prompt prose.
- Placeholder and unresolved-reference blockers are preserved.
- v0.2 import_map, manifest record counts, bundle hash path, and snapshot path
  are defined in a later implementation gate.
- Hope product, V3, desktop, intake, Qwen, Seedance, exporter, workbook, IPC,
  validator, and repair gates remain closed until main control opens them.

## Future Registry Object Fields

The following are planning names only. They are not seed fields, JSON rows,
database tables, Rust DTOs, workbook columns, UI labels, or product prompt
tokens in this gate.

```text
canonical_object_id
canonical_object_name
object_category
source_aliases
source_row_refs
source_register_refs
ip_abstraction_status
allowed_reference_use
blocked_reference_use
continuity_notes
registry_provenance
registry_review_status
```

Allowed future object categories remain planning-only:

```text
character
scene
prop
style
continuity_object
```

No current V108 candidate stem is promoted into any of those categories.

## Gate Order

1. Main control accepts or revises this readiness review.
2. If accepted, open a docs-only canonical object registry schema proposal.
3. Define object categories, alias rules, blocker semantics, provenance, and
   manifest/import-map needs without adding object rows.
4. Only after schema acceptance, open a bounded KB registry seed package with
   exact files and source object names supplied by an accepted source.
5. Rebuild v0.2 manifest / import map / snapshot only in that later registry
   package.
6. Open Hope-side reference handle contract review before any product-facing
   `external_reference_handles` can exist.
7. Keep `reference_control_core` closed until a separate coverage gate accepts
   its source, schema, validator, and product boundaries.
8. Keep Qwen, Seedance, desktop, intake, exporter, workbook, validator, repair,
   and runtime selection gates separate.

## Recommendation

`ONLY_READY_FOR_DOCS_ONLY_REGISTRY_SCHEMA_PROPOSAL`

V108 has enough raw reference evidence to justify a future docs-only canonical
object registry schema proposal. It is not ready for registry implementation,
product-ready external reference handles, V108 row import, positive few-shot
promotion, or `reference_control_core`.

## Required Assertions

```text
raw_source_rows_ready_for_future_dry_run = 115
rows_ready_for_product_import = 0
rows_ready_for_positive_fewshot = 0
product_ready_external_reference_handles = 0
reference_control_core_coverage = 0
qwen_calls = 0
seedance_calls = 0
hope_product_code_changes = 0
desktop_or_intake_changes = 0
v3_branch_edits = 0
rust_dto_validator_exporter_workbook_ipc_changes = 0
```

## Validation

This package is docs-only and changes no seed, manifest, import map, migration,
snapshot, runtime, product, V3, desktop, or intake file. Code and seed
validation were therefore not required and were not run.
