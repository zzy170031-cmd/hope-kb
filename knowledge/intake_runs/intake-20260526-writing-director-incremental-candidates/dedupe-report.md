# Writing And Director Incremental Candidate Dedupe Report

Status: pending draft only.

Scope: sanitized graph handoff candidates for the product translation chain:

```text
film language
-> anime storyboard language
-> AI anime prompt language
-> PWA final fields
-> Qwen constrained text generation
```

No confirmed package was created. No apply was run. No reviewed_wiki, mapping,
runtime snapshot, PWA file, or first-round visual-rule package was modified.

## Result Summary

- received candidates: 22
- suggested KB-entry candidates: 18
- FutureQA inputs: 4
- merge_existing: 16
- new_reviewed_wiki_candidate: 2
- reject_runtime: 0
- future_qa: 4
- report_only: 0
- confirmation queue items: 14
- apply status: not executed
- PWA sync status: not executed

## Anime Translation Value Distribution

- high: 14
- medium: 4
- low: 0
- none: 4

High means the candidate directly helps convert film language into anime
storyboard fields and AI anime prompt constraints. Medium means the candidate
helps the anime route only under field-bound validation. None means the item
does not safely serve the product translation chain and stays out of runtime.

## Existing Coverage Used

- `rw-scene-expression-visible-action`
- `rw-expression-physicalization`
- `rw-director-scheduling-core`
- `rw-camera-language-grammar`
- `rw-shot-intent-taxonomy`
- `rw-transition-motion-dynamics`
- `rw-duration-density-rules`
- `rw-prompt-text-boundary`
- `rw-validation-no-leakage`
- `rw-person-identity-lock`
- `rw-visual-master-consistency`
- `rw-seedance2-storyboard-field-discipline`
- `rw-writing-continuity-core` as supporting background only; it is not a new confirmation target in this package.

## Candidate Conclusions

| # | sanitized candidate | anime_translation_value | conclusion | primary group | target |
| --- | --- | --- | --- | --- | --- |
| 1 | 场景目标与冲突推进 | high | merge_existing | writing_group | `rw-scene-expression-visible-action` |
| 2 | 对白依据锁 | high | new_reviewed_wiki_candidate | validation_group | `rw-dialogue-evidence-lock` |
| 3 | 行动替代说明 | high | merge_existing | writing_group | `rw-scene-expression-visible-action` |
| 4 | 人物动机到动作链 | high | merge_existing | writing_group | `rw-scene-expression-visible-action` |
| 5 | blocking 站位关系 | high | merge_existing | director_group | `rw-director-scheduling-core` |
| 6 | 前中后景层次 | high | merge_existing | director_group | `rw-visual-master-consistency`, `rw-camera-language-grammar` |
| 7 | 视线关系与轴线稳定 | high | merge_existing | director_group | `rw-camera-language-grammar`, `rw-director-scheduling-core` |
| 8 | 主运镜叙事目的 | high | merge_existing | director_group | `rw-camera-language-grammar`, `rw-shot-intent-taxonomy` |
| 9 | 反应镜头密度 | high | merge_existing | director_group | `rw-shot-intent-taxonomy`, `rw-duration-density-rules`, `rw-director-scheduling-core` |
| 10 | 动作剪辑匹配 | high | merge_existing | director_group | `rw-transition-motion-dynamics` |
| 11 | 时长与信息密度 | medium | merge_existing | director_group | `rw-duration-density-rules` |
| 12 | 预备动作-主动作-跟随动作 | high | merge_existing | director_group | `rw-expression-physicalization`, `rw-transition-motion-dynamics` |
| 13 | 关键姿势与动作轮廓 | high | merge_existing | director_group | `rw-expression-physicalization`, `rw-seedance2-storyboard-field-discipline` |
| 14 | 群像动作分层 | high | new_reviewed_wiki_candidate | director_group | `rw-ensemble-action-layering` |
| 15 | 文生视频字段顺序 | high | merge_existing | validation_group | `rw-seedance2-storyboard-field-discipline`, `rw-prompt-text-boundary` |
| 16 | 负面约束简化 | medium | merge_existing | validation_group | `rw-prompt-text-boundary`, `rw-validation-no-leakage` |
| 17 | 字段混写修复 | medium | merge_existing | validation_group | `rw-seedance2-storyboard-field-discipline`, `rw-person-identity-lock`, `rw-prompt-text-boundary` |
| 18 | 事实优先修复 | medium | merge_existing | validation_group | `rw-scene-expression-visible-action`, `rw-person-identity-lock`, `rw-validation-no-leakage` |
| F1 | 真实导演/IP 风格开关 | none | future_qa | director_group | blocked before runtime |
| F2 | 自动对白生成模板 | none | future_qa | writing_group | blocked before runtime |
| F3 | 长 prompt 万能模板 | none | future_qa | validation_group | blocked before runtime |
| F4 | 未去源的失败案例库 | none | future_qa | validation_group | blocked before runtime |

## High Value Candidates

