# KB Full-Chain Content Production Routing Draft 2026-04-26

## Scope

This is a docs-only V0 routing draft for KB-driven full-chain content
production in Hope.

This is not:

- a document-import feature
- a standalone author-writing feature
- a style imitation system
- a runtime provider integration
- a video-generation feature

The product goal is:

```text
light user operation
heavy system output
```

The user may provide only a topic, synopsis, existing document, full story,
chapter, or screenplay material. The system should combine scene type,
continuity state, finalized storyboard facts, and compact KB capability
summaries to produce story, script, director planning, shot rows, clean
`prompt_text`, finalized storyboard output, and Excel export.

## Production Chain

Recommended chain:

```text
user topic / synopsis / document / screenplay material
-> KB capability routing summary
-> story generation or story optimization
-> screenplay structure
-> director blocking
-> shot breakdown
-> storyboard rows
-> prompt_text
-> finalized storyboard bank
-> Excel export
```

The KB contribution must stay advisory and compressed. Product code may use
the summary to guide routing, but the KB summary must not become the source of
truth over user facts or accepted storyboard facts.

## Priority Rule

Hard priority:

```text
user_content_facts
> continuity_state
> finalized_storyboard_bank
> kb_writing_advice
> kb_screenplay_advice
> kb_director_advice
> kb_shot_advice
```

Implications:

- user facts cannot be overwritten by KB
- continuity state cannot be ignored for a stronger scene
- finalized storyboard rows cannot be rewritten by a later authoring hint
- director advice must serve content
- shot advice must serve director blocking and content
- `prompt_text` is packaging, not the source of truth

## Layer 1: Writing Continuity Capability

Purpose:

Keep the story continuable. Character motivation, conflict causality,
emotional progression, timeline, prop state, setup/payoff, and next-segment
bridge must survive long-form generation.

Capability tags:

```text
story_continuity
character_motivation_continuity
conflict_causality
emotional_progression
timeline_integrity
prop_state_integrity
next_scene_bridge
setup_payoff_recovery
chapter_cliffhanger_with_cause
```

Routing rules:

- establish or preserve the plot skeleton before prose expansion
- keep motivation stable unless a justified turn occurs
- require cause and consequence for conflict escalation
- preserve emotional state across scenes and chapters
- keep time order and prop state explicit
- carry setup/payoff obligations into later stages
- end chapters with a bridge that can support the next segment

## Layer 2: Screenplay Structure Capability

Purpose:

Turn story material into filmable script structure without losing key facts,
motivation, scene purpose, or causality.

Capability tags:

```text
screenplay_compression
scene_purpose
dialogue_intent
action_blocking
turning_point_preservation
scene_causality_check
```

Routing rules:

- preserve facts while compressing prose
- split material into scene objective, obstacle, action, reaction, and turn
- convert exposition into dialogue action or visible behavior only when it
  preserves intent
- keep turning points and emotional turns visible
- reject decorative scenes with no causal purpose
- prepare clean handoff to director blocking

## Layer 3: Director Blocking Capability

Purpose:

Serve content by improving staging, performance, rhythm, attention, and
emotional landing. Director advice must not overwrite content.

Capability tags:

```text
director_layer_handoff
performance_focus
blocking_hint
rhythm_hint
visual_focus
continuity_note
```

Routing rules:

- inherit content intent, screenplay structure, and continuity constraints
- suggest performance focus only where it supports character motivation
- suggest blocking only where it clarifies scene purpose or conflict
- adjust rhythm without deleting cause/effect
- guide viewer attention without changing facts
- pass continuity notes to the shot layer

## Layer 4: Shot Language Capability

Purpose:

Turn director blocking into shot tasks, scene type, camera movement, scale,
image description, character action, and clean `prompt_text`.

Capability tags:

```text
shot_intent
shot_scene_type
camera_movement
scene_scale
visualizable_action
prompt_text_cleanliness
seedance_friendly_segmentation
```

Routing rules:

- choose shot intent from scene purpose and director blocking
- keep shot scene type aligned with story and screenplay context
- use camera movement only to clarify action, emotion, or scale
- choose scene scale based on continuity and scene purpose
- convert internal intent into visible action
- keep `prompt_text` clean, bounded, and downstream-friendly
- segment shot work so the renderer can follow it without receiving raw KB
  material

## Input Type Routing

| source_input_type | Primary Goal | Required KB Layers | Notes |
| --- | --- | --- | --- |
| `synopsis` | generate a sustainable plot skeleton | writing continuity, screenplay structure, director blocking, shot language | establish character goals, conflict causality, emotional progression, and next-segment bridge before prose expansion |
| `full_story` | optimize and adapt an existing story | writing continuity, screenplay structure, director blocking, shot language | preserve facts first, then compress to script, organize scene staging, and break into shots |
| `novel_chapter` | continue a chapter and prepare next chapter | writing continuity, screenplay structure, director blocking, shot language | maintain long arc and do not break continuity for a single-chapter climax |
| `screenplay_text` | organize existing script into storyboard-ready structure | screenplay structure, director blocking, shot language, optional writing continuity check | use continuity only to check motivation, causality, timeline, and prop state |
| `mixed_material` | conservatively identify facts and preserve consistency | writing continuity, screenplay structure, then director blocking and shot language | when uncertain, summarize ambiguity and avoid invention |

## Summary-Only KB Contract

Allowed KB output:

```text
kb_context_summary
selected_sample_ids
selected_kb_rules
retrieval_trace_user_summary
full_kb_rows_included = 0
```

`kb_context_summary` should explain:

- selected capability layer
- selected continuity or screenplay reason
- director or shot handoff notes
- blocked risks or downgraded hints
- why the next stage can proceed

It must not expose raw prompt content, source registry values, overlay
payloads, credential values, provider request bodies, or full raw KB rows.

## Risk Blocking

Blocked or downgraded risks:

```text
style_over_story
exaggeration_without_cause
motivation_jump
timeline_break
prop_state_conflict
unfilmable_poetic_drift
continuity_break
real_author_style_imitation
ip_style_clone
director_hint_overrode_story
kb_hint_overrode_user_fact
continuity_state_ignored
finalized_storyboard_bank_ignored
shot_hint_overrode_director_or_content
prompt_text_overrode_story_fact
```

Risk handling:

- if KB conflicts with user facts, drop the KB hint
- if director advice conflicts with story, drop the director hint
- if shot advice conflicts with director blocking or content, drop the shot
  hint
- if `prompt_text` would rewrite story facts, regenerate packaging from the
  accepted facts
- if continuity state is missing, request or synthesize only a conservative
  summary from accepted facts

## Mainline Consumption Recommendation

This draft needs a future Hope mainline consumption gate.

Recommended product-side use:

1. classify input as `synopsis`, `full_story`, `novel_chapter`,
   `screenplay_text`, or `mixed_material`
2. select required KB capability layers
3. request summary-only KB advice
4. generate or optimize content
5. convert to screenplay structure
6. pass content-safe director notes
7. produce shot rows and clean `prompt_text`
8. lock accepted rows in the finalized storyboard bank
9. emit continuity summary for the next chapter or segment
10. export Excel from accepted storyboard rows only
