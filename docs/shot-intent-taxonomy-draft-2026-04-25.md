# Shot Intent Taxonomy Draft 2026-04-25

## Scope

This is a docs-only taxonomy draft for `hope-kb`.

It does not:

- modify the 23-field KB main schema
- modify seed rows
- promote `reserve` rows
- promote rows with `usable_for_fewshot = No`
- add `director_style_ref`
- modify `docs/live-progress.md`
- modify the QA hardening docs-only package
- introduce real director, IP, or brand references

This draft is intended to help future alignment between `hope-kb` taxonomy
work and Hope product-side contracts without reopening the current KB freeze.

## Current Boundary

The current `hope-kb` public seed boundary remains:

```text
23 fields
152 rows
32 GS120-CAND-* rows remain reserve
32 GS120-CAND-* rows remain usable_for_fewshot = No
no director_style_ref in seed main fields
```

The QA hardening packet is separate from this draft and must not be mixed into
the same commit.

## Alignment Target

This draft aligns to the Hope contract:

```text
E:\codex\hope\docs\shot-script-grounding-adaptive-scene-contract-2026-04-25.md
```

The aligned contract fields are:

```text
primary_scene_type
shot_scene_type
shot_intent
adaptation_reason
```

For this draft, these fields are treated as contract-facing planning signals.
They are not proposed as immediate `hope-kb` seed schema changes.

## Core Definitions

`primary_scene_type`

- normalized scene type for the whole request, segment, or scene unit
- represents the broad narrative or staging default
- should remain stable unless the whole segment is reclassified
- should drive broad routing, coarse validation, and default KB matching

`shot_scene_type`

- normalized scene type for a single shot or shot task
- may stay equal to `primary_scene_type` in the common case
- may differ only when shot-local evidence supports a narrower or shifted scene
  classification
- should remain a shot-local override, not a replacement for scene-level
  intent

`shot_intent`

- compact shot-purpose tag
- describes what the shot is trying to do in story or staging terms
- should be combinable with scene typing rather than replacing scene typing
- may support router scoring, example selection, review, and future sidecar
  metadata

`adaptation_reason`

- short explanation for why `shot_scene_type` differs from
  `primary_scene_type`
- required only when a shot-level scene adaptation occurs
- should summarize story evidence rather than expose raw internal payloads

## Relationship Between primary_scene_type And shot_scene_type

Recommended relationship:

1. `primary_scene_type` is the segment-level default.
2. `shot_scene_type` is the shot-level local classification.
3. `shot_scene_type` should default to `primary_scene_type` when no strong
   shot-local evidence exists.
4. `shot_scene_type` may differ only when the shot clearly shifts local
   function, such as:
   - an action beat inside a broader court or dialogue scene
   - a reaction close-up inside a broader battle sequence
   - a transition bridge inside a broader spectacle or campaign segment
5. Whenever they differ, future contracts should require
   `adaptation_reason`.

Recommended interpretation:

```text
primary_scene_type = segment default
shot_scene_type = local shot classification
shot_intent = shot purpose tag
adaptation_reason = why the local shot classification changed
```

This keeps scene type and shot purpose distinct:

- scene type answers "what kind of scene is this"
- shot intent answers "what is this shot doing inside that scene"

## Initial shot_intent Enumeration

Recommended initial enum:

```text
ensemble_establishing
emotional_reaction
combat_impact
power_awaken
spectacle_showcase
transition_bridge
closeup_detail
dialogue_exchange
```

### Intent Definitions

`ensemble_establishing`

- introduces group staging, force layout, geography, formation, or social
  arrangement
- common in war setup, court staging, city overview, council entry, and large
  cast orientation

`emotional_reaction`

- highlights emotional response, hesitation, resolve, fear, grief, shock, or
  silent processing
- often works as a local beat inside larger action, dialogue, or power scenes

`combat_impact`

- emphasizes strike, collision, breach, charge, interception, or tactical
  contact
- intended for high-clarity action punctuation rather than broad spectacle

`power_awaken`

- emphasizes release, activation, ignition, transformation, or decisive reveal
  of force or ability
- suitable for hero highlight, command surge, or key turning-point escalation

`spectacle_showcase`

- emphasizes scale, grandeur, density, environmental force, or visual payoff
- common in siege escalation, massed armies, city growth, and large battlefield
  display

`transition_bridge`

- connects phases, spaces, or beats
- useful for movement between locations, campaign phases, city-state change,
  timeline advance, or handoff into the next action block

`closeup_detail`

- emphasizes a small but meaningful visual anchor
- examples include weapon detail, hand movement, banner wear, interface state,
  dust, flame direction, damage mark, or tactical token detail

`dialogue_exchange`

- emphasizes conversational beat, negotiation, command exchange, court debate,
  briefing, or tactical discussion
- may coexist with tension, restraint, or political maneuvering

## Combination Rules

Recommended rules:

1. `shot_intent` should remain optional and normalized.
2. Each shot should prefer one primary `shot_intent`.
3. A future sidecar metadata layer may allow one secondary intent if needed.
4. `shot_intent` should not replace:
   - `primary_scene_type`
   - `shot_scene_type`
   - `shot_script`
5. When scene type and intent appear to conflict, scene type should define the
   local classification and `shot_intent` should describe the shot function
   within that classification.

Recommended future secondary-intent cases:

