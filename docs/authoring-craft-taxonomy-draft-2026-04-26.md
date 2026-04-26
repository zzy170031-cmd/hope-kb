# Authoring Craft Taxonomy Draft 2026-04-26

## Scope

This is a docs-only V0 draft for coordinating authoring craft, narrative craft,
screenplay rewriting, and director knowledge retrieval in `hope-kb`.

It does not:

- modify the 23-field KB main schema
- modify seed rows
- modify snapshots
- promote `reserve` rows
- promote rows with `usable_for_fewshot = No`
- add `director_style_ref`
- modify `docs/live-progress.md`
- modify the QA hardening docs-only packet
- introduce real author imitation
- introduce real IP, brand, or film-style replication

The goal is to let novel chapter generation, screenplay rewriting, and director
storyboarding consume compact craft guidance from KB without dumping full KB
rows into runtime prompts.

## Current Boundary

The current public KB boundary remains:

```text
23 main fields
152 v0.2 golden sample rows
32 GS120-CAND-* rows remain reserve
32 GS120-CAND-* rows remain usable_for_fewshot = No
no director_style_ref in seed main fields
full_kb_rows_included = 0
```

Allowed runtime disclosure stays limited to:

```text
kb_context_summary
selected_sample_ids
selected_kb_rules
user_readable_retrieval_summary
full_kb_rows_included = 0
```

Forbidden disclosure includes raw prompt content, raw KB rows, full source
registry values, overlay payloads, provider bodies, secrets, local paths, and
any expanded provenance payload.

## Content-First Principle

`authoring_craft` is not an author-style library. It is a content-creation
capability library.

Its job is to help generated chapters become better stories: more readable,
more causally continuous, more performable, and easier to convert into
screenplay and storyboard form. It must not imitate any real author, real IP,
brand, or protected style.

Recommended priority:

```text
content
> screenplay_structure
> director_blocking
> shot_storyboard
> prompt_text
```

Interpretation:

- content facts, user facts, character intent, continuity, and story causality
  outrank every downstream craft hint
- screenplay structure makes content playable, but cannot rewrite the user's
  facts or the story's established facts
- director blocking gives staging suggestions, but cannot override content
  intent or screenplay facts
- shot storyboard turns staging into executable shots, but cannot replace the
  previous layer's meaning
- `prompt_text` is the final packaging surface, not the source of truth

### Should Serve

`authoring_craft` should serve these content and adaptation functions:

1. story hook
2. character desire
3. character pressure
4. conflict engine
5. emotional turn
6. suspense setup
7. setup/payoff recovery
8. scene purpose
9. visualizable action
10. chapter hook
11. novel-to-screenplay compression
12. director-layer handoff

In V0, character pressure is covered by the combination of
`character_desire`, `conflict_escalation`, and `emotional_turn`; scene purpose
is covered by the combination of `narrative_hook`, `scene_transition`, and
`director_blocking_bridge`.

### Should Not Serve

`authoring_craft` should not serve:

1. real author imitation
2. IP style replication
3. brand style replication
4. generic prose beautification without story function
5. director showmanship that conflicts with user facts or established story
   facts

## Continuity-First Constraint

`authoring_craft` is not a virtuoso-writing or spectacle-writing library. Its
first target is `story_continuity`.

It must help the plot remain continuous, character motivation stay stable,
conflict preserve cause and effect, and the next scene remain bridgeable.

Continuity rules:

1. Writing craft may intensify the story, but it must not overwrite the story.
2. Exaggeration, rhetoric, high conflict, spectacle, and reversal all require a
   causal foundation.
3. Any authoring hint that damages continuity state must be downgraded or
   dropped.
4. User facts, continuity facts, character motivation, timeline state, and prop
   state outrank all authoring hints.
5. Later stages cannot cover or replace earlier-stage facts.

Recommended continuity-support tags:

```text
story_continuity
character_motivation_continuity
conflict_causality
emotional_progression
timeline_integrity
prop_state_integrity
next_scene_bridge
scene_causality_check
```

Recommended blocked-risk tags:

```text
style_over_story
exaggeration_without_cause
motivation_jump
timeline_break
prop_state_conflict
unfilmable_poetic_drift
continuity_break
```

Layer responsibility:

```text
authoring_layer serves content
content serves story_continuity
director_layer serves content
shot_layer serves director_blocking
prompt_text packages the accepted result
```

No later layer may overwrite facts from an earlier layer. If a beautiful line,
strong reversal, striking shot, or director hint breaks motivation, timeline,
prop state, scene causality, or next-scene handoff, it should be rejected from
the compact KB summary.

## V0 Taxonomy

