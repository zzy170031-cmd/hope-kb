# Visual Reference Evidence Matrix Boundary v0.2

Status: boundary contract.
Scope: desktop `图片` and `图片2` visual reference folders for storyboard prompt knowledge intake.

This document defines how image-folder references may become Hope KB knowledge and how that knowledge must stay consumable by `hope-web-pwa`. It does not apply the references to the runtime snapshot by itself.

## Core Decision

The visual reference evidence matrix is a KB artifact, but it is not allowed to be a separate knowledge product. Every reviewed matrix entry that claims runtime value must map to the same consumption path used by the PWA adapter:

```text
visual reference -> canonical evidence -> reviewed rule -> wiki-to-runtime mapping -> runtime-kb-snapshot -> PWA adapter -> pwa_snapshot -> PWA storyboard fields
```

A matrix entry that cannot name its runtime rule pack, PWA snapshot target, and PWA storyboard field is not runtime-eligible.

## Layer Boundary

| layer | allowed content | blocked content |
| --- | --- | --- |
| raw evidence | opaque image id, folder label, content hash, visual summary, duplicate group, review notes | direct PWA consumption |
| evidence matrix | extracted rule point, PWA field mapping, positive directive, negative constraint, runtime projection, confidence, review status | copied screenshot text as prompt output |
| reviewed KB | normalized rule, applies-to actions, field mapping, leakage constraints, reviewed status | unreviewed OCR, local path, tutorial wording |
| runtime snapshot | rule packs, scene mappings, influence axes, negative constraints, selected rule summaries | image paths, filenames, OCR text, source screenshots, evidence row ids |
| PWA output | shot-local visual and camera guidance rewritten for the current story | copied reference wording, source claims, matrix ids, local paths |

## Duplicate Boundary

Deduplication must run before rule extraction.

1. Compute a content hash for every image file.
2. Create one canonical evidence item per exact hash group.
3. Record exact duplicates as `duplicate_of` and keep duplicate filenames only in raw evidence metadata.
4. Flag likely near-duplicates separately when images appear visually equivalent but not byte-identical.
5. Do not count duplicate images as independent rule evidence.
6. Many duplicate copies do not increase confidence by themselves.

Initial desktop observation:

- `图片`: 113 PNG files, mostly 1290x2796, with exact duplicate groups observed.
- `图片2`: 15 PNG files, 1290x2796.

## Required Runtime Projection

Every runtime-eligible matrix entry must include all of these bindings:

| required binding | purpose |
| --- | --- |
| `reviewed_wiki_id` | connects matrix entry to reviewed KB wiki |
| `mapping_id` | connects reviewed wiki to `wiki-to-runtime-mapping` |
| `runtime_rule_pack_id` | rule pack that will appear in `runtime-kb-snapshot` |
| `runtime_rule_group` | writing, director, validation, or future QA group |
| `runtime_targets` | KB snapshot targets such as `director_rule_packs`, `scene_mappings`, `negative_constraints` |
| `pwa_snapshot_paths` | exact PWA adapter output paths such as `pwa_snapshot.directorRulePacks[].directives` |
| `pwa_storyboard_fields` | actual PWA output fields served, such as `camera` or `visual_description` |
| `consumed_by_actions` | product actions where the rule can affect generation |

If any binding is missing, the row may remain evidence but must not enter the runtime snapshot.

## PWA Field Mapping

Rules may map only to existing PWA fields unless a separate product change is approved.

| PWA field | allowed reference influence |
| --- | --- |
| `camera` | shot function, camera angle, movement, lens feeling, adjacent-shot relation |
| `shot_size` | executable scale such as wide, medium, close-up, insert, detail |
| `visual_description` | visible subject placement, environment path, material, light, air, composition focus |
| `character_action` | visible emotion as breath, posture, gaze, hand, step, spacing, reaction |
| `dialogue_or_narration` | only story-supported spoken or narration content |
| `prompt_text` | compact current-shot packaging of subject, frame, action, camera, scale, duration, rhythm, style, constraints |
| `note` | reviewer-safe status or production note, not source provenance |

## Translation Rule

References are advisory. Runtime output must be generated from the current user story and accepted body, then guided by KB summaries.

Allowed transformation:

```text
image reference -> canonical evidence -> extracted principle -> reviewed rule -> sanitized runtime directive -> current-shot rewrite
```

Blocked transformation:

```text
image reference -> copied text -> PWA prompt output
```

## Runtime Eligibility Gates

A matrix row is runtime-eligible only when all conditions hold:

- exact duplicates have been grouped
- near-duplicate risk has been reviewed or marked
- the extracted rule is rewritten in original wording
- the rule maps to an existing PWA field
- the row has a positive directive and a negative constraint
- the row has a runtime projection into `runtime-kb-snapshot` and `pwa_snapshot`
- the row has `allowed_in_prompt_text` explicitly set
- blocked source content is absent from runtime fields
- review status is `reviewed`

## Adapter Requirement

The PWA adapter may include only summarized rule packs and mappings. It must reject or strip:

- local paths
- image filenames
- raw OCR
- screenshot text
- source captions
- tutorial source wording
- evidence row ids
- duplicate group ids
- raw source notes

If stripping cannot be proven, the adapter must fail closed and keep the last verified PWA snapshot.
## Product Stability Boundary

The PWA architecture is treated as stable for this work package. Visual,
camera, micro-expression, and scene-description knowledge exists to improve
product output quality, not to redesign the product.

The matrix must not require any of these PWA changes:

- new storyboard row fields
- new generation stages
- replacement of writing, director, storyboard, validation, or export flow
- direct runtime ingestion of raw image evidence
- prompt generation outside the existing KB runtime logic
- bypassing accepted-body fact priority

New knowledge may enter only as rewritten, reviewed, summary-level directives
that map to current PWA fields and current runtime rule packs. The output must
remain governed by the existing Hope production chain:

```text
accepted body -> writing guidance -> director guidance -> storyboard rows
-> prompt_text compilation -> validation -> export
```

Knowledge expansion is therefore an experience-quality upgrade, not an
architecture migration.
