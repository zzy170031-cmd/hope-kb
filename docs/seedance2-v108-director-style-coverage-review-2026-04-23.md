# Seedance2 V108 Director Style Coverage Review 2026-04-23

## Route

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `b9660be`
- package type: KB-only docs coverage review

This review does not modify `E:\codex\hope`, V3, desktop, intake, Qwen,
Seedance, product import behavior, seed rows, manifests, snapshots, few-shot
promotion rules, or `reference_control_core`.

## Reviewed Inputs

- `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`
- `docs/seedance2-v108-field-mapping-review-2026-04-23.md`
- `docs/seedance2-v108-v3-alignment-report-2026-04-23.md`
- `seed/v0.1/director_profiles.json`
- `seed/v0.1/director_rules.json`
- `seed/v0.1/director_scene_affinity.json`
- `E:\codex\hope\docs\generation-field-rule-contract-candidate-2026-04-22.md`

## Source Shape

The normalized V108 staging artifact contains 115 rows.

- `official = 108`
- `reserve = 7`
- `single_shot = 79`
- `sequence_shot = 36`
- `ready_for_positive_fewshot = 0`

Every row remains staging / evidence only. No row is imported into seed and no
row is promoted into positive few-shot.

## V108 Style Coverage Matrix

| style_cluster | rows | official | reserve | ready_for_positive_fewshot | scene_category distribution | current 7-director coverage |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `基础迁移` | 40 | 40 | 0 | 0 | `情绪对话=19`, `其他=8`, `相遇表演=7`, `场域追逐=5`, `热血打斗=1` | mostly covered by current emotion, daily micro-performance, transition, and generic action lanes |
| `日漫` | 25 | 22 | 3 | 0 | `群像表演=11`, `热血打斗=10`, `场域追逐=4` | mostly covered by current action burst, close combat, crowd pressure, and scene lanes |
| `国漫` | 29 | 26 | 3 | 0 | `热血打斗=11`, `场域追逐=10`, `群像表演=7`, `其他=1` | partially covered; current lanes lack guofeng / wuxia / xianxia action grammar, war-ritual blocking, and urban-apocalypse industrial pressure metadata |
| `美漫` | 13 | 12 | 1 | 0 | `场域追逐=5`, `其他=4`, `热血打斗=3`, `群像表演=1` | partially covered by action and crowd-pressure lanes; not the main new prototype target for this review |
| `原创` | 8 | 8 | 0 | 0 | `群像表演=8` | under-covered; current daily micro-performance and crowd lanes do not encode stage / guochao performance blocking as a retrieval style |

Scene-category totals:

| scene_category | rows | style distribution |
| --- | ---: | --- |
| `群像表演` | 27 | `日漫=11`, `原创=8`, `国漫=7`, `美漫=1` |
| `热血打斗` | 25 | `国漫=11`, `日漫=10`, `美漫=3`, `基础迁移=1` |
| `场域追逐` | 24 | `国漫=10`, `基础迁移=5`, `美漫=5`, `日漫=4` |
| `情绪对话` | 19 | `基础迁移=19` |
| `其他` | 13 | `基础迁移=8`, `美漫=4`, `国漫=1` |
| `相遇表演` | 7 | `基础迁移=7` |

## Camera-Directing Coverage Read

`camera_directing_core` exists on every normalized row, but it remains a source
field in staging. It is not imported as a top-level seed field. Fifteen rows are
titled as direct camera / camera-core examples, and 36 sequence rows also carry
camera-purpose text.

Current 7-director coverage is enough for generic camera constraints:

- action burst, close combat, chase axis, crowd pressure, emotional stillness,
  daily micro-performance, subjective transition, and suspense/match-cut routing
  all have an existing internal lane.

The gap is not basic camera literacy. The gap is style-cluster-specific camera
selection:

- `国漫 / 热血打斗` rows need bamboo-rain, blade / scabbard, robe-motion, cold-warm
  guofeng light, roofline, and lightness-action continuity as routing metadata.
- `国漫 / 场域追逐` rows need roof eaves, qinggong-style verticality, high架 /
  vehicle-top pursuit, and jump-escape beats.
