# Knowledge Intake Candidate List v0.2

Status: Batch 1 approved for prototype by controller.
Scope: proposed reviewed wiki knowledge entries to add to `hope-kb` for actual
Hope content production and `hope-web-pwa` runtime snapshot mapping.

This document lists the knowledge entries proposed for intake. It is not yet a
reviewed wiki artifact set and does not authorize runtime consumption by
itself.

The proposed entries exist only to improve Hope content output. They must not
shift product focus toward maintaining or browsing the KB itself.

## Alignment Target

The knowledge base must support the actual Hope production chain:

```text
source text
-> expand or rewrite story body
-> accepted body
-> storyboard task
-> storyboard rows
-> prompt_text
-> validation
-> export
```

Each proposed entry below must eventually map to at least one of these PWA
runtime surfaces:

```text
writing_rule_packs
director_rule_packs
validation_rule_packs
scene_mappings
duration_profiles
selected_kb_rules
kb_context_summary
negative_constraints
action_results
future_qa_backlog
```

## Batch 1: Minimum Product-Visible Knowledge

Batch 1 is the recommended first intake wave. It is intentionally small enough
to review, map, and validate, while still covering the PWA chain from story
body to export.

### 1. `rw-writing-continuity-core`

Purpose: preserve user facts, character motivation, event order, causality,
timeline, prop state, and setup/payoff obligations during expansion and
rewriting.

Supports PWA actions:

```text
expand_story
rewrite_story
accept_story_body
create_story_task
```

Runtime targets:

```text
writing_rule_packs
scene_mappings
kb_context_summary
negative_constraints
```

Content to add:

- user fact priority rules
- character motivation continuity
- conflict causality rules
- timeline and prop state continuity
- setup/payoff carry rules
- next-scene or next-chapter bridge rules
- rule that KB hints cannot overwrite accepted facts

Acceptance criteria:

- maps to at least one writing rule pack
- provides a compact `kb_context_summary` fragment
- has no raw source text or raw prompt body
- has `leakage_count=0`

### 2. `rw-scene-expression-visible-action`

Purpose: convert accepted story material into filmable, visible, and
storyboard-ready scene expression.

Supports PWA actions:

```text
expand_story
rewrite_story
create_story_task
generate_storyboard
```

Runtime targets:

```text
writing_rule_packs
director_rule_packs
scene_mappings
action_results
```

Content to add:

- scene objective
- obstacle
- visible action
- reaction beat
- turning point
- scene purpose
- exposition-to-visible-action conversion
- blocker for decorative scenes with no causal purpose

Acceptance criteria:

- maps to scene expression rules
- supports storyboard task creation
- keeps story facts above scene expression style
- has `leakage_count=0`

### 3. `rw-director-scheduling-core`

Purpose: guide director-level performance focus, blocking, rhythm, visual
attention, and scene-to-shot handoff.

Supports PWA actions:

```text
create_story_task
generate_storyboard
repair_storyboard
```

Runtime targets:

```text
director_rule_packs
scene_mappings
selected_kb_rules
kb_context_summary
```

Content to add:

- performance focus rules
- blocking hint rules
- visual attention rules
- rhythm adjustment rules
- continuity handoff notes
- rule that director hints must not override accepted story facts

Acceptance criteria:

- maps to at least one director rule pack
- can be selected by scene type
- does not introduce new story facts
- has `leakage_count=0`

### 4. `rw-shot-intent-taxonomy`

Purpose: define shot-level intent so storyboard rows carry visible production
work instead of generic descriptions.

Supports PWA actions:

```text
create_story_task
generate_storyboard
repair_storyboard
```

Runtime targets:

```text
director_rule_packs
scene_mappings
selected_kb_rules
```

Content to add:

- establish
- reveal
- pursue
- clash
- reaction
- transition
- payoff
- spectacle
- detail insert
- intent-to-row mapping rules

Acceptance criteria:

- each intent describes visible row behavior
- no intent requires raw KB or graph evidence at runtime
- maps to director guidance
- has `leakage_count=0`

### 5. `rw-duration-density-rules`

Purpose: connect target durations to shot count, beat density, row duration,
and validation rules.

Supports PWA actions:

```text
create_story_task
generate_storyboard
validate_result
```

Runtime targets:

```text
duration_profiles
director_rule_packs
validation_rule_packs
negative_constraints
```

Content to add:

- 5 seconds -> 3 compact shots
- 10 seconds -> 4 compact shots
- 15 seconds -> 5 shots
- 30 seconds -> 7 shots
- 45 seconds -> 9 shots
- 60 seconds -> 12 shots
- duration sum must match target
- no zero-duration row
- no repeated full-budget rows

Acceptance criteria:

- maps to every supported PWA duration
- supports duration validator checks
- provides repair direction for duration mismatch
- has `leakage_count=0`

### 6. `rw-prompt-text-boundary`

Purpose: ensure `prompt_text` is a clean generation-facing field compiled from
accepted facts and confirmed storyboard rows.

Supports PWA actions:

```text
generate_storyboard
repair_storyboard
validate_result
export_result
```

Runtime targets:

```text
director_rule_packs
validation_rule_packs
negative_constraints
action_results
```

Content to add:

- prompt text must include subject, action, visual frame, camera, duration, and
  negative constraints
- prompt text must align with `visual_description`
- prompt text must be compiled after the confirmed row
- no schema IDs
- no trace refs
- no raw KB
- no source register
- no internal hashes

Acceptance criteria:

