# Seedance2 V108 External Reference Handles Canonical Name Review 2026-04-23

This note records the KB-only canonical-name review for V108
`reference_bundle` to `external_reference_handles`.

## Scope

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `6f210f0`
- package type: KB-only reference-handle canonical-name review
- canonical source format: local Seedance2.0 V108 XLSX / DOCX table package
- source package: `golden-samples/seedance2-v108/`
- normalized staging source:
  `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`

This package does not modify `E:\codex\hope`, the V3 proposal branch, desktop,
intake, Qwen, Seedance, Hope product/runtime code, Rust DTOs, validators,
exporters, workbook contracts, or IPC. It does not merge `hope-kb` into
`hope`.

## Canonical Source Reminder

The V108 XLSX / DOCX field table is the current source of truth for this gate.
Old V3 documents are historical proposal / freeze-candidate references only.

The relevant V108 field is:

```text
reference_bundle
```

It must not be interpreted through the older V3 `reference_control_core`,
`asset_registry`, `image_ref_assets`, `video_ref_assets`, or `audio_ref_assets`
assumptions.

## Read-Only Evidence

Read-only evidence from the V108 normalized staging artifact:

- records reviewed: `115`
- official rows: `108`
- reserve rows: `7`
- official source `usable_for_fewshot=Yes`: `97`
- official source `usable_for_fewshot=No`: `11`
- `reference_handle_normalization_needed`: `115`
- prior staging flag `reference_handle_needs_canonical_name`: `75`
- rows whose raw `reference_bundle` contains `待补`: `102`
- rows whose raw `reference_bundle` contains no `待补`: `13`
- reference bundle parts parsed: `349`
- parsed parts with `待补`: `246`
- parsed parts without `待补`: `103`
- unique candidate stems extracted by staging: `37`

The existing staging layer also records:

- `positive_fewshot_candidate_after_placeholder_gate = 13`
- `ready_for_positive_fewshot = 0`
- `prompt_body_candidate_preserved = 13`
- `prompt_body_candidate_blocked_by_placeholder = 102`

This review does not promote any row into runtime positive few-shot.

## Decision

`EXTERNAL_REFERENCE_HANDLES_CANONICAL_NAME_REVIEW_COMPLETE_CANONICAL_NAMES_NOT_READY`

The KB review accepts `reference_bundle` only as raw source evidence for future
external-reference naming. No V108 row currently has product-ready canonical
`external_reference_handles`.

The reason is structural:

- The V108 source values are written as media descriptors such as
  `图片`, `视频`, and `音频`, sometimes with counts or material categories.
- Hope is a text-first storyboard and prompt workbench. It should receive
  explicit object names from story setup or an external reference system, not
  concrete file references.
- The current source does not provide a canonical object registry that says
  which exact character, scene, prop, style, or continuity object each media
  descriptor should bind to.

Therefore the 40 rows that already have extracted candidate stems are still
candidate evidence only. They are not accepted as final
`external_reference_handles`.

## Accepted Handle Semantics

Future product-facing `external_reference_handles` may contain only explicit
object names, grouped by semantic category.

Allowed future categories:

```text
character
scene
prop
style
continuity_object
```

Allowed value examples are name-like handles such as:

```text
character:男主阿澈
character:女主林夏
scene:雨夜公交站
prop:蓝色车票
style:冷暖双光源现实主义
continuity_object:左手伞柄
```

These examples are format examples only. They are not inserted into V108 and do
not describe any current source row.

## Rejected Direct Mappings

The following source patterns must not become final
`external_reference_handles` by direct copy:

- `图片：待补`
- `视频：待补`
- `音频：待补`
- `图片：海雾船首图2张`
- `视频：破浪参考视频1段`
- `音频：海浪风声音效`
- generic labels such as `主角面部`, `角色立绘`, or `黑白角色立绘`
- media counts such as `图2张`, `视频1段`, or `参考视频1段`
- file paths
- URLs
- asset IDs
- real media bindings

