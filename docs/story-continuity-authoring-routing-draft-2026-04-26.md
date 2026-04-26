# Story Continuity Authoring Routing Draft 2026-04-26

## Scope

This is a docs-only routing draft for long-plot, multi-chapter, multi-shot
generation in Hope.

It does not:

- modify seed files
- modify snapshots
- modify the 23-field main schema
- add director-style reference fields
- promote reserved rows or rows marked unavailable for few-shot use
- add runtime provider integration
- add video generation

The core rule is:

```text
authoring capability = story continuity capability
```

The authoring layer serves content continuity. It does not serve prose
showmanship, real-author imitation, or isolated chapter excitement.

## Main Chain

Recommended chain:

```text
user topic / synopsis
-> KB story-continuity authoring summary
-> novel chapter / story setting
-> screenplay text
-> shot breakdown
-> director storyboard rows / prompt_text
-> finalized storyboard bank
-> continuity summary
-> next chapter / next segment
```

Layer rule:

```text
user facts
> continuity state
> finalized storyboard bank
> authoring continuity summary
> screenplay structure
> director blocking
> shot execution
> prompt_text packaging
```

No later layer may overwrite earlier facts. Authoring hints may guide the next
chapter, but must not rewrite user facts, finalized storyboard rows, or the
current continuity state.

## KB Output Boundary

Allowed:

```text
kb_context_summary
selected_sample_ids
selected_kb_rules
retrieval_trace_user_summary
full_kb_rows_included = 0
```

The KB output is summary-only advisory. It may explain what continuity rule was
selected and why, but it must not expose raw rows, raw prompt content, source
registry internals, overlay payloads, credential values, or provider bodies.

## Source Input Type Rules

| source_input_type | Main Goal | Must Establish / Preserve | Must Not Do |
| --- | --- | --- | --- |
| `synopsis` | establish a sustainable plot skeleton | character goal, conflict causality, emotional progression, next-segment bridge | merely expand pretty sentences |
| `full_story` | preserve existing facts | event order, character motivation, timeline, prop state, setup/payoff ledger | reinvent plot, reorder events, change motivation |
| `novel_chapter` | preserve chapter continuity and next-chapter bridge | chapter hook with cause, emotional state, unresolved pressure, carried props/clues | break long arc for single-chapter climax |
| `screenplay_text` | organize existing script into storyboard-ready structure | scene purpose, action/reaction beats, dialogue action, continuity handoff | drop motivation or event causality during compression |
| `mixed_material` | conservatively identify facts and conflicts | source confidence, known facts, uncertain facts, contradiction list | invent missing logic when uncertain |

Conservative fallback:

```text
if uncertain:
  preserve facts
  summarize ambiguity
  avoid invention
  request or carry forward continuity state
```

## Capability Routing Matrix