`authoring_craft` is a retrieval and summary taxonomy. It describes what craft
function a KB summary is serving in a text-generation stage. It is not a new
main-table field in the current freeze.

Recommended initial enum:

```text
story_continuity
character_motivation_continuity
conflict_causality
emotional_progression
timeline_integrity
prop_state_integrity
next_scene_bridge
scene_causality_check
narrative_hook
character_desire
conflict_escalation
emotional_turn
scene_transition
suspense_setup
payoff_setup
dialogue_subtext
visualizable_action
chapter_cliffhanger
screenplay_compression
director_blocking_bridge
```

### Definitions

| authoring_craft | Definition | Best stage | Typical summary contribution |
| --- | --- | --- | --- |
| `story_continuity` | Preserves causal story flow across beats, chapters, scenes, and stages. | all stages | continuity reminder and next-step constraints |
| `character_motivation_continuity` | Keeps character desire, fear, pressure, and decision logic stable unless a justified turn occurs. | novel chapter, screenplay rewrite | motivation before/after summary and jump warning |
| `conflict_causality` | Ensures conflict escalation follows visible causes, stakes, and consequences. | all stages | cause/effect chain and unsupported-conflict warning |
| `emotional_progression` | Ensures emotional shifts move through readable pressure, reaction, and turn rather than abrupt mood changes. | novel chapter, screenplay rewrite | emotional state path and turn evidence |
| `timeline_integrity` | Keeps time order, elapsed time, flashback, and sequence transitions coherent. | all stages | timeline note and bridge requirement |
| `prop_state_integrity` | Preserves object, clue, weapon, location, evidence, and costume state across scenes and shots. | screenplay rewrite, director storyboard | prop state carryover and contradiction warning |
| `next_scene_bridge` | Ensures the current beat leaves a usable handoff into the next scene or chapter. | all stages | transition hook and carryover cue |
| `scene_causality_check` | Checks whether the scene exists for a causal purpose rather than decorative prose or spectacle. | screenplay rewrite, director storyboard | scene-purpose test and drop/merge hint |
| `narrative_hook` | Opens a chapter, scene, or beat with a concrete question, threat, promise, or desire. | novel chapter | one compact hook principle plus matched safe sample IDs |
| `character_desire` | Clarifies what the focal character wants now and what cost, fear, or limit blocks it. | novel chapter | desire/obstacle wording and character-state constraints |
| `conflict_escalation` | Raises pressure through stakes, reversals, power shifts, or tactical complications. | novel chapter, screenplay rewrite | escalation ladder and do-not-flatten warning |
| `emotional_turn` | Marks the point where a character's inner state visibly changes. | novel chapter, screenplay rewrite | before/after emotion summary and visible evidence cues |
| `scene_transition` | Bridges location, time, viewpoint, emotion, or action without losing continuity. | all stages | transition logic, handoff cue, and continuity reminder |
| `suspense_setup` | Plants uncertainty, withheld information, misdirection, or a delayed reveal. | novel chapter | suspense source, allowed ambiguity, and payoff dependency |
| `payoff_setup` | Establishes a detail or promise that later becomes meaningful. | novel chapter, screenplay rewrite | setup/payoff pair and avoid-premature-resolution note |
| `dialogue_subtext` | Makes dialogue carry hidden intent, power relation, hesitation, or contradiction. | novel chapter, screenplay rewrite | subtext goal and spoken/unspoken separation |
| `visualizable_action` | Converts abstract narration into staged, visible behavior that can later become shots. | screenplay rewrite | visible action anchors and sensory compression |
| `chapter_cliffhanger` | Ends a chapter with a new decision, reveal, danger, or unresolved emotional charge. | novel chapter | closing beat type and next-chapter pull |
| `screenplay_compression` | Compresses prose into playable beats, dialogue, and scene actions without losing intent. | screenplay rewrite | reduction principle and retained story facts |
| `director_blocking_bridge` | Converts narrative or screenplay beats into blocking, camera, performance, and transition needs. | director storyboard | bridge summary for scene-performance and camera routing |

## Relationship To Existing KB Surfaces

`authoring_craft` should sit above the existing KB surfaces as a routing label,
not below them as seed data.

Recommended relationship:

```text
authoring_craft = what story-writing function the retrieval should serve
scene_performance_core = how the beat is performed, staged, and emotionally read
camera_directing_core = how the beat can be framed, moved, paced, or cut
director profiles / rules = abstract visual and staging heuristics for routing
committee handoff rules = how one role hands continuity to the next role
story and dialogue rules = prose-to-beat and beat-to-line craft constraints
```

Practical mapping:

