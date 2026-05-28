const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const repoRoot = path.resolve(__dirname, "..");
const reviewedDir = path.join(repoRoot, "knowledge", "reviewed_wiki");
const mappingPath = path.join(repoRoot, "knowledge", "mappings", "wiki-to-runtime-mapping.v0.2.json");
const snapshotPath = path.join(repoRoot, "knowledge", "runtime_snapshots", "latest.candidate.json");
const sampleMappingPath = path.join(repoRoot, "samples", "wiki-to-runtime-mapping.sample.json");
const sampleSnapshotPath = path.join(repoRoot, "samples", "runtime-kb-snapshot.sample.json");

const catalog = {
  "rw-visual-master-consistency": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-visual-master-consistency",
    ruleId: "rule:visual-master-consistency",
    title: "Visual master consistency",
    purpose: "Carry a compact visual master into storyboard rows while preserving accepted facts.",
    axes: ["visual_master", "character_continuity", "visual_grounding"],
    directives: [
      "Carry subject identity, world, palette, lighting logic, material feel, era, and key props into every storyboard row.",
      "Vary framing and action without resetting the visual world.",
      "Treat visual master as continuity guidance, not as a source of new facts.",
    ],
    negatives: ["Do not overwrite accepted body facts.", "Do not expose source images, OCR text, local paths, or source registers."],
    targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
  },
  "rw-camera-language-grammar": {
    wikiType: "shot_language_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-camera-language-grammar",
    ruleId: "rule:camera-language-grammar",
    title: "Camera language grammar",
    purpose: "Bind shot function, scale, angle, movement, composition, and adjacent-shot relation to PWA storyboard fields.",
    axes: ["camera_language", "shot_intent", "structure"],
    directives: [
      "Each row should include shot function, shot scale, camera angle, camera movement, and composition logic when useful.",
      "Prefer precise camera language over broad cinematic quality words.",
      "Avoid repeating the same framing and movement across all rows.",
    ],
    negatives: ["Do not use camera vocabulary as empty decoration.", "Do not expose internal rule ids in prompt_text."],
    targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
  },
  "rw-material-light-air-physicality": {
    wikiType: "shot_language_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-material-light-air-physicality",
    ruleId: "rule:material-light-air-physicality",
    title: "Material light air physicality",
    purpose: "Add scene-bound material feedback, light direction, and air medium cues to visible frame and prompt text.",
    axes: ["physicality", "visual_grounding", "prompt_boundary"],
    directives: [
      "Add visible material weight, resistance, wetness, roughness, wear, or reflection when tied to action.",
      "Add light direction, hardness, rim light, side backlight, haze, rain, dust, smoke, steam, sparks, or water reflection only when supported by scene logic.",
      "Keep physical detail bound to current action.",
    ],
    negatives: ["Do not add unsupported world facts or weather effects.", "Do not replace action with texture lists."],
    targets: ["director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result"],
  },
  "rw-expression-physicalization": {
    wikiType: "scene_expression_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-expression-physicalization",
    ruleId: "rule:expression-physicalization",
    title: "Expression physicalization",
    purpose: "Convert abstract emotion into visible face, posture, breath, and blocking cues that fit shot scale.",
    axes: ["performance", "character_action", "visual_grounding"],
    directives: [
      "Convert abstract emotion into visible performance cues.",
      "Use close-up cues such as jaw tension, cheek muscle, locked gaze, nasal flare, lip pressure, or trembling breath only when shot scale supports them.",
      "Use posture, silhouette, blocking, and motion for wide shots.",
    ],
    negatives: ["Do not output abstract mood without visible action.", "Do not force invisible micro-expression cues into wide shots."],
    targets: ["director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
  },
  "rw-transition-motion-dynamics": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-transition-motion-dynamics",
    ruleId: "rule:transition-motion-dynamics",
    title: "Transition motion dynamics",
    purpose: "Use transitions and motion dynamics to express rhythm, continuity, action pressure, and emotional direction.",
    axes: ["motion_dynamics", "continuity", "structure"],
    directives: [
      "Use transitions and motion dynamics to express rhythm and continuity.",
      "Select fade, dissolve, flash, wipe, push, pull, match cut, or montage by scene function.",
      "Select horizontal, vertical, diagonal, curved, radial, spiral, or S-curve motion by action pressure and emotional direction.",
    ],
    negatives: ["Do not add decorative transitions unrelated to scene function.", "Do not break action continuity or accepted event order."],
    targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
  },
  "rw-seedance2-storyboard-field-discipline": {
    wikiType: "shot_language_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-seedance2-storyboard-field-discipline",
    ruleId: "rule:seedance2-storyboard-field-discipline",
    title: "Seedance2 storyboard field discipline",
    purpose: "Turn Seedance-oriented camera, expression, scene, light, and composition guidance into existing PWA storyboard fields without schema drift.",
    axes: ["camera_language", "shot_size", "visual_grounding", "performance", "prompt_boundary"],
    directives: [
      "Choose one primary camera movement and one compatible viewpoint for the current beat before compiling prompt_text.",
      "Bind shot_size before expression detail: close shots may carry eye, lip, breath, and hand tension; wide shots should carry silhouette, spacing, route, and group motion.",
      "Build visual_description from scene-supported space, subject placement, material, light direction, color temperature, air medium, composition, and depth cues.",
      "Translate emotion into visible action such as gaze, breath, posture, hand motion, step, recoil, pause, or blocking relation.",
      "Use style_profile only after scene type, accepted facts, shot_size, and visible row purpose are already coherent.",
      "Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints only.",
    ],
    negatives: [
      "Do not add anime-only camera vocabulary when the scene style is grounded, documentary, or otherwise not anime-compatible.",
      "Do not put camera, style, composition, source, or KB terms into person.",
      "Do not force tiny facial muscle detail into wide, aerial, group, or battlefield-scale shots.",
      "Do not use light, color, atmosphere, or composition as disconnected decoration.",
      "Do not let style_profile become a second prompt template or override existing final-field responsibilities.",
      "Do not expose source or governance metadata in PWA output.",
    ],
    targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
  },
  "rw-person-identity-lock": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-field-aware-entity",
    ruleId: "rule:field-aware-entity",
    title: "Person identity lock",
    purpose: "Keep the person field limited to accepted story characters or explicit role labels while blocking location, action, scene, camera, style, KB, and placeholder terms.",
    axes: ["field_boundary", "entity_integrity", "export_safety"],
    directives: [
      "person must preserve only accepted-body characters or explicit role labels.",
      "Reject location, action fragment, scene term, camera term, style term, KB term, slash, dash, or empty placeholder in person.",
      "When the person field is uncertain, leave it empty or repair from accepted facts instead of inventing a new name.",
    ],
    negatives: [
      "Do not invent character names.",
      "Do not put location, action, camera, visual style, source, or KB terms into person.",
      "Do not expose governance wording in user exports.",
    ],
    targets: ["validation_rule_packs", "scene_mappings", "selected_kb_rules", "negative_constraints"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    contextSummary: "Person identity is locked to accepted characters or explicit role labels; locations, actions, scene terms, camera terms, style terms, KB terms, and placeholders must not enter person.",
    reviewedAtBucket: "2026-05-25",
  },
  "rw-scene-expression-visible-action": {
    wikiType: "scene_expression_rule",
    packCollection: "writing_rule_packs",
    ruleGroup: "writing_group",
    packId: "wg-scene-expression-visible-action",
    ruleId: "rule:scene-expression-visible-action",
    title: "Scene expression visible action",
    purpose: "Convert accepted story material into filmable, visible, and storyboard-ready anime scene expression without losing facts, motivation, or causal purpose.",
    axes: ["visible_action", "scene_goal", "causality", "anime_storyboard"],
    directives: [
      "Expose the scene objective, obstacle, action, reaction, and turning point as visible storyboard material.",
      "Convert explanation into visible behavior, frame relation, dialogue action, or environmental consequence only when accepted intent is preserved.",
      "Keep conflict escalation tied to cause, consequence, and an action or emotion transition.",
    ],
    negatives: [
      "Do not let style overwrite accepted story facts.",
      "Do not replace visible action with abstract mood labels.",
      "Do not use dialogue to invent missing motives, relationships, or facts.",
    ],
    targets: ["writing_rule_packs", "director_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary"],
    actions: ["expand_story", "rewrite_story", "create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-director-scheduling-core",
        title: "Director scheduling support",
        purpose: "Support visible scene expression through blocking, rhythm, and attention handoff.",
        axes: ["blocking", "rhythm", "visual_attention"],
        directives: [
          "Use staging and attention to make the accepted scene purpose readable.",
          "Keep director hints subordinate to accepted facts and scene objective.",
        ],
        negatives: ["Do not introduce new people, places, or world rules."],
      },
    ],
  },
  "rw-dialogue-evidence-lock": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-dialogue-evidence-lock",
    ruleId: "rule:dialogue-evidence-lock",
    title: "Dialogue evidence lock",
    purpose: "Keep dialogue and narration grounded in user-provided text, accepted facts, explicit character intent, or the current scene goal.",
    axes: ["dialogue_grounding", "fact_integrity", "anime_storyboard", "prompt_boundary"],
    directives: [
      "Use dialogue or narration only when it is supported by accepted facts, user text, clear character intent, or current scene purpose.",
      "When evidence is weak, prefer visible action, reaction, silence, or a repair request instead of invented dialogue.",
      "Keep dialogue and narration aligned with person identity, scene causality, and final-field boundaries.",
    ],
    negatives: [
      "Do not treat this rule as an automatic dialogue template.",
      "Do not invent promises, relationships, motives, exposition, or facts to justify a line.",
      "Do not leak source evidence or internal governance text into dialogue or prompt_text.",
    ],
    targets: ["validation_rule_packs", "writing_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["rewrite_story", "create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    contextSummary: "Dialogue and narration require accepted textual or factual grounding; unsupported dialogue should be replaced by visible anime action, silence, or repair.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-director-scheduling-core": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-director-scheduling-core",
    ruleId: "rule:director-scheduling-core",
    title: "Director scheduling core",
    purpose: "Guide performance focus, blocking, rhythm, visual attention, and scene-to-shot handoff while preserving accepted story facts.",
    axes: ["blocking", "visual_attention", "rhythm", "scene_handoff"],
    directives: [
      "Bind blocking, gaze, spacing, reaction, and rhythm to scene objective, conflict, relation, or reveal.",
      "Use director scheduling to turn accepted story material into visible anime shot guidance.",
      "Keep task, duration, source boundary, and scene purpose aligned before storyboard generation.",
    ],
    negatives: [
      "Do not let director hints overwrite story facts.",
      "Do not introduce new people, places, or world rules.",
      "Do not expose internal KB evidence in storyboard rows.",
    ],
    targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
  },
  "rw-shot-intent-taxonomy": {
    wikiType: "shot_language_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-shot-intent-taxonomy",
    ruleId: "rule:shot-intent-taxonomy",
    title: "Shot intent taxonomy",
    purpose: "Define shot-level intent so storyboard rows carry visible anime production work instead of generic descriptions.",
    axes: ["shot_intent", "visual_function", "anime_storyboard"],
    directives: [
      "Select shot intent from scene purpose and director scheduling before choosing camera or shot size.",
      "Map intents such as establish, reveal, pursue, clash, reaction, transition, payoff, spectacle, and detail insert to visible row behavior.",
      "Use reaction and detail shots only when they clarify story pressure, emotion, continuity, or payoff.",
    ],
    negatives: [
      "Do not use intent labels as user-visible prompt text by themselves.",
      "Do not create rows with no visible action or visual function.",
    ],
    targets: ["director_rule_packs", "selected_kb_rules", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
  },
  "rw-duration-density-rules": {
    wikiType: "shot_language_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-duration-density",
    ruleId: "rule:duration-density",
    title: "Duration density rules",
    purpose: "Connect target durations to shot count, beat density, row duration, and validation rules.",
    axes: ["duration_density", "beat_integrity", "anime_pacing"],
    directives: [
      "Preserve semantic beats while matching target duration exactly.",
      "Use duration to control reaction-shot density, action phases, impact pauses, and row information load.",
      "Keep every row duration positive and aligned with the selected duration profile.",
    ],
    negatives: [
      "Do not accept duration sum mismatch.",
      "Do not treat empty or zero-duration rows as success.",
      "Do not overload a short row with multiple unrelated story objectives.",
    ],
    targets: ["duration_profiles", "director_rule_packs", "validation_rule_packs", "selected_kb_rules", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-duration-integrity",
        title: "Duration integrity",
        purpose: "Validate row duration sums, zero-duration rows, and repeated full-budget rows.",
        axes: ["duration_integrity", "export_safety"],
        directives: [
          "Reject zero-duration rows and duration sum mismatches.",
          "Flag repeated full-budget rows as invalid allocation.",
        ],
        negatives: ["Do not export invalid duration allocation as success."],
      },
    ],
  },
  "rw-ensemble-action-layering": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-ensemble-action-layering",
    ruleId: "rule:ensemble-action-layering",
    title: "Ensemble action layering",
    purpose: "Layer anime group shots into primary action, secondary reaction, background motion, and spatial depth without inventing entities.",
    axes: ["ensemble_blocking", "group_motion", "field_boundary", "anime_storyboard"],
    directives: [
      "Use only accepted characters, explicit role labels, or authorized group labels when describing ensemble action.",
      "Separate primary action, secondary reaction, background group motion, and spatial layer when a group shot needs readability.",
      "Bind group motion to shot size, camera purpose, and scene objective.",
    ],
    negatives: [
      "Do not invent headcount, faction, named characters, teams, or unsupported crowd facts.",
      "Do not place new entities into the person field.",
      "Do not use group action layering as a generic spectacle filler.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    contextSummary: "Ensemble anime shots may layer action only within accepted characters, role labels, or authorized groups; unsupported counts, factions, and new characters are blocked.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-prompt-text-boundary": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-prompt-text-boundary",
    ruleId: "rule:prompt-text-boundary",
    title: "Prompt text boundary",
    purpose: "Ensure prompt_text is a clean AI anime generation-facing field compiled from accepted facts and confirmed storyboard rows.",
    axes: ["prompt_boundary", "field_boundary", "summary_only", "anime_prompt"],
    directives: [
      "Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints.",
      "Keep prompt_text aligned with accepted story facts and confirmed row fields.",
      "Use style_profile as a compact field-bound modifier after the row's subject, action, camera, shot size, and visible frame are settled.",
      "Use compact field-bound anime prompt wording rather than long universal templates.",
    ],
    negatives: [
      "Do not expose trace refs, schema IDs, audit-only provenance, raw KB, local paths, or internal hashes.",
      "Do not emit empty prompt_text.",
      "Do not paste style alias catalogs or blocked style labels into prompt_text.",
      "Do not mix note, status, governance evidence, or repair traces into final fields.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "selected_kb_rules", "negative_constraints"],
    actions: ["import_source", "generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-prompt-text-boundary",
        title: "Prompt text director boundary",
        purpose: "Keep camera, shot size, visible frame, and action cues aligned before prompt compilation.",
        axes: ["camera_language", "prompt_boundary"],
        directives: [
          "Compile camera, shot size, frame, and action cues after the row fields are coherent.",
          "Keep visual prompt details tied to the current shot.",
        ],
        negatives: ["Do not use director jargon as disconnected prompt decoration."],
      },
    ],
  },
  "rw-anime-key-pose-silhouette-priority": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-anime-key-pose-silhouette-priority",
    ruleId: "rule:anime-key-pose-silhouette-priority",
    title: "Anime key pose silhouette priority",
    purpose: "Prioritize visible anime key pose, subject silhouette, action line, and readable frame language over production-process wording.",
    axes: ["anime_key_pose", "subject_silhouette", "action_line", "field_boundary"],
    directives: [
      "Describe anime action through key pose, subject silhouette, center of gravity, action line, and readable frame priority.",
      "Keep the subject silhouette and main action readable before adding light, background, material, or style detail.",
      "Translate production handoff ideas into visible storyboard and prompt fields instead of exposing production-process terms.",
    ],
    negatives: [
      "Do not put production process, workflow labels, internal status, or note text into final storyboard fields.",
      "Do not let background, lighting, style words, or ensemble noise obscure the main subject silhouette.",
      "Do not invent characters, props, or animation steps to justify a pose.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-anime-key-pose-field-boundary",
        title: "Anime key pose field boundary",
        purpose: "Block production-process wording and internal state from final PWA fields while preserving visible key pose constraints.",
        axes: ["field_boundary", "export_safety", "anime_key_pose"],
        directives: [
          "Keep final fields focused on visible pose, silhouette, action line, and frame readability.",
          "Reject production-process wording when it appears as user-visible storyboard text.",
        ],
        negatives: ["Do not expose internal production steps or workflow state as final prompt_text."],
      },
    ],
    contextSummary: "Anime key pose and subject silhouette must stay readable before style, lighting, background, or production-process wording; final fields describe visible pose and action line only.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-anime-translation-failure-guard": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-anime-translation-failure-guard",
    ruleId: "rule:anime-translation-failure-guard",
    title: "Anime translation failure guard",
    purpose: "Prevent live-action film feel, proper-name style switches, IP references, and generic cinematic wording from bypassing anime storyboard field translation.",
    axes: ["anime_translation", "proper_name_safety", "prompt_boundary", "export_safety"],
    directives: [
      "Translate film-language input into anime subject, camera, shot size, visible frame, character action, and field-bound prompt constraints before runtime use.",
      "Block live-action film feel, real director or studio references, IP references, and generic cinematic quality words as runtime style switches.",
      "When a candidate cannot map to fixed PWA final fields, keep it in FutureQA or reject it before adapter downlink.",
    ],
    negatives: [
      "Do not use real directors, studios, films, games, IP names, or proper-name style references as runtime style switches.",
      "Do not let generic film terms replace anime storyboard language.",
      "Do not move FutureQA items, long universal templates, automatic dialogue routines, or generic anime lexicons into runtime.",
    ],
    targets: ["validation_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    contextSummary: "Film language must be translated into anime storyboard and field-bound prompt constraints before runtime; live-action feel, proper-name style switches, IP references, and generic cinematic wording are blocked.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-validation-no-leakage": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-no-leakage",
    ruleId: "rule:no-leakage",
    title: "Validation no leakage",
    purpose: "Prevent raw KB, source registers, prompt bodies, internal refs, local paths, provider config, keys, tokens, and secrets from leaking into PWA prompts, UI, traces, or exports.",
    axes: ["summary_only", "export_safety", "prompt_boundary"],
    directives: [
      "Keep runtime snapshots and PWA output summary-only.",
      "Reject raw source material, source registers, prompt bodies, raw graphs, local paths, provider config, credentials, and internal evidence in user-visible fields.",
      "Keep FutureQA and rejected material out of runtime and adapter downlink.",
    ],
    negatives: [
      "Do not expose raw KB rows, prompt bodies, source registers, local paths, or secrets.",
      "Do not expose internal evidence in exported files.",
      "Do not move FutureQA items into runtime.",
    ],
    targets: ["validation_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["import_source", "generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    contextSummary: "Runtime and adapter output stay summary-only; raw source, graph, prompt body, local path, internal evidence, and secrets are blocked.",
  },
  "rw-storyboard-splitting-by-content-beat": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-storyboard-splitting-by-content-beat",
    ruleId: "rule:storyboard-splitting-by-content-beat",
    title: "Storyboard splitting by content beat",
    purpose: "Split storyboard rows by semantic content beat, shot purpose, readable action phase, information change, or spatial scheduling change instead of fixed row count or fixed seconds.",
    axes: ["content_beat", "shot_split_boundary", "duration_density", "prompt_boundary"],
    directives: [
      "Treat rows_seed as semantic storyboard candidates, not as a fixed row-count table or duration averaging plan.",
      "Split only when the content beat, shot purpose, visible action phase, emotional or information change, or spatial scheduling changes enough to need a separate readable row.",
      "Consider dialogue or narration beats, prop handoff, attention change, and duration density only when they change what the current row must show.",
      "Allow a complete shot to remain one row when content, action, rhythm, and prompt_text readability are coherent.",
    ],
    negatives: [
      "Do not split by fixed row count, fixed seconds, table slots, or rows_seed averaging.",
      "Do not hard-split a complete shot just to fill a table or satisfy a target row count.",
      "Do not create rows with meaningless prompt_text or no visible content change.",
    ],
    targets: ["validation_rule_packs", "director_rule_packs", "scene_mappings", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-storyboard-splitting-by-content-beat",
        title: "Storyboard splitting director support",
        purpose: "Keep storyboard row splitting aligned with shot purpose, action phase, reaction need, and spatial continuity.",
        axes: ["shot_intent", "action_phase", "continuity", "anime_storyboard"],
        directives: [
          "Use shot purpose, action phase, reaction need, or spatial change to justify a new row.",
          "Keep row breaks readable for anime storyboard and AI prompt compilation.",
        ],
        negatives: ["Do not use table shape as a director reason for row splitting."],
      },
    ],
    contextSummary: "Storyboard rows split by semantic beat, readable shot purpose, dialogue or narration beat, spatial continuity, and duration density; fixed row count, fixed seconds, table-driven splitting, rows_seed averaging, and meaningless prompt_text rows are blocked.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-single-shot-preservation-for-existing-fields": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-single-shot-preservation-for-existing-fields",
    ruleId: "rule:single-shot-preservation-for-existing-fields",
    title: "Single shot preservation for existing fields",
    purpose: "Allow one complete, coherent shot to remain one storyboard row using existing final fields when splitting would reduce readability or invent structure.",
    axes: ["single_shot_preservation", "field_boundary", "shot_split_boundary", "anime_storyboard"],
    directives: [
      "Preserve a complete shot as one row when the camera, shot size, visual description, action, dialogue or narration, prompt_text, and duration remain readable together.",
      "Use existing final fields only; a preservation decision must not introduce new PWA columns or internal note fields.",
      "Split a preserved shot only when a real content beat, shot purpose, action phase, reaction, or spatial relation requires a separate row.",
    ],
    negatives: [
      "Do not force a coherent shot into multiple rows because of table shape, fixed duration slicing, or rows_seed count.",
      "Do not duplicate the same action across rows to manufacture row count.",
      "Do not hide split rationale in note, status, or internal fields as final output.",
    ],
    targets: ["validation_rule_packs", "director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-single-shot-preservation-for-existing-fields",
        title: "Single shot preservation director support",
        purpose: "Keep coherent camera movement, blocking, and action continuity inside one row when that is the most readable storyboard expression.",
        axes: ["continuity", "blocking", "shot_intent", "prompt_boundary"],
        directives: [
          "Preserve camera and action continuity when the shot is naturally complete.",
          "Use a new row only when the audience-facing information or action readability changes.",
        ],
        negatives: ["Do not break continuity to satisfy a mechanical row plan."],
      },
    ],
    contextSummary: "A complete coherent shot may remain one final-field row; row splitting is allowed only for meaningful content, action, reaction, spatial, or shot-purpose changes.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-prompt-load-and-shot-plan-boundary": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-prompt-load-and-shot-plan-boundary",
    ruleId: "rule:prompt-load-and-shot-plan-boundary",
    title: "Prompt load and shot plan boundary",
    purpose: "Keep prompt_text compact, row-bound, and single-shot focused while blocking long prompt overload, multi-shot mixing, and empty prompt rows.",
    axes: ["prompt_load", "shot_plan_boundary", "field_boundary", "export_safety"],
    directives: [
      "Compile prompt_text from the current row fields and one main shot objective.",
      "Use shot plan only as internal row planning support; final prompt_text must stay readable, compact, and bound to the current row.",
      "Reduce prompt load by choosing the current subject, action, camera, shot size, visible frame, and safe constraints instead of stacking unrelated instructions.",
    ],
    negatives: [
      "Do not pack multiple shot objectives, unrelated beats, or full scene plans into one prompt_text row.",
      "Do not create empty or meaningless prompt_text rows.",
      "Do not use long universal prompt templates, raw source text, internal notes, or workflow labels as final prompt_text.",
    ],
    targets: ["validation_rule_packs", "director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-prompt-load-and-shot-plan-boundary",
        title: "Prompt load director support",
        purpose: "Keep current-row shot purpose, camera, action, and visible frame aligned before prompt_text compilation.",
        axes: ["shot_intent", "camera_language", "prompt_boundary", "anime_storyboard"],
        directives: [
          "Choose the current row's shot purpose before adding camera, action, and frame details.",
          "Keep shot planning subordinate to final-field readability.",
        ],
        negatives: ["Do not convert shot planning notes into final prompt_text."],
      },
    ],
    contextSummary: "Prompt_text must stay compact, single-shot focused, and row-bound; long prompt overload, multi-shot objectives, empty rows, and workflow labels are blocked.",
    reviewedAtBucket: "2026-05-26",
  },
  "rw-intent-to-story-task-routing": {
    wikiType: "writing_continuity_rule",
    packCollection: "writing_rule_packs",
    ruleGroup: "writing_group",
    packId: "wg-intent-to-story-task-routing",
    ruleId: "rule:intent-to-story-task-routing",
    title: "Intent to story task routing",
    purpose: "Route short user input into story expansion, script rewrite, or storyboard creation before filling existing final storyboard fields.",
    axes: ["task_routing", "scene_goal", "fact_integrity", "anime_storyboard"],
    directives: [
      "Classify short input as story expansion, script rewrite, or storyboard creation before planning rows.",
      "Bind each row to an explicit change, character intent, scene purpose, and field-level output need.",
      "Use routing to decide whether the next step needs visible action, dialogue or narration, camera planning, duration allocation, or repair.",
    ],
    negatives: [
      "Do not answer short input as a generic question when a product task is required.",
      "Do not apply fixed three-act, fifteen-beat, or row-count templates without field evidence.",
      "Do not invent character relationships, dialogue, or backstory to make the route look complete.",
    ],
    targets: ["writing_rule_packs", "director_rule_packs", "validation_rule_packs", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-intent-to-story-task-routing",
        title: "Intent routing director support",
        purpose: "Carry task route, scene purpose, and row objective into camera, action, and duration planning.",
        axes: ["task_routing", "shot_intent", "scene_handoff"],
        directives: [
          "Choose camera and action planning from the confirmed task route.",
          "Keep scene handoff tied to the current row objective.",
        ],
        negatives: ["Do not let routing labels appear as final storyboard text."],
      },
      {
        packCollection: "validation_rule_packs",
        packId: "vg-intent-to-story-task-routing",
        title: "Intent routing validation",
        purpose: "Block route drift that turns product tasks into generic writing advice or unsupported invention.",
        axes: ["task_routing", "field_boundary", "fact_integrity"],
        directives: [
          "Reject output that does not match the selected product task.",
          "Require route evidence before accepting invented dialogue or field content.",
        ],
        negatives: ["Do not expose route labels, internal status, or evidence notes in final fields."],
      },
    ],
    contextSummary: "Short input is first routed to story expansion, script rewrite, or storyboard creation; row fields then serve the confirmed route, scene purpose, and accepted facts.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-character-visual-continuity-anchor": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-character-visual-continuity-anchor",
    ruleId: "rule:character-visual-continuity-anchor",
    title: "Character visual continuity anchor",
    purpose: "Preserve character identity, relationship, appearance, posture, and expression anchors across storyboard rows and prompt_text.",
    axes: ["character_continuity", "visual_grounding", "field_boundary", "prompt_boundary"],
    directives: [
      "Carry confirmed identity, relationship, appearance, posture, and expression anchors into every row that uses the character.",
      "Let prompt_text inherit confirmed character anchors without adding new names, IP references, or source handles.",
      "Repair drift by restoring the accepted character anchor before changing camera, action, or style wording.",
    ],
    negatives: [
      "Do not invent character names, relationships, costumes, or visual signatures.",
      "Do not use file names, source paths, real IP, studio names, or source metadata as continuity switches.",
      "Do not replace accepted person values with generic role labels when a confirmed character anchor exists.",
    ],
    targets: ["validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    contextSummary: "Character identity and visual continuity are anchored to confirmed facts; prompt_text may inherit anchors but must not invent names, relationships, IP references, or source metadata.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-scene-spatial-prop-continuity-anchor": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-scene-spatial-prop-continuity-anchor",
    ruleId: "rule:scene-spatial-prop-continuity-anchor",
    title: "Scene spatial prop continuity anchor",
    purpose: "Keep location, left-right relation, eyeline, prop anchors, background anchors, and event order continuous across adjacent storyboard rows.",
    axes: ["spatial_continuity", "prop_continuity", "event_order", "camera_language"],
    directives: [
      "Preserve character position, eyeline direction, prop state, background anchor, and event order when splitting rows.",
      "Use camera movement and shot size to clarify spatial relation instead of resetting location or screen direction.",
      "Treat a prop handoff, position change, or event-order change as a row-level continuity requirement.",
    ],
    negatives: [
      "Do not jump screen direction, swap left-right placement, lose key props, or change location without accepted cause.",
      "Do not hide spatial continuity repair in final prompt_text as an internal note.",
      "Do not create new props or locations to solve a continuity gap.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-scene-spatial-prop-continuity-anchor",
        title: "Scene spatial prop continuity validation",
        purpose: "Reject row transitions that lose accepted props, locations, eyelines, or event order.",
        axes: ["spatial_continuity", "prop_continuity", "export_safety"],
        directives: [
          "Validate that adjacent rows preserve accepted space, prop state, and event order.",
          "Repair only the field that caused the continuity break.",
        ],
        negatives: ["Do not introduce unsupported props, locations, or continuity explanations."],
      },
    ],
    contextSummary: "Scene continuity preserves location, screen direction, eyeline, prop state, background anchors, and event order across adjacent rows.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-layout-staging-attention-path": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-layout-staging-attention-path",
    ruleId: "rule:layout-staging-attention-path",
    title: "Layout staging attention path",
    purpose: "Arrange subject, action, background, light, composition, and depth so the viewer reads the current row's most important information first.",
    axes: ["composition", "staging", "visual_attention", "anime_storyboard"],
    directives: [
      "Set a primary visual focus before adding secondary action, background detail, light, or style wording.",
      "Use staging, depth, silhouette, contrast, and motion direction to guide attention through the row.",
      "Keep composition subordinate to the current character action, scene objective, and shot size.",
    ],
    negatives: [
      "Do not let multiple subjects, background details, style terms, and action words compete for the same focus.",
      "Do not use composition language as decorative filler disconnected from the row objective.",
      "Do not bury the main action behind atmosphere or background description.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "selected_kb_rules", "kb_context_summary"],
    actions: ["generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-layout-staging-attention-path",
        title: "Layout staging attention validation",
        purpose: "Reject rows where visual focus is ambiguous or the main action is obscured by competing detail.",
        axes: ["visual_attention", "field_boundary", "prompt_boundary"],
        directives: [
          "Validate that prompt_text keeps one primary focus for the row.",
          "Repair visual_description or character_action before adding more style terms.",
        ],
        negatives: ["Do not add attention-path notes as user-visible final text."],
      },
    ],
    contextSummary: "Layout and staging must create a clear attention path: one primary focus, readable action, and supporting background or light only when it serves the current row.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-color-script-emotional-continuity": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-color-script-emotional-continuity",
    ruleId: "rule:color-script-emotional-continuity",
    title: "Color script emotional continuity",
    purpose: "Use visible light source, color temperature, material response, air medium, and emotional color progression to support the current beat and adjacent rows.",
    axes: ["color_script", "lighting_continuity", "material_response", "emotional_progression"],
    directives: [
      "Bind light direction, color temperature, material response, and air medium to the current beat and scene logic.",
      "Use color progression to clarify emotional or information change across adjacent rows.",
      "Route emotional tone through visible light, color temperature, material response, air medium, and rhythm only when accepted body and scene type support it.",
      "Keep visual_description and prompt_text consistent with the established visual world.",
    ],
    negatives: [
      "Do not stack generic anime color words without a visible source or story function.",
      "Do not turn emotional tone into an unrelated style switch.",
      "Do not use real studio, IP, film, or director style names as runtime style switches.",
      "Do not add unsupported weather, light, or atmosphere to manufacture mood.",
    ],
    targets: ["director_rule_packs", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    contextSummary: "Color, light, material, air, rhythm, and emotional tone cues must serve the current beat and adjacent-row continuity without generic style switches or unsupported atmosphere.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-dialogue-narration-allocation-rules": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-dialogue-narration-allocation-rules",
    ruleId: "rule:dialogue-narration-allocation-rules",
    title: "Dialogue narration allocation rules",
    purpose: "Allocate dialogue, narration, action explanation, and duration to the current storyboard row only when supported by text, fact, character intent, or scene purpose.",
    axes: ["dialogue_grounding", "narration_allocation", "duration_density", "field_boundary"],
    directives: [
      "Bind each dialogue or narration line to the current row's person, action, scene purpose, and duration.",
      "Use visible action or silence when dialogue evidence is weak.",
      "Keep narration short and row-bound when it clarifies information that cannot be shown visibly.",
    ],
    negatives: [
      "Do not turn this rule into an automatic dialogue-generation template.",
      "Do not use narration to replace visible action that the row can show.",
      "Do not move dialogue across rows or invent exposition to fill duration.",
    ],
    targets: ["writing_rule_packs", "validation_rule_packs", "duration_profiles", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "writing_rule_packs",
        packId: "wg-dialogue-narration-allocation-rules",
        title: "Dialogue narration writing support",
        purpose: "Keep dialogue and narration grounded in accepted text, character intent, and current-row function.",
        axes: ["dialogue_grounding", "scene_goal", "duration_density"],
        directives: [
          "Choose dialogue, narration, visible action, or silence by row function.",
          "Keep line length and information load compatible with row duration.",
        ],
        negatives: ["Do not invent motives, relationships, or exposition to justify a line."],
      },
    ],
    contextSummary: "Dialogue and narration are allocated to the current row only when grounded by accepted text, facts, character intent, scene purpose, and duration capacity.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-field-level-failure-repair-routing": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-field-level-failure-repair-routing",
    ruleId: "rule:field-level-failure-repair-routing",
    title: "Field level failure repair routing",
    purpose: "Route storyboard failures to the specific affected field while preserving row facts, shot purpose, duration, and summary-only boundaries.",
    axes: ["field_boundary", "repair_routing", "validation", "export_safety"],
    directives: [
      "Locate failure by final field before repairing person, camera, shot_size, visual_description, character_action, dialogue_or_narration, prompt_text, or duration_seconds.",
      "Repair only the affected field unless adjacent fields must change to preserve row coherence.",
      "Keep repair output aligned with accepted facts, shot purpose, duration, and summary-only runtime constraints.",
    ],
    negatives: [
      "Do not rewrite every field with a long universal prompt template.",
      "Do not use style words to overwrite facts or field responsibilities.",
      "Do not mix status, notes, evidence, source refs, or internal repair traces into final fields.",
    ],
    targets: ["validation_rule_packs", "negative_constraints", "duration_profiles", "kb_context_summary"],
    actions: ["repair_storyboard", "validate_result", "export_result"],
    contextSummary: "Field-level repair fixes only the affected final field, preserves row facts and shot purpose, and blocks notes, evidence, source refs, or internal traces from final output.",
    reviewedAtBucket: "2026-05-27",
  },
  "rw-accepted-body-to-style-profile": {
    wikiType: "writing_continuity_rule",
    packCollection: "writing_rule_packs",
    ruleGroup: "writing_group",
    packId: "wg-accepted-body-to-style-profile",
    ruleId: "rule:accepted-body-to-style-profile",
    title: "Accepted body to style profile",
    purpose: "Derive a compact style_profile from accepted body facts, scene type, tone, time period, world logic, and visible production need before any storyboard field receives style guidance.",
    axes: ["style_profile", "fact_integrity", "scene_goal", "prompt_boundary"],
    directives: [
      "Derive style_profile only from accepted body facts, scene type, tone, time period, world logic, and visible production need.",
      "Keep style_profile as a compact routing summary that guides downstream fields without adding unaccepted plot, person, IP, studio, creator, or audit facts.",
      "When accepted body does not support a style direction, keep style_profile neutral and let scene action, camera, continuity, and duration rules carry the row.",
    ],
    negatives: [
      "Do not paste style alias catalogs or audit-only provenance into runtime fields.",
      "Do not let style_profile override accepted facts, character identity, scene type, duration, or row objective.",
      "Do not use real IP, real creators, studios, or blocked style categories as runtime style switches.",
    ],
    targets: ["writing_rule_packs", "director_rule_packs", "validation_rule_packs", "scene_mappings", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "director_rule_packs",
        packId: "dg-accepted-body-to-style-profile",
        title: "Accepted body style profile director support",
        purpose: "Carry the compact style_profile into camera, shot size, visible frame, and prompt planning only after accepted facts are locked.",
        axes: ["style_profile", "shot_intent", "visual_grounding"],
        directives: [
          "Use the style_profile to choose visible production cues, not to create new story facts.",
          "Keep style guidance subordinate to row purpose, shot size, action readability, and continuity.",
        ],
        negatives: ["Do not convert style_profile into a full prompt template or alias list."],
      },
      {
        packCollection: "validation_rule_packs",
        packId: "vg-accepted-body-to-style-profile",
        title: "Accepted body style profile validation",
        purpose: "Reject style_profile output that is unsupported by accepted body facts or leaks audit-only material.",
        axes: ["fact_integrity", "field_boundary", "export_safety"],
        directives: [
          "Validate that style_profile can be traced to accepted body facts, scene type, or explicit tone.",
          "Reject unsupported style labels before adapter downlink.",
        ],
        negatives: ["Do not expose audit-only material in final PWA fields."],
      },
    ],
    contextSummary: "style_profile is derived from accepted body, scene type, tone, time period, world logic, and visible production need; it stays compact, fact-bound, and subordinate to final storyboard fields.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-scene-driven-style-routing": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-scene-driven-style-routing",
    ruleId: "rule:scene-driven-style-routing",
    title: "Scene driven style routing",
    purpose: "Use scene type as the first style prior, then refine with accepted body content and style_profile before projecting style into storyboard fields.",
    axes: ["scene_style_routing", "scene_type", "style_profile", "anime_storyboard"],
    directives: [
      "Treat scene type as the first style prior, then narrow style guidance with accepted body content, scene goal, emotional tone, and visual continuity.",
      "Select only a small style family or production cue set that serves the current scene function.",
      "Keep scene-driven style routing in summary form so runtime receives field guidance, not an alias table.",
    ],
    negatives: [
      "Do not route style from a free-floating alias list when scene type and accepted body conflict with it.",
      "Do not stack multiple style families to make a row look richer.",
      "Do not use style routing to bypass person, action, continuity, camera, shot size, or duration constraints.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-scene-driven-style-routing",
        title: "Scene driven style routing validation",
        purpose: "Reject style routing that contradicts scene type, accepted body, or final-field boundaries.",
        axes: ["scene_type", "style_profile", "field_boundary"],
        directives: [
          "Validate that style routing supports the scene type and accepted facts.",
          "Repair by reducing style guidance before changing story facts or final-field structure.",
        ],
        negatives: ["Do not accept unrelated style switches as valid scene routing."],
      },
    ],
    contextSummary: "Scene type is the first style prior; accepted body and style_profile refine it into a small field-bound style route instead of a free alias list.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-style-to-field-projection": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-style-to-field-projection",
    ruleId: "rule:style-to-field-projection",
    title: "Style to field projection",
    purpose: "Project a confirmed style_profile into existing PWA fields through visible frame, action, camera, shot size, duration, prompt text, and safe negative constraints.",
    axes: ["style_projection", "field_boundary", "visual_grounding", "prompt_boundary"],
    directives: [
      "Project style_profile into visual_description through palette, material, lighting, texture, composition, and space only when visible in the row.",
      "Project style_profile into character_action, camera, shot_size, and duration only when it improves action readability, rhythm, or attention path.",
      "Compile prompt_text from the already-set row fields and one compact style modifier, then place unsafe or unsupported style material into negative_constraints.",
    ],
    negatives: [
      "Do not create new PWA fields for style output.",
      "Do not place style terms in person or use style to replace accepted action.",
      "Do not let prompt_text carry a second scene plan, alias list, or unsupported style stack.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-style-to-field-projection",
        title: "Style to field projection validation",
        purpose: "Validate that style_profile affects only existing final fields and remains summary-only.",
        axes: ["field_boundary", "prompt_boundary", "export_safety"],
        directives: [
          "Reject style output that introduces a new field, rewrites person, or replaces accepted action.",
          "Require unsupported style material to remain out of runtime output.",
        ],
        negatives: ["Do not expose audit-only style catalog entries in final fields."],
      },
    ],
    contextSummary: "style_profile projects only into existing final fields through visible cues, camera, shot size, action readability, duration rhythm, prompt_text, and safe negative constraints.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-chinese-heritage-style-scene-routing": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-chinese-heritage-style-scene-routing",
    ruleId: "rule:chinese-heritage-style-scene-routing",
    title: "Chinese heritage style scene routing",
    purpose: "Allow Chinese heritage visual cues only when the accepted scene, era, object logic, environment, or user intent supports them.",
    axes: ["heritage_style", "scene_style_routing", "visual_grounding", "cultural_safety"],
    directives: [
      "Use Chinese heritage visual cues only when accepted scene facts, era, location, object logic, or user intent support them.",
      "Bind heritage cues to visible material, line rhythm, spatial composition, prop logic, costume logic, environment, or light rather than treating them as a global style switch.",
      "Keep cultural cues subordinate to scene objective, character action, continuity, camera, shot size, and prompt boundary.",
    ],
    negatives: [
      "Do not add heritage motifs as generic decoration or stereotype.",
      "Do not use cultural or religious labels without accepted scene support.",
      "Do not replace accepted setting, props, costume, or character facts with a style family.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-chinese-heritage-style-scene-routing",
        title: "Chinese heritage style routing validation",
        purpose: "Reject unsupported or stereotyped heritage cues before final fields or adapter output.",
        axes: ["cultural_safety", "fact_integrity", "field_boundary"],
        directives: [
          "Require accepted scene support before applying heritage visual cues.",
          "Repair unsupported heritage cues by returning to neutral scene-grounded style_profile guidance.",
        ],
        negatives: ["Do not turn heritage routing into a universal style preset."],
      },
    ],
    contextSummary: "Chinese heritage cues require accepted scene support and must appear as visible material, line, composition, prop, costume, environment, or light cues serving the current row.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-retrofuture-punk-style-scene-routing": {
    wikiType: "director_scheduling_rule",
    packCollection: "director_rule_packs",
    ruleGroup: "director_group",
    packId: "dg-retrofuture-punk-style-scene-routing",
    ruleId: "rule:retrofuture-punk-style-scene-routing",
    title: "Retrofuture punk style scene routing",
    purpose: "Route retrofuture or punk-family cues only when accepted world logic, technology level, material culture, conflict, or environment supports them.",
    axes: ["retrofuture_style", "scene_style_routing", "world_logic", "visual_grounding"],
    directives: [
      "Use retrofuture or punk-family cues only when accepted world logic, technology level, material culture, conflict, or environment supports them.",
      "Translate the style family into visible machinery, material wear, signage logic, lighting, silhouette, motion pressure, or camera rhythm.",
      "Keep one coherent style family per row unless accepted body explicitly requires a contrast.",
    ],
    negatives: [
      "Do not pile up subgenre labels as prompt decoration.",
      "Do not add unsupported machines, cities, costumes, factions, or technology to justify the style.",
      "Do not let retrofuture or punk-family routing override scene type, accepted facts, action readability, or duration.",
    ],
    targets: ["director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"],
    additionalPacks: [
      {
        packCollection: "validation_rule_packs",
        packId: "vg-retrofuture-punk-style-scene-routing",
        title: "Retrofuture punk style routing validation",
        purpose: "Reject subgenre label stacking and unsupported technology or world changes.",
        axes: ["world_logic", "field_boundary", "prompt_boundary"],
        directives: [
          "Validate that retrofuture or punk-family cues have accepted world support.",
          "Repair by reducing the style cue set before changing world facts.",
        ],
        negatives: ["Do not accept subgenre label stacks as visual grounding."],
      },
    ],
    contextSummary: "Retrofuture and punk-family cues require accepted world logic and must project as visible machinery, materials, signage, light, silhouette, motion pressure, or camera rhythm rather than label stacks.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-style-contamination-guard": {
    wikiType: "validation_rule",
    packCollection: "validation_rule_packs",
    ruleGroup: "validation_group",
    packId: "vg-style-contamination-guard",
    ruleId: "rule:style-contamination-guard",
    title: "Style contamination guard",
    purpose: "Block unsupported style switches, raw alias catalog material, blocked categories, and proper-name style references from runtime fields, prompt text, adapter output, and exports.",
    axes: ["style_safety", "prompt_boundary", "export_safety", "summary_only"],
    directives: [
      "Keep style alias normalization and blocked-category review in the audit layer; runtime receives only compact, accepted, field-bound style guidance.",
      "Reject proper-name style references, real IP, real creators, real studios, incomplete labels, unsupported cultural or religious labels, generalized category labels, and unconstrained media-format labels.",
      "When style evidence is weak, repair by dropping the style cue or returning to neutral scene-driven guidance.",
    ],
    negatives: [
      "Do not expose raw alias lists, audit-only provenance, or rejected labels in runtime output.",
      "Do not use proper names, protected works, studios, creators, or unsupported cultural labels as style switches.",
      "Do not let a blocked style item survive inside prompt_text, selected_kb_rules, kb_context_summary, or exports.",
    ],
    targets: ["validation_rule_packs", "scene_mappings", "selected_kb_rules", "negative_constraints", "kb_context_summary"],
    actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result", "export_result"],
    contextSummary: "Style alias review stays audit-only; runtime uses compact accepted style guidance and blocks proper-name references, protected works, studios, creators, unsupported cultural labels, generalized categories, and unconstrained media-format labels.",
    reviewedAtBucket: "2026-05-28",
  },
  "rw-scene-type-as-style-prior": {
    mergeTargets: ["rw-scene-driven-style-routing"],
  },
  "rw-style-alias-family-catalog": {
    mergeTargets: ["rw-style-contamination-guard"],
  },
  "rw-emotional-tone-style-routing": {
    mergeTargets: ["rw-color-script-emotional-continuity"],
  },
  "rw-style-profile-prompt-boundary": {
    mergeTargets: ["rw-prompt-text-boundary", "rw-seedance2-storyboard-field-discipline"],
  },
  "rw-shot-size-camera-purpose-progression": {
    mergeTargets: ["rw-camera-language-grammar", "rw-shot-intent-taxonomy", "rw-director-scheduling-core"],
  },
  "rw-animation-acting-readability-for-fields": {
    mergeTargets: ["rw-expression-physicalization", "rw-anime-key-pose-silhouette-priority", "rw-transition-motion-dynamics"],
  },
  "rw-storyboard-row-field-fusion-for-prompt-text": {
    mergeTargets: ["rw-prompt-text-boundary", "rw-seedance2-storyboard-field-discipline"],
  },
  "rw-duration-allocation-by-field-readability": {
    mergeTargets: ["rw-duration-density-rules"],
  },
  "rw-action-key-pose-staging-for-fields": {
    mergeTargets: ["rw-expression-physicalization", "rw-anime-key-pose-silhouette-priority"],
  },
  "rw-continuity-eyeline-blocking-for-fields": {
    mergeTargets: ["rw-camera-language-grammar", "rw-director-scheduling-core", "rw-transition-motion-dynamics"],
  },
  "rw-dialogue-narration-beat-field-gate": {
    mergeTargets: ["rw-dialogue-evidence-lock", "rw-prompt-text-boundary"],
  },
  "rw-reaction-shot-field-necessity-gate": {
    mergeTargets: ["rw-shot-intent-taxonomy", "rw-duration-density-rules"],
  },
  "rw-camera-shot-size-purpose-for-prompt-text": {
    mergeTargets: ["rw-camera-language-grammar", "rw-shot-intent-taxonomy", "rw-prompt-text-boundary"],
  },
};

