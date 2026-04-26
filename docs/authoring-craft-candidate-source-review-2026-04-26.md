# Authoring Craft Candidate Source Review 2026-04-26

## 1. Research Boundary

This is a docs-only research note for the V0 Hope authoring / screenplay /
director chain.

It does not:

- modify seed files
- modify snapshots
- modify the 23-field main schema
- add `director_style_ref`
- promote `reserve` rows
- promote rows with `usable_for_fewshot = No`
- add runtime provider integration
- add video generation

This note studies public author, award, and screenplay-reference sources only
as inputs for abstract `authoring_craft` capability labels. It does not create
any product-facing author-style target.

Core priority:

```text
content
> screenplay_structure
> director_blocking
> shot_storyboard
> prompt_text
```

## 2. Why The Author Group Is Not A Style Library

The author group is a content craft source pool, not a style replication pool.

Allowed abstraction:

- story hook
- character desire
- character pressure
- conflict engine
- emotional turn
- suspense setup
- setup/payoff recovery
- scene purpose
- visualizable action
- chapter hook
- novel-to-screenplay compression
- director-layer handoff

Forbidden use:

- real author style targeting
- IP or brand style replication
- generic prose beautification with no story function
- director spectacle that conflicts with user facts, continuity facts, or
  established character intent

Every candidate below should be treated as a source of craft patterns and
quality-control questions. The final system should never tell a model to copy
a named author's prose, a known work, a brand, or a protected visual identity.

## 3. Web Reference Sources

Checked sources on 2026-04-26:

| Source | URL | Use In This Note |
| --- | --- | --- |
| Nobel Prize in Literature official list | https://www.nobelprize.org/prizes/lists/all-nobel-prizes-in-literature/all/ | international literary narrative, memory, trauma, voice, and form references |
| China Writers Association / Maodun Literature Prize winners | https://www.chinawriter.com.cn/GB/n1/2019/0816/c405645-31300293.html | Chinese long-form realism, historical scale, regional life, group chronicle |
| Maodun Literature Prize rules | https://wyb.chinawriter.com.cn/Pad/content/202303/15/content69180.html | long-form eligibility and long-novel evaluation boundary |
| SFWA Nebula Awards 2025 winners | https://events.sfwa.org/our-2025-conference/our-2025-nebula-awards-winners/ | science fiction / fantasy forms, speculative conflict, genre compression |
| World Fantasy Awards winners | https://worldfantasy.org/awards/winners/ | fantasy, myth, fable, weird, and symbolic structure references |
| National Book Foundation / National Book Awards | https://www.nationalbook.org/national-book-awards/ | American contemporary fiction, social relation, youth, translated literature |
| Booker Prize winners / longlists | https://thebookerprizes.com/the-booker-library/features/full-list-of-booker-prize-winners-shortlisted-and-longlisted-authors | sustained English-language long-form fiction and complex structure references |
| International Booker Prize 2025 | https://thebookerprizes.com/the-booker-library/prize-years/international/2025 | translated fiction, short-story cycle, global language-area references |
| Science Fiction World / Galaxy Award winners | https://www.sfw.com.cn/go-a2244.htm | Chinese speculative fiction, technical imagination, genre escalation |
| China Television Feitian Awards | https://www.cnapc.cn/xzt/zgdsftj/index.html | domestic screenwriting and performable TV-drama structure references |

These sources are not enough for final product adoption. They are sufficient
for a V0 docs-only candidate map and later sidecar-metadata discussion.

## 4. Domestic Candidate Sources