- `国漫 / 群像表演` rows need fortress / oath / army formation / charge staging,
  not only generic crowd pressure.
- `原创 / 群像表演` rows need stage lighting, synchronized group dance, performer
  entrance, blackout, music beat, and final freeze blocking.
- `末世 / 地铁 / 高架 / 废墟` rows cut across `国漫` and `美漫`; the current
  pressure/suspense lanes can route danger, but they do not encode industrial
  collapse, evacuation, alarm, or infrastructure-scale oppression as a style
  selector.

## Current Seven-Director Insufficiency

The current seven-director fusion model should remain the default background
model. It already covers broad action, crowd pressure, close combat, emotion
landscape, daily micro-performance, subjective deformation, and suspense /
transition behavior.

Coverage is insufficient in four places:

- The current scene affinity taxonomy is generic: `战斗`, `追逐`, `群像`, `日常`,
  `抒情`, `心理`, `转场`, and `爆发` can classify V108 rows, but cannot preserve
  why `国漫 / 武侠轻功` should not route the same way as generic action chase.
- Existing action lanes can emphasize speed, impact, and axis clarity, but they
  do not carry guofeng spatial cues such as roof eaves, bamboo forest, robe /
  scabbard continuity, lightness movement, and ritualized pause-before-burst.
- Existing crowd-pressure lanes can handle formation and crisis, but they do not
  distinguish war-oath / army-charge epic staging from modern panic crowding.
- Existing micro-performance and emotion lanes can cover individual acting, but
  not stage choreography, spotlight timing, synchronized dance, music-synced
  entrances, or group freeze as a camera-language family.

## Recommendation

Add four future `style prototype` candidates only as retrieval and
camera-language selection metadata. Do not write them into seed in this gate.

| candidate prototype | evidence in V108 | gap solved | allowed future use |
| --- | --- | --- | --- |
| `国风武侠 / 仙侠动作调度` | `国漫=29`; especially `热血打斗=11` and `场域追逐=10`; examples include bamboo rain, roof-eave chase, qinggong, blade / scabbard continuity | prevents wuxia / xianxia action from collapsing into generic fast action or close combat | internal retrieval filter and camera-language selector for vertical movement, pause-before-burst, robe / weapon continuity, guofeng light and spatial rhythm |
| `国漫史诗群像 / 战争场面调度` | `国漫 / 群像表演=7`; war / army / oath / battlefield rows appear in group and sequence material | prevents war-oath, fortress push-in, drum field, and army charge from collapsing into generic crowd pressure | internal retrieval filter and camera-language selector for formation depth, command hierarchy, crowd rhythm, oath staging, and battle-scale continuity |
| `都市末世 / 工业压迫调度` | apocalypse / subway / overpass / ruins / evacuation rows cut across `国漫` and `美漫`; `末世` keyword appears in 12 rows | prevents industrial-collapse scenes from routing only as suspense or chase | internal retrieval filter and camera-language selector for infrastructure pressure, alarm layering, evacuation blocking, debris continuity, and urban-collapse spatial compression |
| `国潮舞台 / 原创表演调度` | `原创=8`, all `群像表演`; stage keyword appears across stage and campus-performance rows | prevents stage / group dance / performance rows from collapsing into daily micro-performance or generic crowd | internal retrieval filter and camera-language selector for spotlight timing, synchronized motion, performer entrance, blackout, music beat, group freeze, and stage-axis continuity |

## Boundary

These candidates must not be used as visible prompt text, UI copy, desktop
taxonomy output, or instruction to imitate a specific real director. They are
only planning names for future internal retrieval and camera-language selection.

This review does not authorize:

- seed import of V108 rows
- manifest or snapshot changes
- V3 proposal edits
- Hope product import changes
- runtime writer / validator / repair implementation
- Qwen or Seedance calls
- positive few-shot promotion
- `reference_control_core` creation or backfill

## Decision

`V108_DIRECTOR_STYLE_COVERAGE_REVIEW_DOCS_ONLY_COMPLETE`

Recommendation: add four bounded Chinese-animation / performance style
prototype candidates in a future explicit KB gate, if main control accepts this
coverage review. Until then, V108 remains normalized staging and gap evidence
only.