function parseArgs(argv) {
  const options = { packagePath: "" };
  for (let index = 2; index < argv.length; index += 1) {
    if (argv[index] === "--package") {
      options.packagePath = path.resolve(argv[++index]);
    } else {
      throw new Error(`Unknown argument: ${argv[index]}`);
    }
  }
  if (!options.packagePath) throw new Error("Missing --package <path>");
  return options;
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function unique(items) {
  return Array.from(new Set(items.filter(Boolean)));
}

function scenePackKeyForCollection(collection) {
  if (collection === "writing_rule_packs") return "writing_rule_pack_ids";
  if (collection === "validation_rule_packs") return "validation_rule_pack_ids";
  return "director_rule_pack_ids";
}

function scenePackKey(def) {
  return scenePackKeyForCollection(def.packCollection);
}

function targetFieldPath(target, def) {
  if (["writing_rule_packs", "director_rule_packs", "validation_rule_packs"].includes(target)) return `${target}[].directives`;
  if (target === "scene_mappings") return `scene_mappings.*.${scenePackKey(def)}`;
  if (target === "selected_kb_rules") return "scene_mappings.*.selected_kb_rules";
  if (target === "negative_constraints") return "scene_mappings.*.negative_constraints";
  if (target === "kb_context_summary") return "scene_mappings.*.kb_context_summary";
  return `scene_mappings.*.${target}`;
}

function packDefinitionsFor(def) {
  return [
    {
      packCollection: def.packCollection,
      packId: def.packId,
      title: def.title,
      purpose: def.purpose,
      axes: def.axes,
      directives: def.directives,
      negatives: def.negatives,
    },
  ].concat(def.additionalPacks || []);
}

function packFor(id, packDef) {
  return {
    id: packDef.packId,
    name: packDef.title,
    intent: packDef.purpose,
    influence_axes: packDef.axes,
    directives: packDef.directives,
    negative_constraints: packDef.negatives,
    source_reviewed_wiki_ids: [id],
  };
}

function mdFor(rec, def) {
  return [
    `# ${rec.reviewed_wiki_id}`,
    "",
    `wiki_type: ${def.wikiType}`,
    "review_status: reviewed",
    "effective_confidence: medium",
    "runtime_eligible: true",
    "leakage_count: 0",
    "",
    "## Purpose",
    "",
    `${rec.chinese_title || def.title}: ${def.purpose}`,
    "",
    "## Source Basis",
    "",
    "This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.",
    "",
    "## Applies To",
    "",
    def.actions.map((item) => `- ${item}`).join("\n"),
    "",
    "## Claims Summary",
    "",
    def.directives.map((item) => `- ${item}`).join("\n"),
    "",
    "## Runtime Mapping",
    "",
    `- ${def.packCollection}: ${def.packId}`,
    `- selected_kb_rules: ${def.ruleId}`,
    `- runtime_targets: ${def.targets.join(", ")}`,
    "",
    "## PWA Fields Served",
    "",
    (rec.serves_pwa_fields || []).map((item) => `- ${item}`).join("\n"),
    "",
    "## Negative Constraints",
    "",
    def.negatives.map((item) => `- ${item}`).join("\n"),
    "",
  ].join("\n");
}

function mappingFor(rec, def, sceneIds) {
  const runtimePackIds = packDefinitionsFor(def).map((packDef) => packDef.packId);
  return {
    mapping_id: `map-${rec.reviewed_wiki_id}`,
    mapping_version: "wiki-to-runtime/v0.2",
    reviewed_wiki_id: rec.reviewed_wiki_id,
    reviewed_wiki_hash: `sha256:summary-${rec.reviewed_wiki_id}`,
    wiki_type: def.wikiType,
    runtime_targets: def.targets,
    scene_type_ids: sceneIds,
    kb_actions: def.actions,
    rule_group: def.ruleGroup,
    runtime_rule_pack_ids: runtimePackIds,
    selected_kb_rule_ids: [def.ruleId],
    summary_fragment_ids: [`summary:${rec.reviewed_wiki_id.replace(/^rw-/, "")}`],
    negative_constraint_ids: ["negative:summary-only-confirmed-intake"],
    target_field_paths: def.targets.map((target) => targetFieldPath(target, def)),
    duration_targets: [],
    failure_pattern_ids: [],
    repair_mapping_ids: [],
    future_qa_ids: [],
    sanitized_fragment_hashes: [`sha256:summary-${rec.reviewed_wiki_id}`],
    fallback_behavior: "supporting_rule",
    reviewed_at_bucket: def.reviewedAtBucket || "2026-05-23",
    runtime_eligible_reason: `${rec.reviewed_wiki_id} was confirmed to serve PWA storyboard rows and prompt_text through summary-only runtime targets.`,
    blocked_reason: "",
    coverage_status: "covered",
    review_status: "reviewed",
    effective_confidence: "medium",
    runtime_eligible: true,
    leakage_count: 0,
  };
}

function addTargetEntry(targetMap, targetId, sourceRec, sourceAcceptedId) {
  const targetDef = catalog[targetId];
  if (!targetDef || targetDef.mergeTargets) {
    throw new Error(`Merge target is missing concrete apply catalog support: ${targetId}`);
  }
  const existing = targetMap.get(targetId);
  const targetRec = existing
    ? existing.rec
    : {
        ...sourceRec,
        reviewed_wiki_id: targetId,
        chinese_title: targetDef.title,
        status: "confirmed",
      };
  targetRec.serves_pwa_fields = unique((targetRec.serves_pwa_fields || []).concat(sourceRec.serves_pwa_fields || []));
  targetMap.set(targetId, {
    rec: targetRec,
    def: targetDef,
    sourceAcceptedIds: unique(((existing && existing.sourceAcceptedIds) || []).concat(sourceAcceptedId)),
  });
}

function expandAcceptedTargets(accepted) {
  const targetMap = new Map();
  const acceptedTargetStatus = [];
  accepted.forEach((rec) => {
    const def = catalog[rec.reviewed_wiki_id];
    if (!def) return;
    const targetIds = def.mergeTargets || [rec.reviewed_wiki_id];
    targetIds.forEach((targetId) => addTargetEntry(targetMap, targetId, rec, rec.reviewed_wiki_id));
    acceptedTargetStatus.push({
      accepted_id: rec.reviewed_wiki_id,
      action: def.mergeTargets ? "merge_existing" : "apply_reviewed_wiki",
      target_reviewed_wiki_ids: targetIds,
    });
  });
  return { targetEntries: Array.from(targetMap.values()), acceptedTargetStatus };
}

function runNode(args) {
  const result = spawnSync(process.execPath, args, { cwd: repoRoot, encoding: "utf8" });
  if (result.error) {
    throw new Error(`${args.join(" ")} failed to spawn: ${result.error.message}`);
  }
  if (result.status !== 0) {
    throw new Error(`${args.join(" ")} failed\n${result.stdout}\n${result.stderr}`);
  }
  return JSON.parse(result.stdout);
}

function ensureActionCoverage(snapshot, def) {
  const matrix = snapshot.coverage_matrix || {};
  const coverage = matrix.kb_action_coverage || [];
  def.actions.forEach((action) => {
    let row = coverage.find((item) => item.kb_action === action);
    if (!row) {
      row = { kb_action: action, covered: true, rule_pack_ids: [] };
      coverage.push(row);
    }
    row.covered = true;
    row.rule_pack_ids = unique((row.rule_pack_ids || []).concat(def.packId));
  });
  matrix.kb_action_coverage = coverage;
  snapshot.coverage_matrix = matrix;
}

function main() {
  const options = parseArgs(process.argv);
  const validate = runNode([path.join("scripts", "validate-manual-intake-package.js"), "--package", options.packagePath]);
  const pkg = readJson(options.packagePath);

  if (!pkg.confirmation || pkg.confirmation.confirmed !== true) throw new Error("Package is not user-confirmed.");

  const acceptedIds = unique((pkg.confirmation.accepted_reviewed_wiki_ids || []).map((id) => String(id || "").trim()));
  if (!acceptedIds.length) throw new Error("No accepted reviewed_wiki ids in confirmation.");

  const acceptedIdSet = new Set(acceptedIds);
  const accepted = (pkg.recommendations || []).filter((rec) => acceptedIdSet.has(rec.reviewed_wiki_id) && rec.status === "confirmed");
  const acceptedRecIds = new Set(accepted.map((rec) => rec.reviewed_wiki_id));
  const missing = acceptedIds.filter((id) => !acceptedRecIds.has(id));
  if (missing.length) throw new Error(`Accepted ids are missing confirmed recommendations: ${missing.join(", ")}`);
  if (!accepted.length) throw new Error("No confirmed recommendations to apply.");
  const unsupported = accepted.filter((rec) => !catalog[rec.reviewed_wiki_id]).map((rec) => rec.reviewed_wiki_id);
  if (unsupported.length) throw new Error(`Accepted ids are missing apply catalog support: ${unsupported.join(", ")}`);
  const unsupportedMergeTargets = accepted.flatMap((rec) => {
    const def = catalog[rec.reviewed_wiki_id];
    return (def.mergeTargets || []).filter((targetId) => !catalog[targetId] || catalog[targetId].mergeTargets);
  });
  if (unsupportedMergeTargets.length) {
    throw new Error(`Accepted ids reference merge targets without concrete apply catalog support: ${unique(unsupportedMergeTargets).join(", ")}`);
  }

  fs.mkdirSync(reviewedDir, { recursive: true });

  const mapping = readJson(mappingPath);
  const snapshot = readJson(snapshotPath);
  const sceneIds = (snapshot.scene_types || []).map((scene) => scene.id);
  const mappingByWiki = new Map(mapping.mappings.map((item) => [item.reviewed_wiki_id, item]));
  const packMaps = {
    writing_rule_packs: new Map((snapshot.writing_rule_packs || []).map((pack) => [pack.id, pack])),
    director_rule_packs: new Map((snapshot.director_rule_packs || []).map((pack) => [pack.id, pack])),
    validation_rule_packs: new Map((snapshot.validation_rule_packs || []).map((pack) => [pack.id, pack])),
  };

  const applied = [];
  const appliedDefs = [];
  const { targetEntries, acceptedTargetStatus } = expandAcceptedTargets(accepted);

  targetEntries.forEach(({ rec, def }) => {
    if (!def) return;

    fs.writeFileSync(path.join(reviewedDir, `${rec.reviewed_wiki_id}.md`), mdFor(rec, def), "utf8");
    mappingByWiki.set(rec.reviewed_wiki_id, mappingFor(rec, def, sceneIds));
    packDefinitionsFor(def).forEach((packDef) => {
      packMaps[packDef.packCollection].set(packDef.packId, packFor(rec.reviewed_wiki_id, packDef));
    });
    applied.push(rec.reviewed_wiki_id);
    appliedDefs.push(def);
  });

  mapping.mappings = Array.from(mappingByWiki.values());
  mapping.coverage_summary.runtime_eligible_wiki_count = mapping.mappings.filter((item) => item.runtime_eligible).length;
  mapping.coverage_summary.mapped_runtime_eligible_wiki_count = mapping.coverage_summary.runtime_eligible_wiki_count;
  mapping.coverage_summary.unmapped_runtime_eligible_wiki_count = 0;
  mapping.coverage_summary.scene_type_count = sceneIds.length;
  mapping.coverage_summary.leakage_count = 0;

  snapshot.writing_rule_packs = Array.from(packMaps.writing_rule_packs.values());
  snapshot.director_rule_packs = Array.from(packMaps.director_rule_packs.values());
  snapshot.validation_rule_packs = Array.from(packMaps.validation_rule_packs.values());

  Object.keys(snapshot.scene_mappings || {}).forEach((sceneId) => {
    const entry = snapshot.scene_mappings[sceneId];
    appliedDefs.forEach((def) => {
      packDefinitionsFor(def).forEach((packDef) => {
        const key = scenePackKeyForCollection(packDef.packCollection);
        entry[key] = unique((entry[key] || []).concat(packDef.packId));
      });
      entry.selected_kb_rules = unique((entry.selected_kb_rules || []).concat(def.ruleId));
      entry.influence_axes = unique((entry.influence_axes || []).concat(def.axes));
      entry.negative_constraints = unique((entry.negative_constraints || []).concat(def.negatives));
      if (def.contextSummary && !(entry.kb_context_summary || "").includes(def.contextSummary)) {
        entry.kb_context_summary = `${entry.kb_context_summary || ""} ${def.contextSummary}`.trim();
      }
      ensureActionCoverage(snapshot, def);
    });
  });

  snapshot.coverage_matrix.unmapped_runtime_eligible_wiki_count = 0;
  snapshot.coverage_matrix.leakage_count = 0;

  writeJson(mappingPath, mapping);
  writeJson(sampleMappingPath, mapping);
  writeJson(snapshotPath, snapshot);
  writeJson(sampleSnapshotPath, snapshot);

  const build = runNode([path.join("scripts", "build-pwa-kb-adapter-output.js")]);
  const result = {
    status: "applied",
    package: path.relative(repoRoot, options.packagePath),
    processed_accepted_ids: acceptedIds,
    accepted_target_status: acceptedTargetStatus,
    applied_reviewed_wiki_ids: applied,
    validate,
    build,
  };
  pkg.status = "applied_to_kb";
  pkg.kb_application = {
    applied: true,
    applied_at: new Date().toISOString(),
    command: `node scripts\\apply-confirmed-intake-package.js --package ${path.relative(repoRoot, options.packagePath)}`,
    validation_status: "passed",
    output: result,
  };
  writeJson(options.packagePath, pkg);
  console.log(JSON.stringify(result, null, 2));
}

main();