```text
ensemble_establishing + spectacle_showcase
combat_impact + emotional_reaction
dialogue_exchange + closeup_detail
transition_bridge + spectacle_showcase
power_awaken + closeup_detail
```

## scene_type x shot_intent Recommended Mapping

The table below is a planning guide, not a hard validator.

| scene_type | recommended shot_intent | notes |
| --- | --- | --- |
| war_battlefield | `ensemble_establishing`, `combat_impact`, `spectacle_showcase`, `emotional_reaction`, `transition_bridge`, `closeup_detail` | supports army layout, impact beats, aftermath reactions, and tactical inserts |
| war_siege | `ensemble_establishing`, `combat_impact`, `spectacle_showcase`, `closeup_detail`, `transition_bridge` | supports multi-corps攻城, breach points, fire lines, and siege escalation |
| war_command | `dialogue_exchange`, `ensemble_establishing`, `closeup_detail`, `emotional_reaction` | supports command relay, map view, formation brief, and silent resolve |
| court_intrigue | `dialogue_exchange`, `emotional_reaction`, `closeup_detail`, `transition_bridge` | supports debate, suspicion, pause beats, and political handoff |
| hero_highlight | `power_awaken`, `spectacle_showcase`, `emotional_reaction`, `closeup_detail` | supports awakening, aura release, decisive turn, and hero detail |
| slg_sandbox | `ensemble_establishing`, `transition_bridge`, `closeup_detail`, `spectacle_showcase` | supports sandbox viewport, route planning, and territory overview |
| slg_march | `transition_bridge`, `ensemble_establishing`, `closeup_detail` | supports marching lines, route shift, deployment handoff, and motion cues |
| slg_city_build | `spectacle_showcase`, `transition_bridge`, `closeup_detail`, `ensemble_establishing` | supports city evolution, phased build-up, infrastructure reveal, and overview |
| slg_battle_report | `closeup_detail`, `dialogue_exchange`, `transition_bridge` | supports report UI, result reading, tactical summary, and post-battle explanation |
| dialogue_scene | `dialogue_exchange`, `emotional_reaction`, `closeup_detail` | supports exchange rhythm, pause, and facial emphasis |
| transition_scene | `transition_bridge`, `ensemble_establishing`, `spectacle_showcase` | supports bridge shots, travel compression, and time/location shifts |

## SLG And CN War Constraints

For the current KB enhancement area, the following interpretation is
recommended:

### CN War / Court / Command

- prioritize group geography, rank order, formation readability, command relay,
  reaction timing, and tactical consequence
- use `dialogue_exchange` for court strategy, negotiation, or command relay
- use `combat_impact` for decisive clash rather than for every battle frame
- use `emotional_reaction` for oath, hesitation, fear, grief, or resolve beats

### SLG

SLG intent usage should remain grounded in visible shot grammar only:

- sandbox viewport
- marching route
- city evolution
- battle report UI

Recommended SLG pairings:

```text
sandbox viewport -> ensemble_establishing / transition_bridge
marching route -> transition_bridge / closeup_detail
city evolution -> spectacle_showcase / transition_bridge
battle report UI -> closeup_detail / dialogue_exchange
```

This keeps SLG taxonomy tied to visible shot organization instead of abstract
system labels.

## docs-only / sidecar metadata Landing Suggestions

Recommended order of landing:

1. docs-only taxonomy draft
2. sidecar metadata experiment if total control opens a structured layer
3. only later consider contract adoption
4. do not reopen the 23-field KB seed schema inside the current freeze

Recommended sidecar-only fields for future exploration:

```text
primary_scene_type
shot_scene_type
shot_intent
adaptation_reason
secondary_shot_intent
```

Sidecar metadata should:

- remain machine-readable
- avoid real director, IP, and brand references
- avoid copying raw `prompt_body`
- avoid changing reserve status or few-shot eligibility
- remain optional until downstream contracts are accepted

## Alignment With Hope Contract

This draft aligns with the Hope contract in the following ways:

1. `shot_intent` remains an optional, normalized shot-purpose signal.
2. `shot_intent` does not replace `shot_script`.
3. `shot_scene_type` defaults to `primary_scene_type` unless local evidence
   supports an adaptation.
4. `adaptation_reason` is needed only when the shot-level scene type changes.
5. The current work stays docs-only and does not modify the KB seed schema.

This draft does not yet propose:

- a runtime implementation
- a desktop UI change
- a `hope-kb` seed schema change
- a `director_style_ref` field
- a promotion path for reserve rows

## Explicit Non-Goals

This draft does not:

- change the KB 23-field main schema
- change the current 152-row count
- promote `reserve` rows
- promote rows with `usable_for_fewshot = No`
- add `director_style_ref`
- expose real director names
- expose real film, game, or brand names
- merge the QA hardening packet into taxonomy work

## Open Questions For Total Control

1. Should `shot_intent` remain docs-only for now, or move later into sidecar
   metadata?
2. Should sidecar metadata allow one secondary intent, or keep exactly one
   normalized primary intent?
3. Which `scene_type` enum should be treated as the canonical source for
   `primary_scene_type` and `shot_scene_type` normalization across repos?
4. Should `adaptation_reason` stay product-contract-only until runtime
   adoption, or should KB-side docs define a narrower controlled vocabulary
   first?
