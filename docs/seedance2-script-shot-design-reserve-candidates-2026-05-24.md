# Seedance2 Script And Shot Design Reserve Candidates 2026-05-24

Status: reserve candidate only.
Scope: two user-provided visual cards about script outline logic and storyboard shot design.
Runtime status: not runtime eligible yet.

## Boundary

These two images are reserve knowledge candidates. They are not reviewed wiki,
not runtime snapshot content, and not PWA output material.

The extracted knowledge must be used only as rewritten principles. Runtime and
user-facing output must not include image filenames, local paths, OCR text,
watermarks, source labels, raw screenshot wording, matrix ids, or source hashes.

Any future promotion must follow:

```text
reserve candidate
-> dedupe and abstraction
-> reviewed wiki
-> wiki-to-runtime mapping
-> runtime-kb-snapshot
-> PWA adapter
-> existing PWA storyboard fields
```

## Source Candidates

| reserve id | source type | hash | dimensions | reserve purpose | runtime eligibility |
| --- | --- | --- | --- | --- | --- |
| `reserve-20260524-script-outline-logic-card` | user-provided image card | `sha256:c94b39d4f3555b5b3e05bc7b2ba6d053b3942b985ee9ae7555e7d1bda4e2b31d` | 1242x1660 | Upstream story completeness and script-outline checklist. | blocked until reviewed and mapped to writing/task rules. |
| `reserve-20260524-shot-design-language-card` | user-provided image card | `sha256:732893d784414197cc841ec39135a41ef398f6e8897e05a2c3d95964642ffcaa` | 1242x1660 | Storyboard field checklist for shot scale, camera angle, composition, performance description, and background. | blocked until reviewed and mapped to existing PWA storyboard fields. |

## Candidate 1: Script Outline Logic

### Extracted Principle

A script task should first establish a stable story direction before storyboard
generation. The upstream task should clarify genre or setting family, visual
style, target audience emotion, and the story logic chain.

### Possible KB Placement

| target | value |
| --- | --- |
| reviewed wiki candidate | `rw-writing-continuity-core`, `rw-scene-expression-visible-action`, or a future upstream script-outline checklist entry |
| runtime group | writing group first, director group only after task creation |
| PWA actions | `expand_story`, `rewrite_story`, `accept_story_body`, `create_story_task` |
| direct storyboard field target | none |
| indirect storyboard impact | improves accepted body and storyboard task completeness before rows are generated |

### Candidate Rules

Positive rules:

- Before creating storyboard rows, ensure the accepted story material has a
  clear setting family, style direction, target emotional effect, and causal
  chain.
- Treat time, place, characters, trigger, development, and outcome as an
  upstream completeness checklist for story task creation.
- Use this candidate to improve task readiness, not to add row-level camera
  language.

Negative rules:

- Do not write this checklist into `prompt_text`.
- Do not invent story facts when the accepted body is incomplete.
- Do not map this candidate directly to `camera`, `shot_size`,
  `visual_description`, or `character_action`.
- Do not copy prompt templates or screenshot wording into runtime output.

### Field Consumption Boundary

This candidate is not directly consumed by the current storyboard table. It may
only affect upstream writing/task guidance after review. If promoted, it should
help the PWA prepare better source facts and task fragments while preserving the
existing row shape.

## Candidate 2: Storyboard Shot Design Language

### Extracted Principle

Storyboard rows become more controllable when each shot explicitly separates
shot scale, camera angle, composition, performance description, and background
description. The same story can produce different visual quality depending on
these choices.

### PWA Field Mapping

| extracted concept | existing PWA field | allowed influence |
| --- | --- | --- |
| shot scale | `shot_size` | Select wide, medium, close-up, insert, detail, or related scale according to visible function. |
| camera angle / viewpoint | `camera` | Add front, back, side, high angle, low angle, overhead, POV, or similar viewpoint only when it serves the beat. |
| composition | `visual_description`, `camera` | Use center, rule of thirds, diagonal, foreground occlusion, depth, or focus only to improve current-shot readability. |
| appearance / clothing / expression / action / emotion | `character_action`, `visual_description` | Convert description into visible performance and subject presentation; expression detail must respect shot size. |
| place / scene details / light / color / atmosphere | `visual_description` | Add scene-bound background, lighting, palette, and atmosphere without inventing unsupported facts. |
| complete prompt packaging | `prompt_text` | Compile after row fields are confirmed; do not copy source templates. |

### Possible KB Placement

| target | value |
| --- | --- |
| reviewed wiki candidate | `rw-camera-language-grammar`, `rw-expression-physicalization`, `rw-material-light-air-physicality`, `rw-prompt-text-boundary` |
| runtime group | director group and validation group |
| PWA actions | `create_story_task`, `generate_storyboard`, `repair_storyboard`, `validate_result`, `export_result` |
| direct storyboard fields | `camera`, `shot_size`, `visual_description`, `character_action`, `prompt_text` |

### Candidate Rules

Positive rules:

- Every row should keep shot scale, camera/viewpoint, visual frame, performance
  action, and background information in their own existing fields.
- Camera angle and composition should be used to clarify visual attention,
  spatial relation, emotional pressure, or reveal.
- Background details should include only scene-supported place, light, color,
  material, and atmosphere.
- Performance descriptions should convert emotion into visible expression,
  body posture, breath, gaze, hand motion, or spacing.

Negative rules:

- Do not merge `shot_size` into `camera`.
- Do not put camera or composition vocabulary into `person`.
- Do not invent dialogue from the image card.
- Do not use composition, light, or atmosphere words as disconnected decoration.
- Do not output screenshot text, source template wording, filenames, local
  paths, watermarks, source labels, or hashes.

## Review Checklist Before Promotion

This reserve entry may be promoted only when all checks pass:

- duplicate review confirms both images are distinct or marks any duplicate
  relation;
- extracted principles are rewritten and source-safe;
- each runtime-eligible rule maps to an existing PWA field;
- every mapped rule has a positive directive and a negative constraint;
- `review_status=reviewed`;
- `leakage_count=0`;
- no raw source text, local path, source hash, screenshot text, or watermark is
  emitted into runtime snapshot or PWA output.

## Desktop Impact

No desktop or PWA package update is required while this remains reserve
knowledge. Desktop update is required only after a later change modifies PWA
runtime-consumed artifacts such as:

```text
public/kb/latest.json
dist/kb/latest.json
src/lib/kb.ts
src/lib/prompts.ts
src/lib/validator.ts
export formatting
desktop launcher/package files
```

For this reserve-only step, the existing desktop build and launcher can stay as
they are.
