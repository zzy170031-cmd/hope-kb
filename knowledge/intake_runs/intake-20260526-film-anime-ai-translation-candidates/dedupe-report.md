# Film Anime AI Translation Candidate Dedupe Report

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
runtime snapshot, PWA file, first-round visual-rule package, or second-round
writing-director package was modified.

## Result Summary

- received candidates: 18
- suggested KB-entry candidates: 14
- FutureQA inputs: 4
- merge_existing: 11
- new_reviewed_wiki_candidate source items: 3
- new_reviewed_wiki_candidate targets: 2
- reject_runtime: 0
- future_qa: 4
- confirmation queue items: 13
- apply status: not executed
- PWA sync status: not executed

## Anime Translation Value Distribution

- high: 14
- medium: 0
- low: 0
- none: 4

High means the item directly converts film language into anime storyboard fields
and AI anime prompt constraints. None means the item does not safely serve the
field-bound product translation chain and stays out of runtime.

## Existing Coverage Used

- `rw-visual-master-consistency`
- `rw-camera-language-grammar`
- `rw-material-light-air-physicality`
- `rw-expression-physicalization`
- `rw-transition-motion-dynamics`
- `rw-person-identity-lock`
- `rw-prompt-text-boundary`
- `rw-validation-no-leakage`
- `rw-scene-expression-visible-action`
- `rw-dialogue-evidence-lock`
- `rw-director-scheduling-core`
- `rw-shot-intent-taxonomy`
- `rw-duration-density-rules`
- `rw-ensemble-action-layering`
- `rw-seedance2-storyboard-field-discipline`

## Candidate Conclusions

| # | sanitized candidate | anime_translation_value | conclusion | primary group | target |
| --- | --- | --- | --- | --- | --- |
| 1 | 景别转动漫可读尺度 | high | merge_existing | director_group | `rw-camera-language-grammar`, `rw-visual-master-consistency` |
| 2 | 运镜转动漫动势层 | high | merge_existing | director_group | `rw-camera-language-grammar`, `rw-transition-motion-dynamics`, `rw-director-scheduling-core` |
| 3 | 剪辑节奏转反应镜头与冲击停顿 | high | merge_existing | director_group | `rw-shot-intent-taxonomy`, `rw-duration-density-rules`, `rw-transition-motion-dynamics` |
| 4 | 电影表演转动漫姿态动作线 | high | merge_existing | director_group | `rw-expression-physicalization` |
| 5 | 电影光影转动漫色块与受光关系 | high | merge_existing | director_group | `rw-material-light-air-physicality`, `rw-visual-master-consistency` |
| 6 | 编剧信息转画面承载 | high | merge_existing | writing_group | `rw-scene-expression-visible-action` |
| 7 | AI 动漫提示词字段编译顺序 | high | merge_existing | validation_group | `rw-prompt-text-boundary`, `rw-seedance2-storyboard-field-discipline` |
| 8 | 速度线与背景流动表达快速动作 | high | merge_existing | director_group | `rw-transition-motion-dynamics`, `rw-camera-language-grammar` |
| 9 | 关键姿态优先于制作工序 | high | new_reviewed_wiki_candidate | director_group | `rw-anime-key-pose-silhouette-priority` |
| 10 | 单镜头单主目标防过载 | high | merge_existing | validation_group | `rw-duration-density-rules`, `rw-prompt-text-boundary`, `rw-seedance2-storyboard-field-discipline` |
| 11 | 动漫主体轮廓优先 | high | new_reviewed_wiki_candidate | director_group | `rw-anime-key-pose-silhouette-priority` |
| 12 | 动漫动作预备与跟随 | high | merge_existing | director_group | `rw-expression-physicalization`, `rw-transition-motion-dynamics` |
| 13 | 动漫反应镜头承接情绪变化 | high | merge_existing | director_group | `rw-shot-intent-taxonomy`, `rw-duration-density-rules`, `rw-director-scheduling-core` |
| 14 | 动漫化失败防护：真人电影感过强 | high | new_reviewed_wiki_candidate | validation_group | `rw-anime-translation-failure-guard` |
| F1 | 真实导演、工作室、电影、游戏、IP 风格开关 | none | future_qa | validation_group | blocked before runtime |
| F2 | 长 prompt 万能模板 | none | future_qa | validation_group | blocked before runtime |
| F3 | 自动对白生成套路 | none | future_qa | validation_group | blocked before runtime |
| F4 | 泛动漫风格词库 | none | future_qa | validation_group | blocked before runtime |

