const fs = require("fs");
const path = require("path");

const allowedFields = new Set(["visual_description", "character_action", "camera", "shot_size", "prompt_text", "scene_profile", "negative_constraints", "kb_context_summary", "selected_kb_rules", "person", "duration_seconds"]);
const allowedActions = new Set(["generate_storyboard", "repair_storyboard", "validate_result", "create_story_task", "export_result"]);
const allowedTargets = new Set(["writing_rule_packs", "director_rule_packs", "validation_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary", "negative_constraints", "duration_profiles"]);
const allowedPackageStatuses = new Set(["request_recorded", "sources_collected", "screened", "recommendations_ready", "confirmed_package", "applied_to_kb", "failed"]);
const allowedRecommendationStatuses = new Set(["ready_for_confirmation", "future_qa", "confirmed", "waiting_user_confirmation", "proposal_only"]);
const runtimeSourceDecisions = new Set(["keep"]);
const deniedKeyPattern = /^(raw_source_text|raw_kb_rows|prompt_body|source_register|overlay_json|raw_graph|provider_config|api_key|token|secret|credential|local_path|source_image|ocr_text)$/i;
const deniedValuePattern = /([A-Za-z]:\\|sk-[A-Za-z0-9]{12,}|Bearer\s+[A-Za-z0-9._-]{20,}|-----BEGIN [A-Z ]+PRIVATE KEY-----)/;