| authoring_craft group | Existing KB surfaces to summarize | Relationship |
| --- | --- | --- |
| hook/desire/conflict/suspense/payoff/cliffhanger | `story_structure_templates`, `character_arc_patterns`, `dialogue_style_rules`, `scene_taxonomy` | provides chapter-level narrative pressure and structure |
| emotional turn/dialogue subtext | `character_arc_patterns`, `dialogue_style_rules`, `scene_performance_core` summaries | converts inner change into visible or speakable evidence |
| visualizable action/screenplay compression | `prompt_templates`, `classic_case_examples`, `scene_performance_core` summaries | compresses prose into scene beats without raw sample leakage |
| scene transition/director blocking bridge | `transition_vocabulary`, `committee_handoff_rules`, `camera_directing_core`, director routing summaries | preserves continuity while preparing shot-level staging |

This relationship keeps director knowledge useful without importing real
director names, adding a `director_style_ref` field, or asking the runtime to
imitate a person or protected work.

## Layered Integration With Director KB

Recommended layer handoff:

1. The authoring layer provides content intent.
2. The screenplay layer provides performable structure.
3. The director layer provides blocking and staging suggestions.
4. The shot layer provides executable shots.
5. The later layer must not overwrite facts from the earlier layer.

Practical rule:

```text
user facts and continuity facts
> authoring craft intent
> screenplay structure
> director blocking
> shot execution
> prompt_text packaging
```

Director KB may contribute only when it supports the inherited content intent.
If a director hint looks visually stronger but changes character desire,
conflict logic, continuity, or user facts, the hint should be dropped or
downgraded in the compact retrieval summary.

Recommended warning codes for future sidecar or telemetry use only:

```text
kb_director_hint_dropped_due_to_story_conflict
authoring_hint_dropped_due_to_user_fact_conflict
continuity_fact_overrode_kb_hint
```

These warning codes are not new seed fields and must not enter the 23-field
main schema in the current freeze.

## Novel Chapter Stage Retrieval

Novel chapter generation should retrieve KB summaries for narrative momentum,
character pressure, and end-of-chapter pull. It should not retrieve raw sample
rows or full prompt text.

Recommended selection order:

1. Classify the chapter function:
   opening hook, goal pursuit, conflict escalation, reversal, aftermath,
   relationship pressure, transition, setup, payoff, or cliffhanger.
2. Choose 1-3 `authoring_craft` tags from:
   `narrative_hook`, `character_desire`, `conflict_escalation`,
   `emotional_turn`, `scene_transition`, `suspense_setup`, `payoff_setup`,
   `dialogue_subtext`, `chapter_cliffhanger`.
3. Match safe KB rules by story function first, scene type second, and sample
   eligibility third.
4. Prefer compact summaries from story, character, dialogue, transition, and
   scene-taxonomy surfaces.
5. Return only selected IDs, rule IDs, and a compressed human-readable summary.

Recommended chapter-stage payload shape:

```text
kb_context_summary = compact narrative craft summary
selected_sample_ids = IDs only
selected_kb_rules = IDs only
user_readable_retrieval_summary = why these craft rules were selected
full_kb_rows_included = 0
```

Chapter retrieval should answer:

- what promise or question pulls the reader forward
- what the focal character wants in this chapter
- what pressure or obstacle escalates the beat
- what visible or emotional turn changes the state
- what detail must be preserved for later payoff
- what ending pressure makes the next chapter necessary

It should not answer by naming a real author, copying a known work, exposing
row bodies, or reconstructing a full prompt package.

## Screenplay Rewrite Stage Retrieval

Screenplay rewriting should turn prose into playable scene material. It should
use authoring craft summaries from the chapter stage as upstream constraints,
then retrieve additional compression and performance summaries.

Recommended selection order:

1. Read the inherited chapter craft summary and selected IDs.
2. Preserve upstream `character_desire`, `conflict_escalation`, `emotional_turn`,
   `scene_transition`, `payoff_setup`, and `chapter_cliffhanger` constraints.
3. Add screenplay-facing tags:
   `screenplay_compression`, `dialogue_subtext`, `visualizable_action`,
   `scene_transition`, and `director_blocking_bridge` when needed.
4. Prefer KB summaries that help split exposition into action, dialogue,
   reaction, silence, and transition beats.
5. Return only compact rewrite guidance plus selected IDs and rule IDs.

Screenplay rewrite retrieval should answer:

- which prose intent must survive compression
- which internal thought becomes visible behavior
- which line carries subtext instead of exposition
- which beat needs a pause, reaction, or turn
- which scene transition must remain continuous
- which details must be handed to director storyboarding

