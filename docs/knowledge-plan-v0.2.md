# Knowledge Plan v0.2

Status: planning inventory.
Scope: content families needed to turn `hope-kb` into a sustained reviewed
wiki and graph knowledge source for `hope-web-pwa`.

This plan lists the knowledge content still needed after the KB/PWA product
chain contract. It does not create reviewed wiki artifacts or authorize runtime
consumption by itself.

Hope-KB exists to improve Hope content output. It should stay subordinate to
the production chain and should not become a separate knowledge product, graph
UI, or user-facing workspace.

## Content Principles

- User facts outrank KB guidance.
- Hope output quality outranks KB artifact completeness.
- Reviewed wiki outranks draft wiki.
- Runtime snapshots consume summaries and IDs only.
- Every runtime rule must trace back to reviewed wiki.
- Every failure pattern should either have a repair mapping or be marked
  FutureQA-only.
- Every scene type should have writing, director, validation, and duration
  coverage.
- No raw source, raw KB row, prompt body, source register, local path, provider
  config, secret, or raw graph dump may enter runtime assets.

## Required Knowledge Domains

### 1. Writing Continuity

Purpose: support `expand_story`, `rewrite_story`, `accept_story_body`, and
`create_story_task`.

Documents to create:

```text
docs/knowledge/writing-continuity/core-rules.md
docs/knowledge/writing-continuity/failure-patterns.md
docs/knowledge/writing-continuity/repair-mappings.md
docs/knowledge/writing-continuity/scene-type-coverage.md
```

Minimum content:

- fact priority rules
- motivation continuity rules
- conflict causality rules
- timeline and prop-state rules
- setup/payoff carry rules
- chapter or segment bridge rules
- failure patterns for motivation jumps, timeline breaks, and unsupported
  invention
- repair mappings for each failure pattern

Runtime mapping:

- `writing_rule_packs`
- `scene_mappings`
- `kb_context_summary`
- `negative_constraints`

### 2. Scene Expression

Purpose: turn accepted story material into filmable scene intent and
storyboard-ready structure.

Documents to create:

```text
docs/knowledge/scene-expression/core-rules.md
docs/knowledge/scene-expression/scene-purpose-taxonomy.md
docs/knowledge/scene-expression/dialogue-action-rules.md
docs/knowledge/scene-expression/compression-and-turning-points.md
```

Minimum content:

- scene objective and obstacle taxonomy
- visible action conversion rules
- dialogue intent rules
- action/reaction/turn segmentation
- prose-to-scene compression rules
- failure patterns for decorative scenes with no causal purpose
- repair mappings for unfilmable or over-abstract scene drafts

Runtime mapping:

- `writing_rule_packs`
- `director_rule_packs`
- `scene_mappings`
- `action_results`

### 3. Director Scheduling

Purpose: guide blocking, performance focus, rhythm, visual attention, and
scene-to-shot handoff.

Documents to create:

```text
docs/knowledge/director-scheduling/core-rules.md
docs/knowledge/director-scheduling/blocking-patterns.md
docs/knowledge/director-scheduling/rhythm-and-attention.md
docs/knowledge/director-scheduling/continuity-handoff.md
```

Minimum content:

- performance focus rules
- blocking patterns
- attention routing rules
- rhythm adjustment rules
- continuity handoff notes
- director-does-not-overwrite-story rule
- failure patterns for director hints overriding accepted facts
- repair mappings for bad blocking and unclear visual focus

Runtime mapping:

- `director_rule_packs`
- `scene_mappings`
- `selected_kb_rules`
- `kb_context_summary`

### 4. Shot Language And Prompt Packaging

Purpose: support `generate_storyboard`, `repair_storyboard`, prompt
compilation, and duration-aware shot planning.

Documents to create:

```text
docs/knowledge/shot-language/shot-intent-taxonomy.md
docs/knowledge/shot-language/camera-and-scale-rules.md
docs/knowledge/shot-language/duration-density-rules.md
docs/knowledge/shot-language/prompt-text-boundary.md
```