| Capability | Continuity Problem Solved | User Topic / Synopsis Use | Novel Chapter / Setting Use | Screenplay Use | Director / Shot Handoff | Cannot Do |
| --- | --- | --- | --- | --- | --- | --- |
| `story_continuity` | plot breaks between segments | build a sustainable beginning, pressure path, and continuation promise | preserve previous facts and state before adding new events | keep scene order and event logic intact | hand over current story facts and unresolved pressure | replace user facts or finalized storyboard facts |
| `character_motivation_continuity` | character desire jumps without cause | define goal, fear, pressure, and decision boundary | keep desire stable unless a justified turn occurs | turn motivation into playable action and dialogue | preserve motivation in blocking and performance emphasis | invent a new motive for stronger drama |
| `conflict_causality` | conflict escalates without reason | define conflict engine and stakes ladder | link obstacle, choice, consequence, and next pressure | compress conflict into scene objective and obstacle | preserve cause/effect in action beats | add spectacle or reversal without cause |
| `emotional_progression` | mood shifts abruptly | define emotional baseline and expected turn | track pressure, reaction, turn, and residue | express emotion through action, silence, line, or pause | pass emotional state to performance and shot emphasis | jump from one emotion to another for effect only |
| `timeline_integrity` | event order or elapsed time becomes incoherent | mark time span and sequence assumptions | preserve flashback, timeskip, and present-time boundaries | split scenes without changing event order | pass time markers to shot and transition planning | reorder events to improve pacing without approval |
| `prop_state_integrity` | objects, clues, weapons, evidence, or costumes contradict prior state | identify key carried items and clue obligations | track object location, owner, condition, and reveal status | preserve prop state through scene compression | pass visible prop state into shots and continuity notes | move or transform objects without story cause |
| `next_scene_bridge` | current scene leaves no usable continuation | define the next pressure, question, or destination | close with a bridge into the next chapter or segment | create handoff beat, transition cue, or unresolved action | pass bridge cue to director blocking and shot entry | end only with decoration or mood |
| `scene_causality_check` | decorative scenes do not advance story | test whether each scene has purpose | remove, merge, or revise scenes without causal function | convert scene purpose into objective / obstacle / turn | keep only shots that support scene purpose | keep a scene just because prose is attractive |
| `setup_payoff_recovery` | planted details are lost before payoff | record setup candidates and payoff obligations | carry clues, promises, relationships, and object states | preserve setup details through compression | ensure shots do not erase future payoff anchors | resolve or discard setup accidentally |
| `chapter_cliffhanger_with_cause` | cliffhanger feels arbitrary | derive ending pressure from goal, conflict, clue, or decision | end with forward pull grounded in previous events | convert cliffhanger into act break or scene-ending pressure | pass unresolved pressure to shot or final image | add shock without causal foundation |
| `screenplay_compression` | prose-to-script loses core facts | identify facts that must survive compression | compress description into playable beat order | keep motivation, event cause, and emotional turn visible | pass action/reaction units to shot breakdown | delete causality to shorten text |
| `director_layer_handoff` | director or shot layer overwrites story | identify non-overwritable facts and intent | summarize content intent for later stages | mark director-safe staging needs | pass only blocking / shot suggestions that serve content | let director hint override story, user fact, or continuity state |

## Risk Tags

Must be blocked or downgraded:

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
```

Risk handling:

| Risk Tag | Action |
| --- | --- |
| `style_over_story` | drop style hint; preserve content |
| `exaggeration_without_cause` | require cause or downgrade |
| `motivation_jump` | preserve prior motivation or require a turn |
| `timeline_break` | preserve event order and mark contradiction |
| `prop_state_conflict` | preserve prior prop state and flag conflict |
| `unfilmable_poetic_drift` | convert only if it supports visible action |
| `continuity_break` | stop expansion and summarize break |
| `real_author_style_imitation` | remove named-style routing |
| `ip_style_clone` | remove protected-style routing |
| `director_hint_overrode_story` | drop director hint |
| `kb_hint_overrode_user_fact` | drop KB hint |
| `continuity_state_ignored` | restore continuity state as priority |
| `finalized_storyboard_bank_ignored` | restore finalized storyboard facts |

## Stage Handoff Contract

Authoring to screenplay:

```text
content_intent
character_motivation_state
conflict_cause_effect_chain
timeline_state
prop_state
setup_payoff_ledger
next_scene_bridge
```

Screenplay to director:

```text
scene_objective
action_reaction_units
dialogue_action_goal
visible_behavior_anchor
continuity_constraints
handoff_warning_codes
```

Director to shot:

```text
blocking_suggestion
shot_execution_hint
continuity_visible_anchor
transition_entry_exit
prompt_text_packaging_note
```

The director and shot layers must serve content. They may intensify staging,
clarify visibility, or improve transition, but they must not change the story
state.

## Mainline Consumption Notes

If this draft is consumed by Hope mainline later, product-side routing should:

1. request compact continuity summaries only
2. carry selected sample IDs and rule IDs only
3. preserve `full_kb_rows_included = 0`
4. treat authoring output as advisory, not authoritative over user facts
5. treat finalized storyboard facts as locked unless the user explicitly opens
   a revision gate