The rewrite stage may inherit chapter summary text, but it should not inherit
raw rows, raw sample text, full prompt bodies, or expanded provenance.

## Director Storyboard Stage Inheritance

Director storyboarding should inherit the previous two stages as compact
constraints, then add staging, blocking, camera, scene-performance, and
transition guidance.

Recommended inheritance chain:

```text
novel chapter summary
-> screenplay rewrite summary
-> director storyboard retrieval summary
```

The director stage should carry forward:

- selected upstream `authoring_craft` tags
- selected upstream sample IDs and rule IDs
- chapter-level desire, conflict, emotional turn, setup/payoff, and cliffhanger
  summaries
- screenplay-level compression, dialogue-subtext, visible-action, and
  transition summaries

Then it may add:

- `director_blocking_bridge` summary
- scene-performance summary
- camera-language summary
- transition/handoff summary
- continuity reminder

Recommended director-stage payload shape:

```text
kb_context_summary = inherited authoring summary plus compact director bridge
selected_sample_ids = IDs only, including inherited IDs when needed
selected_kb_rules = IDs only, including inherited rules when needed
user_readable_retrieval_summary = chapter -> screenplay -> storyboard rationale
full_kb_rows_included = 0
```

The director stage should not reinterpret upstream chapter intent as a new
story. It should convert inherited intent into visible staging while preserving
character desire, conflict pressure, emotional turn, transition logic, and
payoff obligations.

## Sidecar Metadata Candidates Only

The following items may be considered later as sidecar metadata if total
control opens a structured metadata gate. They are not current main-schema
fields.

```text
authoring_craft_tags
authoring_stage
authoring_retrieval_intent
chapter_craft_summary
screenplay_craft_summary
director_bridge_summary
inherited_authoring_summary_ids
inherited_selected_sample_ids
inherited_selected_kb_rules
craft_confidence
stage_transition_reason
setup_payoff_links
dialogue_subtext_goal
visualizable_action_anchors
director_blocking_bridge_tags
```

Sidecar metadata should remain:

- optional
- machine-readable
- summary-only
- detached from the 23-field main schema
- free of real author/director/IP/brand imitation targets
- safe to omit without changing seed validation

## Forbidden Main-Table Additions

The following must not enter the 23-field main schema in the current freeze:

```text
authoring_craft
authoring_craft_tags
authoring_stage
chapter_craft_summary
screenplay_craft_summary
director_bridge_summary
director_style_ref
real_author_ref
real_director_ref
real_ip_ref
brand_style_ref
style_replication_ref
full_prompt_content
raw_kb_row
raw_source_registry
raw_overlay_payload
```

Also forbidden:

- using `authoring_craft` to promote `reserve` rows
- using `authoring_craft` to promote `usable_for_fewshot = No` rows
- copying or reconstructing raw prompt content
- exposing source registry internals or overlay payloads
- naming a real author as a target to imitate
- naming a real director, film, game, brand, or IP as a target style
- turning stage summaries into persistent seed fields without a new gate
- mixing this docs-only draft with QA hardening docs in the same scope

## Retrieval Summary Contract

Any future runtime or product-side use should preserve this compact contract:

```text
stage = novel_chapter | screenplay_rewrite | director_storyboard
authoring_craft_tags = sidecar candidate only
kb_context_summary = compressed, sanitized craft guidance
selected_sample_ids = IDs only
selected_kb_rules = IDs only
user_readable_retrieval_summary = compact explanation
full_kb_rows_included = 0
```

`kb_context_summary` may explain craft function, scene pressure, visible action,
dialogue subtext, transition logic, and director bridge needs. It must not
expand into rows, raw prompt content, full provenance, source registry entries,
or overlay payloads.

## Landing Recommendation

Recommended V0 landing path:

1. Keep this draft docs-only.
2. Let total control decide whether `authoring_craft` becomes future sidecar
   metadata.
3. If a sidecar gate opens, define the sidecar separately from the 23-field
   seed schema.
4. Only after downstream contracts and QA gates are accepted should runtime
   routing consume `authoring_craft` as structured metadata.

## Open Questions For Total Control

1. Should `authoring_craft` remain a docs-only retrieval taxonomy for now, or
   become a future sidecar metadata enum?
2. Should novel chapter generation select at most three `authoring_craft` tags,
   or allow one primary plus secondary tags?
3. Should screenplay rewrite always inherit chapter-stage craft summaries, or
   only inherit them when a chapter ID is available?
4. Should director storyboarding require `director_blocking_bridge` whenever
   inherited prose contains no visible action anchor?
5. Which repo should own the first runtime-facing sidecar contract if total
   control opens that gate: `hope-kb` docs or Hope mainline contracts?
