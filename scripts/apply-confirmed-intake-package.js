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
      "Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints only.",
    ],
    negatives: [
      "Do not add anime-only camera vocabulary when the scene style is grounded, documentary, or otherwise not anime-compatible.",
      "Do not put camera, style, composition, source, or KB terms into person.",
      "Do not force tiny facial muscle detail into wide, aerial, group, or battlefield-scale shots.",
      "Do not use light, color, atmosphere, or composition as disconnected decoration.",
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
      "Use compact field-bound anime prompt wording rather than long universal templates.",
    ],
    negatives: [
      "Do not expose trace refs, schema IDs, source registers, raw KB, prompt bodies, local paths, or internal hashes.",
      "Do not emit empty prompt_text.",
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

  accepted.forEach((rec) => {
    const def = catalog[rec.reviewed_wiki_id];
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
  const result = { status: "applied", package: path.relative(repoRoot, options.packagePath), applied_reviewed_wiki_ids: applied, validate, build };
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