| Candidate Source | Country / Language Area | Main Type | Abstract Capability | Hope Scenario | Chapter | Screenplay | Storyboard | Risk | Docs-only | Sidecar Candidate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Maodun Prize winner pool | China / Chinese | long-form realism, historical fiction, regional fiction | long-arc causality, era pressure, group relation, social stakes | long serial chapter planning, reality-grounded group drama | Yes | Yes | Indirect | medium if author names become style targets | Yes | Yes, as award-source cluster only |
| Liang Xiaosheng / "The World" reference cluster | China / Chinese | family chronicle, social realism | multi-generation pressure, work-unit/community relation, chapter-to-era scale | realistic group scenes, family/social pressure, slow emotional payoff | Yes | Yes | Indirect | medium | Yes | Maybe, as abstract group-chronicle tag |
| Qiao Ye / "Baoshui" reference cluster | China / Chinese | rural contemporary fiction | community observation, local scene purpose, everyday conflict | village/town chapter scenes, character desire in community network | Yes | Yes | Indirect | medium | Yes | Maybe |
| Liu Liangcheng / "Bemba" reference cluster | China / Chinese | mythic / grassland / memory fiction | nonlinear time, mythic causality, fable-like emotional turn | mythic history, ritualized chapter movement | Yes | Maybe | Maybe | medium | Yes | Docs-only first |
| Sun Ganlu / "A Panorama of Rivers and Mountains" reference cluster | China / Chinese | historical / espionage / urban revolutionary narrative | coded suspense, concealed identity, mission pressure | historical suspense, intelligence scenes, hidden desire | Yes | Yes | Yes | medium | Yes | Maybe |
| Dong Xi / "Echoes" reference cluster | China / Chinese | suspense / psychological realism | investigation pressure, echoing clues, moral ambiguity | mystery chapters, clue recurrence, emotional turn | Yes | Yes | Yes | medium | Yes | Maybe |
| Ge Fei / "Jiangnan Trilogy" reference cluster | China / Chinese | modern history, intellectual / regional narrative | historical layering, desire under ideology, recurring motif | complex historical chapters and memory transitions | Yes | Maybe | Indirect | medium | Yes | Docs-only first |
| Jin Yucheng / "Blossoms" reference cluster | China / Shanghainese-Chinese urban narration | city ensemble, dialogue-driven realism | oral texture, multi-voice city scene, social subtext | urban ensemble dialogue and scene texture | Yes | Yes | Indirect | high if voice is copied | Yes | No until paraphrase-safe |
| Bi Feiyu / "Massage" reference cluster | China / Chinese | sensory realism, character ensemble | embodied perception, dignity pressure, interpersonal blocking | character-pressure scenes and sensory viewpoint conversion | Yes | Yes | Yes | medium | Yes | Maybe |
| Liu Zhenyun / "One Sentence Is Ten Thousand Sentences" reference cluster | China / Chinese | social satire, dialogue / loneliness | dialogue subtext, social contradiction, repeated desire frustration | dialogue-heavy chapters and interpersonal pressure | Yes | Yes | Indirect | high if comedic voice is copied | Yes | Docs-only first |
| Mai Jia / "Decoded" / "Plot Against" reference cluster | China / Chinese | espionage, code-breaking, suspense | puzzle engine, concealed information, mission stakes | suspense setup, clue payoff, chapter hook | Yes | Yes | Yes | medium | Yes | Maybe |
| Chen Yan / "The Protagonist" reference cluster | China / Chinese | performance-life fiction, stage tradition | scene purpose, performer pressure, theatrical transformation | performer chapters, backstage-to-stage conversion | Yes | Yes | Yes | medium | Yes | Maybe |

Domestic pool conclusion: the V0 Chinese source side should prefer award
clusters and capability tags over named-author routing. The safest initial
sidecar candidates are `group_chronicle_pressure`, `community_scene_purpose`,
`suspense_clue_recurrence`, `dialogue_subtext_pressure`, and
`performance_scene_bridge`.

## 5. International Candidate Sources

