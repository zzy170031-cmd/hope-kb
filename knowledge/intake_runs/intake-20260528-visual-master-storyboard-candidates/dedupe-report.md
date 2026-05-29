# Dedupe Report: storyboard image prompt export candidates

run_id: `intake-20260528-visual-master-storyboard-candidates`
status: `draft_only`
created_at: `2026-05-28`

## Scope

This intake run covers visual master, production bible, multi-panel storyboard image prompts, and the newly added derived export lane: `导出故事板图提示词`.

The export candidate is derived only from confirmed PWA storyboard fields. It is not a new final PWA field, does not replace existing storyboard export headers, does not write back into storyboard row fields, and does not promise automatic video generation.

The WeChat article remains demand inspiration only. This run does not copy article text, universal prompt text, or complete case examples.

## Existing KB Baseline

Existing nearby entries in this checkout include:

- `rw-visual-master-consistency`
- `rw-preproduction-storyboard-guide`
- `rw-shot-intent-taxonomy`
- `rw-camera-language-grammar`
- `rw-material-light-air-physicality`
- `rw-prompt-text-boundary`
- `rw-duration-density-rules`
- `rw-transition-motion-dynamics`
- `rw-validation-no-leakage`
- `rw-validation-no-pseudo-success`

## Candidate Dedupe Decisions

| candidate | decision | existing alignment | notes |
| --- | --- | --- | --- |
| `rw-storyboard-image-prompt-export` | new export-boundary candidate | `rw-prompt-text-boundary`, `rw-preproduction-storyboard-guide`, `rw-duration-density-rules` | Main new lane: derived storyboard image prompt export from confirmed rows only. |
| `rw-visual-master-board-from-confirmed-rows` | merge-or-new | `rw-visual-master-consistency`, `rw-preproduction-storyboard-guide`, `rw-material-light-air-physicality` | Visual master is rebuilt from confirmed rows, not source articles. |
| `rw-character-reference-sheet-for-storyboard-image` | new-or-merge | `rw-visual-master-consistency`, `rw-writing-continuity-core`, `rw-validation-no-leakage` | Character reference description only; no new person facts or mandatory turnarounds. |
| `rw-scene-prop-continuity-for-storyboard-image` | new-or-merge | `rw-writing-continuity-core`, `rw-visual-master-consistency`, `rw-material-light-air-physicality` | Adds scene, prop, route, and landmark continuity for board export. |
| `rw-multipanel-layout-boundary` | new export-boundary candidate | `rw-preproduction-storyboard-guide`, `rw-duration-density-rules`, `rw-prompt-text-boundary` | Layout follows confirmed row count and duration; no forced 8/9 grid. |
| `rw-frame-variation-and-shot-function-guard` | merge likely | `rw-shot-intent-taxonomy`, `rw-camera-language-grammar`, `rw-transition-motion-dynamics` | Prevents repeated composition, shot size, action, or lighting without fact drift. |
| `rw-storyboard-image-prompt-safety-boundary` | new safety-boundary candidate | `rw-prompt-text-boundary`, `rw-validation-no-leakage`, `rw-validation-no-pseudo-success` | Blocks KB/source/trace/hash/path/secret leakage in derived prompt export. |
| `rw-image-video-tool-handoff-boundary` | new handoff-boundary candidate | `rw-prompt-text-boundary`, `rw-validation-no-leakage`, `rw-director-scheduling-core` | Handoff to image/video tools only; no automatic generation or direct API binding. |

## Source Coverage

Domestic coverage now includes:

- 即梦 / Seedance: ByteDance Seed official page and Seedance technical report.
- 豆包 / 火山引擎: Volcengine developer-community image/video workflow references, used only as auxiliary structure evidence.
- 智谱: CogVideoX-2 official docs.
- 可灵: Kuaishou official multi-image reference announcement.
- 通义万相 / 阿里云: Model Studio image-to-video API reference.
- 腾讯混元: Tencent Cloud video task and data-structure docs, plus HunyuanVideo model-card context.

Foreign and industry coverage includes:

- OpenAI GPT Image / Sora video docs.
- Runway Gen-4 references and image/text-to-video prompting docs.
- Google Veo prompt and image-to-video best-practice docs.
- Toon Boom preproduction, animatic, panel-view, and asset docs.
- StudioBinder storyboard composition guidance.

GitHub auxiliary coverage includes:

- VibeFrame
- Jellyfish
- OpenMontage
- Director's Console
- Awesome AI Video Prompting

GitHub sources are auxiliary product-structure evidence only, not P0 authority.

## PWA Field Mapping

