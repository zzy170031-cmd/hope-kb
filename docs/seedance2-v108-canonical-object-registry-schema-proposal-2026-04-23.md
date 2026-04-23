# Seedance2 V108 Canonical Object Registry Schema Proposal 2026-04-23

## Route

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- starting anchor: `8cd8335`
- Hope control anchor: `22162ea`
- package type: KB-only docs schema proposal

This proposal is schema planning only. It does not create registry rows,
canonical object names, product-ready external reference handles, image URLs,
asset IDs, character appearances, scene designs, V108 imports, positive
few-shot promotions, or `reference_control_core`.

## Reviewed Inputs

- `E:\codex\hope\docs\seedance2-v108-canonical-object-registry-schema-proposal-dispatch-2026-04-23.md`
- `E:\codex\hope\docs\seedance2-v108-canonical-object-registry-readiness-control-review-2026-04-23.md`
- `docs/seedance2-v108-canonical-object-registry-readiness-2026-04-23.md`
- `docs/seedance2-v108-external-reference-handles-canonical-name-review-2026-04-23.md`
- `docs/seedance2-v108-field-mapping-review-2026-04-23.md`
- `docs/seedance2-v108-v3-alignment-report-2026-04-23.md`
- `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`
- `seed/v0.1/source_register.json`
- `seed/v0.2/source_register.json`
- `seed/v0.2/import_map.json`

## Evidence Baseline

| evidence surface | count |
| --- | ---: |
| total normalized V108 rows | 115 |
| rows with `reference_bundle` | 115 |
| `reference_bundle` placeholder rows | 102 |
| placeholder-free `reference_bundle` rows | 13 |
| `reference_handle_normalization_needed` | 115 |
| `reference_handle_needs_canonical_name` | 75 |
| rows with candidate stems | 40 |
| unique candidate stems | 37 |
| parsed reference parts | 349 |
| parsed reference parts with placeholder text | 246 |
| parsed reference parts without placeholder text | 103 |
| product-ready external reference handles | 0 |
| rows ready for positive few-shot | 0 |
| `reference_control_core` coverage | 0 |

V108 candidate stems remain non-canonical evidence. They are not object names,
not handles, not source object rows, not prompt text, and not registry data.

## Schema Planning Objects

The following names are planning names only:

- `CanonicalObjectRegistrySchema`
- `CanonicalObjectRecord`
- `CanonicalObjectCategory`
- `CanonicalObjectAliasRule`
- `CanonicalObjectProvenance`
- `CanonicalObjectUsePolicy`
- `CanonicalObjectBlocker`
- `CanonicalObjectImportMapRequirement`
- `CanonicalObjectManifestRequirement`
- `CanonicalObjectReviewStatus`

These names are not seed files, JSON records, database tables, Rust DTOs, IPC
payloads, workbook sheets, UI taxonomy, validator enums, Qwen prompts, or
Seedance prompt structures in this gate.

## CanonicalObjectRegistrySchema

`CanonicalObjectRegistrySchema` is the future container definition for object
records and their governance rules.

It must preserve these boundaries:

- It can represent only accepted object names supplied by a later source gate.
- It must not infer object names from V108 sample titles, prompt prose, media
  labels, candidate stems, or old V3 assumptions.
- It must keep raw V108 `reference_bundle` values as source evidence.
- It must keep product-facing handles closed until a Hope-side reference handle
  contract accepts how registry rows become `external_reference_handles`.

## CanonicalObjectCategory

Allowed object categories are planning names only:

```text
character
scene
prop
style
continuity_object
```

Category meaning:

| category | planning meaning | direct V108 promotion allowed now |
| --- | --- | --- |
| `character` | named story/person/role object | no |
| `scene` | named location or recurring scene object | no |
| `prop` | named prop or object with continuity meaning | no |
| `style` | named style reference approved by a later registry gate | no |
| `continuity_object` | object whose identity or position must persist across shots | no |

No current V108 candidate stem is accepted into any category.

## CanonicalObjectRecord

Future `CanonicalObjectRecord` fields, planning-only:

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

Field rules:

- `canonical_object_id`: stable registry key assigned by a later registry seed
  package, not by this proposal.
- `canonical_object_name`: human-readable object name accepted from a later
  source gate, never invented from V108 candidate stems.
- `object_category`: one of the five planning categories above.
- `source_aliases`: raw aliases, stems, labels, or source phrases that may map
  to an accepted object name after review.
- `source_row_refs`: V108 source row IDs or source row references that justify
  why an alias was reviewed.
- `source_register_refs`: source register IDs and provenance references.
- `ip_abstraction_status`: planning status for whether the object is safe,
  abstracted, blocked, or needs legal / main-control review.
- `allowed_reference_use`: what a future system may do with the object name.
- `blocked_reference_use`: what a future system must not do with the object.
- `continuity_notes`: constraints for preserving object identity, position,
  relation, or continuity.
- `registry_provenance`: evidence trail from source registers and accepted
  review documents.
- `registry_review_status`: review lifecycle state before any product use.

## Alias Rule

`CanonicalObjectAliasRule` separates aliases from canonical names:

- Alias: source wording or candidate stem that may point toward an object, such
  as a media descriptor, scene/action label, raw stem, or row-local phrase.
- Canonical object name: accepted object label supplied by a later registry gate
  and reviewed for category, provenance, IP abstraction, and allowed use.

Alias rules:

- aliases can never become product-ready handles by direct copy
- aliases can never create `reference_control_core`
- aliases can be many-to-one after review
- aliases can remain unresolved forever without blocking raw source retention
- placeholder aliases must preserve `placeholder_present`
- ambiguous aliases must preserve `ambiguous_source_name`
- aliases from `prompt_body` remain prompt text evidence, not registry fields