## New Candidate Targets

- `rw-anime-key-pose-silhouette-priority`: director-first candidate. It merges
  "关键姿态优先于制作工序" and "动漫主体轮廓优先" into one rule so final PWA fields
  describe visible key pose, silhouette, action line, center of gravity, and
  readable subject priority instead of production-process wording.
- `rw-anime-translation-failure-guard`: validation-first candidate. It blocks
  live-action film feel, proper-name style switches, IP/style pollution, and
  generic film terminology from entering runtime unless translated into anime
  storyboard and prompt fields.

## High Value Candidates

- 景别转动漫可读尺度: maps shot scale to anime subject readability.
- 运镜转动漫动势层: maps camera movement to anime motion layer and background flow.
- 剪辑节奏转反应镜头与冲击停顿: maps editing rhythm to reaction cuts and pauses.
- 电影表演转动漫姿态动作线: maps performance to pose, gaze, hand, weight, and action line.
- 电影光影转动漫色块与受光关系: maps lighting to color-block hierarchy and light direction.
- 编剧信息转画面承载: maps story information to visible action and frame load.
- AI 动漫提示词字段编译顺序: maps storyboard fields to field-bound AI prompt text.
- 速度线与背景流动表达快速动作: maps fast motion to anime speed lines and background flow.
- 关键姿态优先于制作工序: keeps final fields focused on visible key pose.
- 单镜头单主目标防过载: keeps each row field-bound and readable.
- 动漫主体轮廓优先: makes subject silhouette the first anime readability guard.
- 动漫动作预备与跟随: maps action phases to anticipation and follow-through.
- 动漫反应镜头承接情绪变化: maps emotional turns to visible reaction shots.
- 动漫化失败防护：真人电影感过强: blocks live-action feel from replacing anime translation.

## Low Or None Downgrades

- low: none.
- none: 真实导演、工作室、电影、游戏、IP 风格开关. Reason: proper names and IPs cannot become runtime style switches.
- none: 长 prompt 万能模板. Reason: conflicts with field-bound prompt compilation and risks prompt overload.
- none: 自动对白生成套路. Reason: encourages unsupported dialogue invention and conflicts with dialogue evidence lock.
- none: 泛动漫风格词库. Reason: too generic and not mapped to fixed PWA fields.

## Runtime Eligibility

Runtime-eligible after user confirmation:

- 14 KB-entry candidates.
- 13 confirmation queue targets.
- All runtime-eligible candidates are summary-only, field-bound, and constrained
  to existing PWA final fields.

Not runtime-eligible:

- all 4 FutureQA items.

Reject_runtime:

- none in this pass.

## Confirmation Queue

Recommended follow-up user confirmation targets:

- `rw-camera-language-grammar`
- `rw-director-scheduling-core`
- `rw-transition-motion-dynamics`
- `rw-shot-intent-taxonomy`
- `rw-duration-density-rules`
- `rw-expression-physicalization`
- `rw-material-light-air-physicality`
- `rw-scene-expression-visible-action`
- `rw-prompt-text-boundary`
- `rw-seedance2-storyboard-field-discipline`
- `rw-visual-master-consistency`
- `rw-anime-key-pose-silhouette-priority`
- `rw-anime-translation-failure-guard`

## FutureQA And Runtime Blocks

- 真实导演、工作室、电影、游戏、IP 风格开关: FutureQA only until de-named and abstracted.
- 长 prompt 万能模板: FutureQA only; blocked by prompt boundary and duration-density constraints.
- 自动对白生成套路: FutureQA only; blocked by dialogue evidence lock.
- 泛动漫风格词库: FutureQA only until converted into field-bound visual rules.

## Translation Chain Checks

Each runtime-eligible item was checked against:

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

## Draft Package

- `knowledge/intake_runs/intake-20260526-film-anime-ai-translation-candidates/intake-package.draft.json`
- confirmation state: false
- KB application state: false
- apply gate: closed
- PWA sync: not executed