Audio descriptors remain source evidence for `audio_directing_core` or a future
sound-cue planning gate. They are not accepted as
`external_reference_handles` in this review.

## Candidate Stem Inventory

The staging artifact extracted these candidate stems. They remain review
evidence only and require canonical-name confirmation before any future import:

| candidate stem | count | current status |
| --- | ---: | --- |
| `追逐` | 4 | candidate only |
| `俯冲` | 2 | candidate only |
| `俯冲动作` | 2 | candidate only |
| `地铁断电` | 2 | candidate only |
| `城楼誓师` | 2 | candidate only |
| `天台逆光` | 2 | candidate only |
| `巷道追逐` | 2 | candidate only |
| `海雾船首` | 2 | candidate only |
| `破浪` | 2 | candidate only |
| `礼堂灯光` | 2 | candidate only |
| `结印` | 2 | candidate only |
| `结印手势` | 2 | candidate only |
| `群舞` | 2 | candidate only |
| `群舞动作` | 2 | candidate only |
| `车顶跳跃` | 2 | candidate only |
| `车顶追逐` | 2 | candidate only |
| `雨中对峙` | 2 | candidate only |
| `高架废墟` | 2 | candidate only |
| `主角面部` | 1 | candidate only; too generic |
| `乐队演奏` | 1 | candidate only |
| `冲阵` | 1 | candidate only |
| `屋檐追杀` | 1 | candidate only |
| `屋顶跳跃` | 1 | candidate only |
| `屋顶追逐` | 1 | candidate only |
| `战斗切镜` | 1 | candidate only |
| `拉伸动作` | 1 | candidate only |
| `甲板站位` | 1 | candidate only |
| `竹林雨夜` | 1 | candidate only |
| `竹林雨夜场景` | 1 | candidate only |
| `群像` | 1 | candidate only |
| `群像集结` | 1 | candidate only |
| `舞台聚光灯` | 1 | candidate only |
| `角色立绘` | 1 | candidate only; too generic |
| `踏竹动作` | 1 | candidate only |
| `轻功` | 1 | candidate only |
| `重拳动作` | 1 | candidate only |
| `黑白角色立绘` | 1 | candidate only; too generic |

## Normalization Rule For Future Gate

A later KB import or schema gate may normalize V108 reference material only if a
separate canonical object registry exists.

Required future registry shape:

```text
project_or_story_id
handle_category
canonical_handle_name
source_aliases
allowed_prompt_use
disallowed_prompt_use
continuity_notes
provenance
```

Required future rules:

- Use exact character, scene, prop, style, or continuity object names.
- Do not expand a character handle into appearance, clothing, age, face, or
  body description.
- Do not expand a scene handle into full environment art direction.
- Do not invent names from sample titles, media descriptors, or prompt prose.
- If an object name is missing, emit `unresolved_reference_handle`.
- If a field is blank or `待补`, preserve `future_model_fill_surface`.
- Keep `reference_control_core` absent until a separate coverage gate exists.

## Prompt Boundary

When Hope later writes storyboard or shot prompt text, it may write:

- action
- expression
- tone
- camera movement
- lighting relation
- rhythm
- continuity relation

It must not use this review to write:

- concrete image references
- URLs
- asset IDs
- generated appearance descriptions for named characters
- generated scene-design descriptions for named scenes
- completed `reference_control_core`

The prompt should refer to already-known objects by name after a future gate
accepts those names.

## Effect On Current V108 Package

No current V108 source row is promoted by this review.

Current status after this review:

- raw V108 source package: accepted as immutable KB source
- normalized staging: accepted as review evidence
- candidate stems: retained as non-canonical evidence
- product-ready `external_reference_handles`: `0`
- runtime positive few-shot promotion: `0`
- `reference_control_core` coverage: `0`

## Next Gate

The next eligible gate is a KB-only canonical object registry package, if the
operator provides story/person/scene/prop/style names or an external reference
system export.

Without that registry, the project may continue to vNext schema planning, but
schema planning must preserve the status here: V108 `reference_bundle` is source
evidence only, not final `external_reference_handles`.
