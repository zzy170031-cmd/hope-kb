# Visual Rule Candidate Dedupe Report

Status: pending confirmation only.

Scope: summary-only graph handoff candidates for image-derived storyboard rules.

## Result Summary

- received candidates: 17
- merge_existing: 14
- new_reviewed_wiki_candidate: 0
- reject_runtime: 1
- future_qa: 2
- confirmation queue items: 9
- apply status: not executed
- PWA sync status: not executed

## Existing Coverage Used

- `rw-visual-master-consistency`
- `rw-camera-language-grammar`
- `rw-material-light-air-physicality`
- `rw-expression-physicalization`
- `rw-transition-motion-dynamics`
- `rw-seedance2-storyboard-field-discipline`
- `rw-person-identity-lock`
- `rw-prompt-text-boundary`
- `rw-validation-no-leakage`
- `rw-writing-continuity-core`
- `rw-scene-expression-visible-action`

## Candidate Conclusions

| # | sanitized candidate | conclusion | target |
| --- | --- | --- | --- |
| 1 | Visual master continuity anchor | merge_existing | `rw-visual-master-consistency` |
| 2 | Storyboard field separation | merge_existing | `rw-seedance2-storyboard-field-discipline` |
| 3 | Camera angle serves shot function | merge_existing | `rw-camera-language-grammar` |
| 4 | Composition attention path | merge_existing | `rw-visual-master-consistency` |
| 5 | Foreground midground background depth | merge_existing | `rw-material-light-air-physicality` |
| 6 | Light color and material grounding | merge_existing | `rw-material-light-air-physicality` |
| 7 | Environment responds to action | merge_existing | `rw-material-light-air-physicality` |
| 8 | Emotion as visible performance | merge_existing | `rw-expression-physicalization` |
| 9 | Micro expression respects shot size | merge_existing | `rw-expression-physicalization` |
| 10 | Transition and motion rhythm | merge_existing | `rw-transition-motion-dynamics` |
| 11 | Person field identity guard | merge_existing | `rw-person-identity-lock` |
| 12 | Prompt text compiler boundary | merge_existing | `rw-prompt-text-boundary` |
| 13 | Source leakage prevention | merge_existing | `rw-validation-no-leakage` |
| 14 | Upstream story task readiness | merge_existing | `rw-writing-continuity-core` |
| 15 | Reviewer note and status are not final fields | reject_runtime | covered by existing safety boundaries |
| 16 | Style prototype after de-naming | future_qa | review layer only |
| 17 | Product capability reference | future_qa | review layer only |

## Confirmation Queue

The 5179 page can show checkboxes for the 9 `ready_for_confirmation` merge targets in the draft package. Two additional upstream-story merge findings remain report-only because they sit outside the requested confirmation target set. Confirmation records user intent only; it must not be treated as permission to apply, sync PWA, write runtime, or create new PWA fields.

## Deferred Items

- No new reviewed wiki candidate is recommended in this pass.
- The P2 style/aesthetic candidate remains FutureQA until de-named and re-reviewed.
- Product capability facts remain FutureQA and must not become creative prompt rules.
- Reviewer note/status handling is rejected for runtime output and remains an internal safety boundary.