- 人物: only confirmed storyboard person facts; supports character consistency.
- 运镜: converted to camera motion / rhythm notes; no equipment jargon required.
- 景别: controls multi-panel scale variation.
- 画面描述: source for scene, material, lighting, atmosphere, and spatial continuity.
- 角色动作: source for key poses, action phases, and visible emotion.
- 对白/旁白: accepted body or confirmed row dialogue only; no invented captions.
- 分镜提示词: core input to the derived export; no internal rules or KB evidence.
- 时长(秒): pacing and panel-count suggestion only; no mechanical frame padding.

## Blocked / FutureQA

Color script and lighting progression are folded into `rw-visual-master-board-from-confirmed-rows` and existing `rw-material-light-air-physicality`; they are not a separate ninth candidate in this draft.

Blocked:

- Directly copying the WeChat universal prompt.
- Importing the 15 complete cases into runtime.
- Forcing every segment into 8/9 panels.
- Marketing claim that one image can make a whole film.
- IP, Disney, Studio Ghibli, real director, real studio, real actor, or public figure imitation.
- Writing generated images back into PWA fields.
- Changing final PWA fields or storyboard export headers.
- Putting `source_refs`, raw source, prompt body, internal trace, hashes, local paths, or secrets into runtime or PWA export.

FutureQA:

- Automatic video generation capability.
- Direct API integration for GPT Image, Jimeng, Doubao, Seedance, Kling, Wan, Tencent, Runway, or Veo.
- Provider-specific prompt syntax and moderation policy handling.
- Actual UI/export placement for `导出故事板图提示词`.

## Runtime Exclusion

This draft does not write:

- `knowledge/reviewed_wiki`
- `knowledge/mappings/wiki-to-runtime-mapping.v0.2.json`
- `knowledge/runtime_snapshots/latest.candidate.json`
- `samples/pwa-kb-adapter-output.sample.json`
- Hope PWA or desktop package files

`source_refs` stay in the draft audit layer only.

## 5179 Dashboard Note

The package status is `recommendations_ready` and all primary candidates use `ready_for_confirmation`, so the native 5179 evidence-ledger confirmation queue can render them from `/api/intake/latest` as confirmable recommendations. No temporary summary panel was added.


## Image2 Quality Control Precise Evidence Supplement

status: draft supplement only; confirmed package intentionally not mutated.

### Scope Boundary

This supplement strengthens the current run with Image2 quality-control candidates. It does not copy user experience text, fixed prompt templates, official document passages, or negative prompt dictionaries into runtime. It does not implement PWA functionality and does not alter the already-created confirmed package.

### Candidate Evidence Coverage

| candidate | new source_refs | coverage | decision |
| --- | ---: | --- | --- |
| `rw-image2-structured-quality-prompt` | 3 | official platform + official Adobe + GitHub auxiliary | keep for KB dedupe confirmation |
| `rw-image2-visual-hierarchy-control` | 3 | industry design + industry composition + official Runway | keep, likely merge with layout/staging rules |
| `rw-image2-dark-material-cleanliness` | 3 | official Google + material prompt reference + official Stability | keep, merge with material/light/air physicality |
| `rw-image2-reflection-control` | 3 | material reference + official Runway + official Google | keep, merge with material/light/air physicality |
| `rw-image2-weather-particle-control` | 3 | official Runway + official Runway resource + official Google | keep, field-bound export guard |
| `rw-image2-hand-prop-contact-integrity` | 3 | HOI research + HOI research + official Runway references | keep, merge with acting/readability and character continuity |
| `rw-image2-group-composition-density-guard` | 3 | education poster hierarchy + industry composition + official Runway | keep, merge with layout/staging and shot intent |
| `rw-image2-local-repair-prompt-boundary` | 3 | official OpenAI + official Google + ComfyUI open-source docs | keep as repair/export boundary, not runtime automation |
| `rw-image2-quality-preset-boundary` | 3 | official OpenAI + official Midjourney + official Stability | keep as boundary rule |
| `rw-image2-prompt-module-routing` | 3 | official Runway + official Adobe + GitHub auxiliary | keep as future export-module routing |

### Blocked

- User experience text verbatim.
- Full fixed Image2 prompt templates.
- Large negative prompt dictionaries.
- IP, Disney, Studio Ghibli, real director, or real studio style switches.
- Fixed output style forced across all scenes.
- Quality-word stacking as a substitute for structured prompt controls.
- Image2 quality rules changing current PWA person, dialogue, or duration fields.
- Full-image rewrite for local repair when the task is local correction.

### FutureQA

- Direct Image2/provider API integration.
- Generated image writeback into PWA fields or exports.
- Automatic local inpainting/repair.
- Multi-model automatic image scoring or ranking.
- Provider-specific negative prompt semantics and parameter adapters.

### PWA Field Boundary

The Image2 candidates serve future derived export or repair guidance only. They do not add current final storyboard fields, do not alter storyboard export headers, and do not write image prompts or repair results back into storyboard rows.