- maps to prompt boundary validation
- supports export safety
- blocks internal reference leakage
- has `leakage_count=0`

### 7. `rw-validation-no-pseudo-success`

Purpose: prevent empty rows, missing prompt text, blocked state, or stale
artifacts from being treated as successful output.

Supports PWA actions:

```text
validate_result
repair_storyboard
export_result
```

Runtime targets:

```text
validation_rule_packs
action_results
negative_constraints
```

Content to add:

- rows must be non-empty
- visual descriptions must be non-empty
- prompt text must be non-empty
- `rows_match=true` cannot pass when `rows=0`
- blocked export cannot be marked ready
- repair must be explicit after validation failure

Acceptance criteria:

- maps to validation rule pack
- gives repair direction for empty rows and missing prompt text
- blocks export when output is not ready
- has `leakage_count=0`

### 8. `rw-validation-no-leakage`

Purpose: prevent raw KB, source register, prompt body, internal refs, local
paths, provider config, and secrets from leaking into PWA prompts, UI, traces,
or exports.

Supports PWA actions:

```text
import_source
validate_result
export_result
golden_sample_review
```

Runtime targets:

```text
validation_rule_packs
negative_constraints
action_results
```

Content to add:

- `raw_kb_rows_included=0`
- `raw_sample_text_absent=true`
- `source_register_absent=true`
- `overlay_json_absent=true`
- `prompt_body_absent=true`
- no local path
- no API key or token
- no provider config
- no internal hash or ref in user export

Acceptance criteria:

- maps to export and prompt safety
- keeps runtime snapshot summary-only
- rejects denied fields structurally
- has `leakage_count=0`

## Batch 2: Failure And Repair Knowledge

Batch 2 should follow Batch 1 after the first mapping prototype is reviewed.

### 9. `rw-writing-failure-patterns`

Purpose: record story-body failure patterns.

Candidate patterns:

- `motivation_jump`
- `timeline_break`
- `unsupported_worldbuilding`
- `fact_overwrite_by_style`
- `causality_gap`
- `emotion_state_jump`
- `chapter_bridge_missing`

Runtime targets:

```text
future_qa_backlog
validation_rule_packs
repair_mapping
```

### 10. `rw-writing-repair-mappings`

Purpose: map writing failures to repair actions.

Candidate mappings:

- `motivation_jump` -> restore cause and intent
- `timeline_break` -> rebuild event order
- `unsupported_worldbuilding` -> remove unsupported additions
- `causality_gap` -> add cause/effect bridge
- `emotion_state_jump` -> add transition beat

Runtime targets:

```text
repair_mapping
action_results
kb_context_summary
```

### 11. `rw-validation-field-aware-entity`

Purpose: prevent `person` field pollution by location, action fragments, scene
terms, or generic placeholders.

Supports PWA actions:

```text
generate_storyboard
validate_result
repair_storyboard
```

Runtime targets:

```text
validation_rule_packs
failure_pattern
repair_mapping
```

Candidate checks:

- `person_field_valid`
- `location_not_in_person`
- `action_fragment_not_in_person`
- `scene_term_not_in_person`
- `generic_role_placeholder_absent`

### 12. `rw-export-safety-rules`

Purpose: ensure exported storyboard or script files are user-deliverable and
contain no internal evidence.

Supports PWA actions:

```text
export_result
validate_result
```

Runtime targets:

```text
validation_rule_packs
action_results
negative_constraints
```

Candidate checks:

- export uses confirmed rows only
- export contains prompt text
- export contains no internal refs
- export contains no raw KB
- export duration matches target
- export blocked when validation fails

### 13. `rw-pwa-trace-to-failure-patterns`

Purpose: convert PWA QA trace fields into KB failure patterns.

Candidate trace mappings:

- `rows_count=0` -> `no_pseudo_success`
- `prompt_text_present=false` -> `prompt_text_missing`
- `person_field_valid=false` -> `field_pollution`
- `export_internal_evidence_exposed=true` -> `export_leakage`
- duration mismatch -> `duration_allocation_failure`

Runtime targets:

```text
failure_pattern
future_qa_candidate
validation_rule_packs
```

### 14. `rw-pwa-repair-routing`

Purpose: route PWA validation failures to repair strategies.

Candidate routes:

- empty rows -> regenerate storyboard with visible action constraints
- prompt missing -> recompile prompt from confirmed row
- person polluted -> re-extract characters from accepted body
- duration mismatch -> rerun duration allocator
- internal refs leaked -> sanitize export and block release

Runtime targets:

```text
repair_mapping
action_results
kb_context_summary
```

### 15. `rw-futureqa-intake`

Purpose: keep unresolved or model-sensitive issues out of runtime while
preserving them for later QA.

Rules:

- FutureQA is not eval truth
- FutureQA requires human review
- FutureQA requires Git promotion
- FutureQA requires rebuild and validation
- FutureQA becomes stale if model or snapshot bindings change

Runtime targets:

```text
future_qa_backlog
validation_rule_packs
```

## Controller Approval

Approved Batch 1:

```text
rw-writing-continuity-core
rw-scene-expression-visible-action
rw-director-scheduling-core
rw-shot-intent-taxonomy
rw-duration-density-rules
rw-prompt-text-boundary
rw-validation-no-pseudo-success
rw-validation-no-leakage
```

Batch 1 is enough to build a reviewed wiki artifact prototype and a meaningful
wiki-to-runtime mapping prototype for `hope-web-pwa`.

Hold Batch 2 until Batch 1 has been mapped and lightly validated.
