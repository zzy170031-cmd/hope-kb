const fs = require("fs");
const path = require("path");

const repoRoot = path.resolve(__dirname, "..");
const defaultSnapshotPath = path.join(repoRoot, "knowledge", "runtime_snapshots", "latest.candidate.json");
const defaultOutputPath = path.join(repoRoot, "samples", "pwa-kb-adapter-output.sample.json");

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function toPwaRulePack(pack) {
  return {
    id: pack.id,
    name: pack.name,
    intent: pack.intent,
    influenceAxes: pack.influence_axes || [],
    directives: pack.directives || [],
    negativeConstraints: pack.negative_constraints || [],
  };
}

function toPwaDurationProfile(profile) {
  return {
    duration: profile.duration,
    targetShotCount: profile.target_shot_count,
    beatDensity: profile.beat_density,
    pacingDirective: profile.pacing_directive,
  };
}

function toPwaSceneMappings(sceneMappings) {
  return Object.fromEntries(
    Object.entries(sceneMappings).map(([sceneId, mapping]) => [
      sceneId,
      {
        writingRulePackIds: mapping.writing_rule_pack_ids || [],
        directorRulePackIds: mapping.director_rule_pack_ids || [],
        validationRulePackIds: mapping.validation_rule_pack_ids || [],
        sceneProfile: mapping.scene_profile || "",
        negativeConstraints: mapping.negative_constraints || [],
        influenceAxes: mapping.influence_axes || [],
      },
    ]),
  );
}

function buildAdapterOutput(snapshot) {
  return {
    adapter_version: "pwa-kb-adapter/v0.2",
    adapter_status: "prototype",
    source_snapshot_version: snapshot.snapshot_version,
    source_snapshot_hash: snapshot.snapshot_hash,
    scene_catalog_status: "partial",
    rule_pack_crosswalk_status: "prototype",
    pwa_snapshot: {
      snapshotVersion: `${snapshot.snapshot_version}:pwa-adapter-prototype`,
      sceneTypes: snapshot.scene_types.map((item) => item.id),
      allowedDurations: snapshot.allowed_durations,
      writingRulePacks: snapshot.writing_rule_packs.map(toPwaRulePack),
      directorRulePacks: snapshot.director_rule_packs.map(toPwaRulePack),
      validationRulePacks: snapshot.validation_rule_packs.map(toPwaRulePack),
      sceneMappings: toPwaSceneMappings(snapshot.scene_mappings),
      durationProfiles: snapshot.duration_profiles.map(toPwaDurationProfile),
    },
    sanitized_summary_contract: {
      summary_only: true,
      allowed_fields: [
        "kb_snapshot_hash",
        "scene_type_count",
        "scene_type_id",
        "scene_type_label",
        "duration_options",
        "selected_sample_ids",
        "selected_kb_rules",
        "writing_group_rule_pack_ids",
        "director_group_rule_pack_ids",
        "kb_context_summary",
        "applied_to",
        "action_results",
        "influence_axes",
        "scene_profile",
        "negative_constraints",
        "kb_oracle_affects_structure",
        "raw_kb_rows_included",
        "raw_sample_text_absent",
        "source_register_absent",
        "overlay_json_absent",
        "prompt_body_absent",
      ],
      denied_fields: [
        "raw_kb_rows",
        "raw_source_text",
        "raw_graph",
        "source_register",
        "overlay_json",
        "prompt_body",
        "provider_config",
        "api_key",
        "token",
        "secret",
        "local_path",
      ],
    },
    fail_closed: {
      fallback_required: true,
      fallback_target: "hope-web-pwa built-in KB_SNAPSHOT or last-known-good adapter output",
      reject_when: [
        "missing_or_unparseable",
        "missing_required_camel_case_field",
        "unsafe_runtime_safety_flag",
        "partial_scene_catalog_used_as_full_product_catalog",
        "unresolved_rule_pack_reference",
        "raw_or_secret_field_present",
      ],
    },
  };
}

function parseArgs(argv) {
  const options = {
    snapshotPath: defaultSnapshotPath,
    outputPath: defaultOutputPath,
    check: false,
  };
  for (let index = 2; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--check") {
      options.check = true;
    } else if (arg === "--snapshot") {
      options.snapshotPath = path.resolve(argv[++index]);
    } else if (arg === "--out") {
      options.outputPath = path.resolve(argv[++index]);
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
  }
  return options;
}

function main() {
  const options = parseArgs(process.argv);
  const snapshot = readJson(options.snapshotPath);
  const adapterOutput = buildAdapterOutput(snapshot);
  const nextText = `${JSON.stringify(adapterOutput, null, 2)}\n`;

  if (options.check) {
    const currentText = fs.existsSync(options.outputPath) ? fs.readFileSync(options.outputPath, "utf8") : "";
    if (currentText !== nextText) {
      console.error(`PWA adapter output is stale: ${path.relative(repoRoot, options.outputPath)}`);
      process.exit(1);
    }
    console.log(
      JSON.stringify({
        status: "passed",
        output: path.relative(repoRoot, options.outputPath),
        scene_count: adapterOutput.pwa_snapshot.sceneTypes.length,
        rule_pack_count:
          adapterOutput.pwa_snapshot.writingRulePacks.length +
          adapterOutput.pwa_snapshot.directorRulePacks.length +
          adapterOutput.pwa_snapshot.validationRulePacks.length,
      }),
    );
    return;
  }

  writeJson(options.outputPath, adapterOutput);
  console.log(
    JSON.stringify({
      status: "written",
      output: path.relative(repoRoot, options.outputPath),
      scene_count: adapterOutput.pwa_snapshot.sceneTypes.length,
      rule_pack_count:
        adapterOutput.pwa_snapshot.writingRulePacks.length +
        adapterOutput.pwa_snapshot.directorRulePacks.length +
        adapterOutput.pwa_snapshot.validationRulePacks.length,
    }),
  );
}

main();