Minimum content:

- shot intent taxonomy
- camera movement rules
- shot size and scene scale rules
- visible-frame requirements
- duration-to-shot-count guidance
- prompt text composition boundary
- failure patterns for empty prompt text, abstract visuals, and duration
  mismatch
- repair mappings for prompt and visual grounding failures

Runtime mapping:

- `director_rule_packs`
- `duration_profiles`
- `validation_rule_packs`
- `negative_constraints`

### 5. Validation, Leakage, And Export Safety

Purpose: prevent pseudo-success and unsafe output in validation, traces, UI,
and export.

Documents to create:

```text
docs/knowledge/validation/no-pseudo-success.md
docs/knowledge/validation/field-aware-entity-rules.md
docs/knowledge/validation/no-leakage-rules.md
docs/knowledge/validation/export-safety-rules.md
```

Minimum content:

- rows non-empty rules
- prompt text non-empty rules
- field-aware person/entity validation
- stale task and stale row rules
- internal ref export blockers
- summary-only leakage rules
- failure patterns for leaked internal refs, local paths, and raw KB terms
- repair mappings for each blocker class

Runtime mapping:

- `validation_rule_packs`
- `negative_constraints`
- `action_results`
- `future_qa_backlog`

### 6. PWA Feedback To KB

Purpose: convert product QA failures into reviewed KB improvement tasks.

Documents to create:

```text
docs/knowledge/pwa-feedback/trace-to-failure-patterns.md
docs/knowledge/pwa-feedback/repair-routing.md
docs/knowledge/pwa-feedback/futureqa-intake.md
docs/knowledge/pwa-feedback/coverage-gap-review.md
```

Minimum content:

- mapping from PWA QA trace fields to failure patterns
- mapping from validation blockers to repair strategies
- coverage gap review process
- FutureQA candidate intake criteria
- rules for not treating FutureQA as eval truth

Runtime mapping:

- `future_qa_backlog`
- `validation_rule_packs`
- `action_results`
- `kb_context_summary`
- `validation_rule_packs`

## Coverage Matrix To Build

The first coverage matrix should check:

```text
scene_type x writing_rule_packs
scene_type x director_rule_packs
scene_type x validation_rule_packs
kb_action x rule_group
duration x duration_profile
failure_pattern x repair_mapping
reviewed_wiki_id x runtime_rule_pack_id
selected_kb_rule_id x reviewed_wiki_id
summary_fragment_id x reviewed_wiki_id
```

No runtime snapshot should be released while any required runtime-eligible
mapping is missing.

## Proposed Artifact Roots

These roots are proposed for a later artifact-prototype gate:

```text
knowledge/wiki_draft/
knowledge/reviewed_wiki/
knowledge/mappings/
knowledge/runtime_snapshots/
```

Each reviewed wiki page should eventually include:

```text
reviewed_wiki_id
wiki_type
title
claims_summary
source_coverage_summary
linked_schema_ids
capability_tags
scene_type_ids
kb_actions
failure_pattern_ids
repair_mapping_ids
review_status
effective_confidence
runtime_eligible
leakage_count
```

## Build Order

1. Keep `docs/kb-product-chain-contract-v0.2.md` as the boundary contract.
2. Validate schemas and samples.
3. Create reviewed wiki artifact prototype with 3 to 5 pages only.
4. Create wiki-to-runtime mapping prototype.
5. Compile a larger runtime snapshot prototype.
6. Add validator coverage for mapping completeness and leakage.
7. Integrate `hope-web-pwa` snapshot loading while keeping built-in fallback.
8. Use PWA QA traces to create failure pattern and repair mapping waves.

## First Knowledge Batch Recommendation

The first real content batch should be small and product-visible:

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

This batch is enough to prove the mapping from reviewed wiki to PWA rule packs
without pretending the whole knowledge base is complete.