| Candidate Source | Country / Language Area | Main Type | Abstract Capability | Hope Scenario | Chapter | Screenplay | Storyboard | Risk | Docs-only | Sidecar Candidate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Nobel Literature laureate pool | international / multilingual | literary fiction, drama, poetry, testimony | narrative innovation, memory, trauma, voice, moral pressure | high-level authoring craft review and emotional continuity | Yes | Maybe | Indirect | high if laureate names become style targets | Yes | Yes, as award-source cluster only |
| Han Kang reference cluster | Korea / Korean | historical trauma, body, fragile interiority | emotional turn, bodily vulnerability, trauma pressure | quiet emotional chapters, fragile-life scenes | Yes | Maybe | Indirect | high | Yes | Docs-only first |
| Jon Fosse reference cluster | Norway / Norwegian | prose and drama | silence, repetition, unsaid emotion, performable minimalism | quiet scenes, subtext, pause beats | Yes | Yes | Yes | high | Yes | Docs-only first |
| Annie Ernaux reference cluster | France / French | memory, class, autobiographical social witness | personal memory under collective restraint | reflective chapters and social memory transitions | Yes | Maybe | Indirect | high | Yes | Docs-only first |
| Abdulrazak Gurnah reference cluster | Tanzania / English | migration, colonial aftermath, displacement | cross-cultural pressure, exile desire, moral ambiguity | migration arcs and identity pressure | Yes | Yes | Indirect | medium | Yes | Maybe |
| Kazuo Ishiguro reference cluster | UK / English | memory, unreliable narration, emotional withholding | delayed realization, restrained inner desire, late payoff | first-person memory chapters and hidden loss | Yes | Yes | Indirect | high | Yes | Docs-only first |
| Svetlana Alexievich reference cluster | Belarus / Russian | polyphonic testimony | multi-voice testimony, collective trauma structure | documentary-like group voice and memory montage | Yes | Yes | Maybe | high for voice copying | Yes | Docs-only first |
| Booker Prize winner pool | UK / Ireland publication / English | sustained long-form fiction | long-form structure, contemporary social relation, formal innovation | novel arc design and chapter endurance | Yes | Maybe | Indirect | medium | Yes | Yes, as prize-source cluster |
| International Booker pool | global translated fiction | translated literary fiction, short-story cycle | cross-cultural compression, translated voice, sequence architecture | global narrative structures and short cycle logic | Yes | Maybe | Indirect | medium | Yes | Yes, as prize-source cluster |
| National Book Awards pool | United States / English and translated literature | contemporary fiction, translated literature, youth literature | social issue framing, identity pressure, contemporary relation | social-relation chapters and youth/family pressure | Yes | Maybe | Indirect | medium | Yes | Yes, as award-source cluster |

International pool conclusion: do not route by author name in generation. Use
award-level clusters and capability labels such as `memory_pressure`,
`cross_cultural_displacement`, `unspoken_emotion`, `polyphonic_testimony`, and
`long_form_social_relation`.

## 6. Genre Candidate Sources