- 场景目标与冲突推进: writing rule that converts conflict into anime visible beats.
- 对白依据锁: validation-first new candidate; prevents unsupported dialogue/narration basis.
- 行动替代说明: converts exposition into anime action, reaction, or visual relation.
- 人物动机到动作链: binds motivation to posture, route, reaction, and consequence.
- blocking 站位关系: converts film blocking into anime spacing, silhouette, and pressure.
- 前中后景层次: converts film depth into anime frame layers and subject readability.
- 视线关系与轴线稳定: preserves anime eyeline and screen-direction continuity.
- 主运镜叙事目的: converts camera movement into anime main movement, speed feeling, and background flow.
- 反应镜头密度: converts reaction editing into anime reaction cuts, pauses, and impact emphasis.
- 动作剪辑匹配: converts film match cuts into anime action continuity and impact frame logic.
- 预备动作-主动作-跟随动作: converts motion phases into anime anticipation and follow-through.
- 关键姿势与动作轮廓: converts performance into anime key pose, silhouette, and center of gravity.
- 群像动作分层: director-first new candidate for anime ensemble layering with no entity invention.
- 文生视频字段顺序: converts anime storyboard language into field-bound AI anime prompt order.

## Medium Value Candidates

- 时长与信息密度: useful for anime pacing, but must stay bound to duration validation and cannot drive style by itself.
- 负面约束简化: useful for prompt safety, but it is validation support, not creative anime direction.
- 字段混写修复: useful for anime field repair, but it mainly blocks pollution and overloading.
- 事实优先修复: useful for repair safety, but it is a general fact-preservation guard.

## Low Or None Downgrades

- low: none in this pass.
- none: 真实导演/IP 风格开关. Reason: proper names and IPs cannot become runtime style switches.
- none: 自动对白生成模板. Reason: it encourages unsupported dialogue invention and must not override dialogue evidence lock.
- none: 长 prompt 万能模板. Reason: it conflicts with field-bound anime prompt compilation and risks prompt overload.
- none: 未去源的失败案例库. Reason: it is not sanitized enough for runtime or adapter downlink.

## Translation Chain Checks

All runtime-eligible candidates in this draft were checked against:

- film language input;
- anime storyboard conversion;
- AI anime prompt conversion;
- impacted final PWA fields;
- whether Qwen receives constraints for anime storyboard text rather than generic live-action film text;
- summary-only transport;
- pollution risks around proper names, IPs, invented facts, fabricated dialogue, and field mixing.

PWA final fields remain fixed:

- 人物
- 运镜
- 景别
- 画面描述
- 角色动作
- 对白/旁白
- 分镜提示词
- 时长(秒)

序号 stays PWA-controlled. 操作 stays UI-controlled. note/status stays internal
validation or repair state only and is not a final generated field.

## Group Routing Summary

- writing_group: candidates 1, 3, 4; supports candidates 2, 9, 12, 14, 18.
- director_group: candidates 5, 6, 7, 8, 9, 10, 11, 12, 13, 14; supports candidates 15 and 17.
- validation_group: candidates 2, 15, 16, 17, 18; supports all runtime-eligible director/writing candidates where field boundary, leakage, fact invention, dialogue basis, person identity, or anime prompt overload is involved.
- pwa_adapter_group: supports candidates 15, 16, 17 and FutureQA F3/F4 only as summary-only downlink governance; it is not a creative rule group and does not receive raw source.

## Confirmation Queue

Recommended follow-up user confirmation targets:

- `rw-scene-expression-visible-action`
- `rw-dialogue-evidence-lock`
- `rw-director-scheduling-core`
- `rw-camera-language-grammar`
- `rw-shot-intent-taxonomy`
- `rw-transition-motion-dynamics`
- `rw-duration-density-rules`
- `rw-expression-physicalization`
- `rw-ensemble-action-layering`
- `rw-visual-master-consistency`
- `rw-seedance2-storyboard-field-discipline`
- `rw-prompt-text-boundary`
- `rw-validation-no-leakage`
- `rw-person-identity-lock`

The two new candidates are:

- `rw-dialogue-evidence-lock`: validation-first; prevents unsupported dialogue or narration basis and must not become an automatic dialogue template.
- `rw-ensemble-action-layering`: director-first; allows anime group action layering only inside accepted facts and must not invent count, faction, or new characters.

## FutureQA And Runtime Blocks

- 真实导演/IP 风格开关: FutureQA only until de-named and abstracted; real names cannot become runtime style switches.
- 自动对白生成模板: FutureQA only; may inform validation examples, but must not become automatic unsupported dialogue generation.
- 长 prompt 万能模板: FutureQA only; conflicts with field-bound summary-only prompt compilation.
- 未去源的失败案例库: FutureQA only; blocked until fully sanitized and summarized.

## Draft Package

- `knowledge/intake_runs/intake-20260526-writing-director-incremental-candidates/intake-package.draft.json`
- confirmation state: false
- KB application state: false
- apply gate: closed
- PWA sync: not executed