V108 candidate stems remain non-canonical because they are frequently scene,
action, media, or style descriptors rather than accepted story objects. Examples
include chase, group dance, stage light, character face, roof pursuit, subway
power loss, and battlefield charge descriptors.

## Provenance Rule

`CanonicalObjectProvenance` must record provenance without deriving names from
prompt prose.

Allowed provenance inputs:

- `seed/v0.2/source_register.json` source IDs for V108 raw XLSX, DOCX,
  normalized staging, field mapping review, and V3 alignment report
- row references from normalized staging
- accepted main-control review documents
- future operator-provided object registries or story setup sources

Blocked provenance shortcuts:

- no generated prompt text as source of truth
- no invented image, video, audio, URL, file path, or asset ID
- no object name created from sample title alone
- no object name created from a candidate stem without review
- no old V3 `reference_control_core` or asset registry assumption as current
  source truth

## Use Policy

`CanonicalObjectUsePolicy` must define allowed and blocked use before any
product integration.

Allowed future use, after later gates:

- internal retrieval filtering by accepted object name
- continuity preservation by accepted object name
- product-facing `external_reference_handles` only after Hope-side acceptance
- validator evidence that an accepted object was referenced consistently

Blocked use:

- expanding a character name into appearance, clothing, age, face, or body
  description
- expanding a scene name into full art direction
- adding URLs, asset IDs, image paths, or file references
- appending raw `reference_bundle` material into prompt text
- using registry data as positive few-shot promotion
- creating or filling `reference_control_core`

## Blocker Rule

`CanonicalObjectBlocker` planning names:

```text
placeholder_present
ambiguous_source_name
ip_abstraction_unresolved
source_provenance_missing
canonical_name_missing
alias_unreviewed
schema_not_accepted
import_map_missing
manifest_hash_path_missing
snapshot_gate_missing
v3_alignment_gap
reference_control_core_closed
product_handle_gate_not_accepted
```

Blocker rules:

- `placeholder_present` blocks object row creation from that source phrase.
- `ambiguous_source_name` blocks product-ready handles until a reviewer maps or
  rejects the alias.
- `ip_abstraction_unresolved` blocks appearance / scene-design expansion.
- `source_provenance_missing` blocks registry row acceptance.
- `canonical_name_missing` blocks `external_reference_handles`.
- `reference_control_core_closed` stays true until a separate coverage gate is
  accepted.

## Review Status

`CanonicalObjectReviewStatus` planning values:

```text
draft_schema_only
pending_source_names
pending_alias_review
pending_ip_review
accepted_for_registry_seed
rejected_alias
blocked_placeholder
blocked_provenance
blocked_product_use
```

Current package status:

```text
draft_schema_only
```

No source object name is selected in this gate.

## Import Map Requirement

`CanonicalObjectImportMapRequirement` for a later implementation gate:

- add a new registry seed file path only after schema acceptance
- add a dedicated import map entry for object records
- add a dedicated import map entry for provenance or alias rows if separated
- define table names, JSON path, record count keys, and primary key fields
- keep existing v0.2 import map entries unchanged unless main control opens an
  implementation package

Current `seed/v0.2/import_map.json` has no canonical object registry entry.
This proposal does not change it.

## Manifest Requirement

`CanonicalObjectManifestRequirement` for a later implementation gate:

- add registry seed files to v0.2 `bundle_order`
- add `record_counts` for object records and optional alias/provenance rows
- recompute `content_hash`
- document hash algorithm and source preservation contract
- rebuild a versioned snapshot only after seed/import_map/manifest changes are
  explicitly authorized

Current v0.2 manifest / hash path remains unchanged in this proposal.

## Snapshot And Content Hash Gate

A later snapshot gate must prove:

- object record counts match manifest counts
- aliases do not duplicate canonical IDs incorrectly
- provenance references point to accepted source IDs
- blocker rows remain inspectable
- SQLite `quick_check` passes
- product-ready external handle count remains 0 unless a product handle gate has
  separately accepted promotion

This proposal does not run a snapshot build because no seed, import map,
manifest, migration, or snapshot input changed.

## `reference_control_core` Boundary

`reference_control_core` remains closed.

This schema proposal does not define `reference_control_core`, does not create a
field for it, and does not allow it to be inferred from:

- raw `reference_bundle`
- candidate stems
- accepted style lanes
- prompt text
- old V3 reference fields
- future canonical object names

A separate coverage gate must define source evidence, schema, validator,
product behavior, and repair boundaries before `reference_control_core` can be
considered.

## Future Gate Needed Before Registry Rows

The next implementation-facing gate must be a bounded KB registry seed package
opened by main control. It must name exact files and provide accepted source
object names from one of:

- project / story setup supplied by the operator
- an accepted external reference system export
- a main-control accepted object registry source package

That future gate must also define import_map, manifest, content hash, snapshot,
validation, and review packet requirements. Without accepted object names, no
registry rows can be added.

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

## Decision

`CANONICAL_OBJECT_REGISTRY_SCHEMA_PROPOSAL_DOCS_ONLY_COMPLETE`

This proposal is ready for main-control review as a schema candidate. It is not
ready for registry row creation, product handles, V108 import, positive
few-shot, Qwen / Seedance use, or `reference_control_core`.

## Validation

This package is docs-only. It changes no seed file, manifest, import map,
snapshot, normalized staging artifact, runtime file, product file, V3 file,
desktop file, or intake file. Code and seed validation were not required and
were not run.