| Candidate Source | Country / Language Area | Main Type | Abstract Capability | Hope Scenario | Chapter | Screenplay | Storyboard | Risk | Docs-only | Sidecar Candidate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| SFWA Nebula Award pool | mostly US / English, international SFF | science fiction, fantasy, game writing, dramatic presentation | speculative premise, escalation engine, form-fit length control | sci-fi/fantasy chapter hooks and world-conflict logic | Yes | Yes | Yes | medium | Yes | Yes, as award-source cluster |
| John Wiswell / 2025 Nebula novel winner cluster | US / English | fantasy / speculative empathy | unusual premise with emotional accessibility | creature/otherness POV, emotional hook | Yes | Maybe | Indirect | medium | Yes | Docs-only first |
| A.D. Sui / 2025 Nebula novella cluster | international English SFF | novella / speculative conflict | compact speculative escalation | novella-like mission or compact arc | Yes | Yes | Maybe | medium | Yes | Docs-only first |
| World Fantasy Awards pool | international / English-led fantasy | fantasy, fable, myth, weird fiction | mythic premise, symbolic object, strangeness as pressure | mythic chapter hooks, fable logic, symbolic payoff | Yes | Yes | Yes | medium | Yes | Yes, as award-source cluster |
| Patricia A. McKillip / early World Fantasy cluster | US / English | mythic fantasy | lyrical myth structure, object-based desire | fantasy chapter motive and symbolic quest | Yes | Maybe | Maybe | high if prose texture is copied | Yes | Docs-only first |
| Susanna Clarke / historical-fantasy cluster | UK / English | alternate history, magic system | social rule + magic rule interaction | historical fantasy exposition compression | Yes | Yes | Yes | medium | Yes | Maybe |
| Nnedi Okorafor / Africanfuturism cluster | Nigeria-US / English | science fantasy, Afrofuturist / Africanfuturist fiction | cultural worldbuilding, embodied speculative stakes | worldbuilding with local belief and technology pressure | Yes | Yes | Yes | medium | Yes | Maybe |
| Galaxy Award pool | China / Chinese | Chinese science fiction | technical imagination, social speculation, premise escalation | Chinese sci-fi worldbuilding and technology conflict | Yes | Yes | Yes | medium | Yes | Yes, as award-source cluster |
| Liu Cixin / Galaxy reference cluster | China / Chinese | hard SF, macro-scale speculative conflict | scale escalation, technology-pressure engine, civilization stakes | large-scale sci-fi planning and conflict engine | Yes | Yes | Yes | high due IP/style risk | Yes | Docs-only first |
| Wang Jinkang / Galaxy reference cluster | China / Chinese | biological / ethical SF | bioethical conflict, idea-to-plot pressure | technology ethics and dilemma chapters | Yes | Yes | Maybe | medium | Yes | Maybe |
| Han Song reference cluster | China / Chinese | surreal / institutional SF | uncanny system pressure, bureaucratic nightmare logic | dark speculative institutions and social pressure | Yes | Yes | Maybe | high | Yes | Docs-only first |
| Jiang Bo reference cluster | China / Chinese | hard SF / space opera | mission escalation, technical stakes, actionable worldbuilding | space / technology action chapters | Yes | Yes | Yes | medium | Yes | Maybe |
| Bao Shu reference cluster | China / Chinese | speculative reworking, meta-SF | premise inversion, alternate route, idea recombination | premise exploration and speculative alternatives | Yes | Maybe | Indirect | high if linked to known IP | Yes | Docs-only first |
| Domestic screenwriting awards / Feitian script pool | China / Chinese | TV drama, series writing | performable structure, episode turn, scene objective, dialogue action | novel-to-screenplay compression and act/episode planning | Maybe | Yes | Yes | medium | Yes | Yes, as screenwriting-source cluster |

Genre pool conclusion: SFF sources are valuable for premise pressure,
worldbuilding constraints, and chapter hooks, but many named works carry high
style/IP risk. V0 should begin with award-source clusters and neutral
capability tags rather than named-work routing.

## 7. Author Capability To Hope Scenario Mapping

| Capability | Primary Hope Use | Candidate Source Families | Stage Fit | Sidecar Fit |
| --- | --- | --- | --- | --- |
| story hook | first page / first scene pull | Nebula, Galaxy, Booker, Maodun suspense works | chapter | yes |
| character desire | protagonist objective, blocked want | Maodun realism, National Book Awards, Nobel memory fiction | chapter, screenplay | yes |
| character pressure | social, bodily, family, institutional, technical pressure | Maodun, Nobel, Galaxy, Feitian | chapter, screenplay | yes |
| conflict engine | repeated force that keeps the story moving | Nebula, Galaxy, Booker, Maodun historical novels | all | yes |
| emotional turn | state change with visible evidence | Nobel, National Book Awards, Maodun realism | chapter, screenplay | yes |
| suspense setup | clue, withheld information, threat, or delayed reveal | Mai Jia cluster, Dong Xi cluster, Nebula, Galaxy | chapter, screenplay, storyboard | yes |
| setup/payoff recovery | planted detail that later becomes meaningful | Booker, Nebula, Galaxy, Maodun | all | yes |
| scene purpose | why this scene must exist | Feitian, Maodun, screenplay/drama clusters | screenplay, storyboard | yes |
| visualizable action | convert thought into performable behavior | Feitian, World Fantasy, director KB | screenplay, storyboard | yes |
| chapter hook | final beat that pulls next chapter | Nebula, Galaxy, suspense clusters | chapter | yes |
| novel-to-screenplay compression | reduce prose to action, dialogue, reaction, silence | Feitian, drama/script award pools | screenplay | yes |
| director-layer handoff | preserve content intent while adding staging | Feitian, director KB, transition rules | storyboard | yes |