function parseArgs(argv) {
  const out = { packagePath: "" };
  for (let i = 2; i < argv.length; i += 1) {
    if (argv[i] === "--package") out.packagePath = path.resolve(argv[++i]);
    else throw new Error("Unknown argument: " + argv[i]);
  }
  if (!out.packagePath) throw new Error("Missing --package <path>");
  return out;
}
function readJson(filePath) { return JSON.parse(fs.readFileSync(filePath, "utf8")); }
function walk(value, trail, errors) {
  if (value === null || value === undefined) return;
  if (typeof value === "string") {
    if (deniedValuePattern.test(value)) errors.push("Denied raw/path/secret-like value at " + trail);
    return;
  }
  if (Array.isArray(value)) {
    value.forEach(function(item, index) { walk(item, trail + "[" + index + "]", errors); });
    return;
  }
  if (typeof value === "object") {
    Object.entries(value).forEach(function(pair) {
      const key = pair[0];
      const child = pair[1];
      const next = trail ? trail + "." + key : key;
      if (deniedKeyPattern.test(key)) errors.push("Denied field name at " + next);
      walk(child, next, errors);
    });
  }
}
function validatePackage(pkg) {
  const errors = [];
  if (pkg.schema_version !== "hope-kb-manual-intake-package/v0.2") errors.push("schema_version must be hope-kb-manual-intake-package/v0.2");
  if (!pkg.run_id) errors.push("run_id is required");
  if (!allowedPackageStatuses.has(pkg.status)) errors.push("bad package status: " + pkg.status);
  if (!pkg.write_policy || pkg.write_policy.manual_intake_only !== true || pkg.write_policy.requires_user_confirmation !== true) errors.push("manual write policy is required");
  if (pkg.write_policy && (pkg.write_policy.dashboard_may_write_reviewed_wiki !== false || pkg.write_policy.dashboard_may_write_runtime !== false)) errors.push("dashboard must not write reviewed_wiki or runtime directly");
  if (!Array.isArray(pkg.source_candidates)) errors.push("source_candidates must be an array");
  if (!Array.isArray(pkg.screening_results)) errors.push("screening_results must be an array");
  if (!Array.isArray(pkg.recommendations)) errors.push("recommendations must be an array");
  const sourceIds = new Set((pkg.source_candidates || []).map(function(item) { return item.source_id; }));
  (pkg.source_candidates || []).forEach(function(candidate) {
    if (!candidate.source_id) errors.push("source candidate missing source_id");
    if (!candidate.source_url) errors.push("source candidate missing source_url: " + (candidate.source_id || "unknown"));
    const fields = candidate.serves_pwa_fields || [];
    if (!candidate.decision) errors.push("source candidate missing decision: " + candidate.source_id);
    if (runtimeSourceDecisions.has(candidate.decision) && !fields.some(function(field) { return allowedFields.has(field); })) errors.push("source candidate does not serve PWA fields: " + candidate.source_id);
  });
  const acceptedIds = new Set(((pkg.confirmation && pkg.confirmation.accepted_reviewed_wiki_ids) || []).filter(Boolean));
  const recommendationIds = new Set((pkg.recommendations || []).map(function(rec) { return rec.reviewed_wiki_id; }));
  (pkg.recommendations || []).forEach(function(rec) {
    if (!/^rw-[a-z0-9-]+$/.test(rec.reviewed_wiki_id || "")) errors.push("bad reviewed_wiki_id: " + rec.reviewed_wiki_id);
    if (rec.status && !allowedRecommendationStatuses.has(rec.status)) errors.push("bad recommendation status: " + rec.reviewed_wiki_id + " -> " + rec.status);
    if (!rec.chinese_title) errors.push("missing chinese_title: " + rec.reviewed_wiki_id);
    const fields = rec.serves_pwa_fields || [];
    if (!fields.some(function(field) { return allowedFields.has(field); })) errors.push("recommendation does not serve PWA fields: " + rec.reviewed_wiki_id);
    const actions = rec.target_pwa_actions || [];
    if (!actions.some(function(action) { return allowedActions.has(action); })) errors.push("recommendation does not target PWA actions: " + rec.reviewed_wiki_id);
    const targets = rec.runtime_targets || [];
    if (!targets.some(function(target) { return allowedTargets.has(target); })) errors.push("recommendation has no allowed runtime target: " + rec.reviewed_wiki_id);
    (rec.source_candidate_refs || []).forEach(function(ref) { if (!sourceIds.has(ref)) errors.push("recommendation references missing source candidate: " + rec.reviewed_wiki_id + " -> " + ref); });
    if (rec.status === "confirmed" && (!pkg.confirmation || pkg.confirmation.confirmed !== true)) errors.push("confirmed recommendation without package confirmation: " + rec.reviewed_wiki_id);
    if (rec.status === "confirmed" && pkg.confirmation && pkg.confirmation.confirmed === true && !acceptedIds.has(rec.reviewed_wiki_id)) errors.push("confirmed recommendation missing from accepted ids: " + rec.reviewed_wiki_id);
  });
  if (pkg.status === "confirmed_package" && (!pkg.confirmation || pkg.confirmation.confirmed !== true)) errors.push("confirmed_package status requires confirmation.confirmed=true");
  if (pkg.status === "confirmed_package" && (!pkg.confirmation || !Array.isArray(pkg.confirmation.accepted_reviewed_wiki_ids) || !pkg.confirmation.accepted_reviewed_wiki_ids.length)) errors.push("confirmed_package requires non-empty confirmation.accepted_reviewed_wiki_ids");
  if (pkg.status === "applied_to_kb" && (!pkg.kb_application || pkg.kb_application.applied !== true || pkg.kb_application.validation_status !== "passed")) errors.push("applied_to_kb status requires kb_application.applied=true and validation_status=passed");
  ((pkg.confirmation && pkg.confirmation.accepted_reviewed_wiki_ids) || []).forEach(function(id) {
    if (!recommendationIds.has(id)) errors.push("accepted id does not match a recommendation: " + id);
  });
  walk(pkg, "", errors);
  return errors;
}
function main() {
  const options = parseArgs(process.argv);
  const pkg = readJson(options.packagePath);
  const errors = validatePackage(pkg);
  if (errors.length) {
    console.error(JSON.stringify({ status: "failed", error_count: errors.length, errors: errors }, null, 2));
    process.exit(1);
  }
  console.log(JSON.stringify({ status: "passed", package: path.basename(options.packagePath), run_id: pkg.run_id, source_candidates: pkg.source_candidates.length, recommendations: pkg.recommendations.length, confirmed: !!(pkg.confirmation && pkg.confirmation.confirmed) }, null, 2));
}
main();