## 8. Author Layer To Screenplay Layer

Recommended handoff:

```text
authoring_content_intent
-> playable_scene_objective
-> dialogue_action
-> visible_behavior
-> transition_or_cut_need
```

Rules:

- screenplay compression must preserve the original content intent
- internal thought should become action, silence, object use, gesture, or
  spoken conflict only when the conversion is justified
- exposition should be split into scene objective, obstacle, reaction, and
  turn
- chapter hook can become act break, episode beat, or scene-ending pressure
- screenplay notes should never replace user facts

Potential sidecar-only fields:

```text
authoring_content_intent
screenplay_scene_objective
dialogue_action_goal
visible_behavior_anchor
compression_loss_warning
```

## 9. Author Layer To Director KB

Recommended handoff:

```text
content intent
-> performable structure
-> director blocking suggestion
-> shot execution hint
```

Director KB may add:

- blocking relation
- staging pressure
- camera readability concern
- transition continuity
- performance emphasis

Director KB must not add:

- new facts not supported by the user or previous stage
- real author/director/IP/brand targets
- a visually impressive beat that changes the content intent
- shot ideas that erase setup/payoff obligations

Recommended warning codes:

```text
kb_director_hint_dropped_due_to_story_conflict
authoring_hint_dropped_due_to_user_fact_conflict
continuity_fact_overrode_kb_hint
```

These are future sidecar / telemetry candidates only. They are not schema
fields in the current freeze.

## 10. Risks And Forbidden Items

High-risk cases:

- named author used as generation target
- named work used as story or style target
- award label treated as permission to reproduce voice
- genre source used to recreate protected settings, characters, terms, or plot
- director hint overrides user-provided story facts
- screenplay compression removes a required emotional turn

Forbidden in this V0 path:

- seed edits
- snapshot edits
- 23-field main schema edits
- director-style reference fields
- reserve or few-shot status promotion
- provider/runtime integration
- full source-payload disclosure
- raw row disclosure
- raw prompt-content disclosure
- overlay-payload disclosure

## 11. V0 Recommended Minimal Author Capability Set

Recommended minimal V0 capability tags:

```text
story_hook
character_desire
character_pressure
conflict_engine
emotional_turn
suspense_setup
setup_payoff_recovery
scene_purpose
visualizable_action
chapter_hook
novel_to_screenplay_compression
director_layer_handoff
```

Recommended stage grouping:

```text
novel_chapter:
  story_hook
  character_desire
  character_pressure
  conflict_engine
  emotional_turn
  suspense_setup
  setup_payoff_recovery
  chapter_hook

screenplay_rewrite:
  scene_purpose
  visualizable_action
  novel_to_screenplay_compression
  dialogue_action_goal
  emotional_turn

director_storyboard:
  director_layer_handoff
  visualizable_action
  scene_purpose
  setup_payoff_recovery
  continuity_warning
```

## 12. Sidecar Metadata Recommendation

Safe to consider for sidecar metadata:

```text
authoring_capability_tags
source_family
stage_fit
scenario_fit
risk_level
content_priority_rank
screenplay_bridge_needed
director_bridge_needed
warning_codes
```

Docs-only for now:

```text
named_author_notes
named_work_notes
award_history_notes
translation_context_notes
style-risk notes
```

Do not move named author or named work fields into runtime selection unless
total control opens a separate governance gate. Even then, prefer award-source
clusters, capability tags, and sanitized summaries over named-source routing.

## 13. Next-Step Recommendation

V0 should land in this order:

1. Keep this source review docs-only.
2. Align the capability names with `authoring_craft` taxonomy.
3. Open a sidecar metadata gate only after total control approves.
4. If sidecar opens, start with source-family clusters and warning codes.
5. Keep all runtime text-generation payloads summary-only and IDs-only.

