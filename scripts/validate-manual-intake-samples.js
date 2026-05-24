const fs = require("fs");
const path = require("path");

const repoRoot = path.resolve(__dirname, "..");
const dashboardRoot = path.join(repoRoot, "web", "kb-flow-dashboard");

const files = {
  runs: path.join(dashboardRoot, "intake-runs.sample.json"),
  sources: path.join(dashboardRoot, "source-candidates.sample.json"),
  recommendations: path.join(dashboardRoot, "intake-recommendations.sample.json"),
};

const allowedPwaFields = new Set([
  "person",
  "shot_size",
  "camera",
  "visual_description",
  "character_action",
  "dialogue_or_narration",
  "prompt_text",
  "duration_seconds",
]);

const allowedSourceTypes = new Set(["official_doc", "mainstream_interview", "production_breakdown", "community_tutorial"]);
const allowedConfidence = new Set(["high", "medium_high", "medium", "low"]);
const allowedDecisions = new Set(["keep", "future_qa", "reject"]);
const allowedRecommendationStatuses = new Set(["proposal_only", "waiting_user_confirmation"]);
const deniedRuntimeStatuses = new Set(["reviewed", "runtime_eligible", "approved", "written"]);
const deniedTextPatterns = [
  /source_register/i,
  /overlay_json/i,
  /raw_kb_rows/i,
  /provider_config/i,
  /api[_-]?key/i,
  /authorization/i,
  /bearer/i,
  /secret/i,
  /local_path/i,
  /[A-Za-z]:\\/,
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function fail(errors, message) {
  errors.push(message);
}

function assert(condition, errors, message) {
  if (!condition) fail(errors, message);
}

function list(value) {
  return Array.isArray(value) ? value : [];
}

function assertArray(value, errors, message) {
  assert(Array.isArray(value), errors, message);
}

function assertFields(fields, errors, where, options) {
  const allowEmpty = options && options.allowEmpty;
  assertArray(fields, errors, where + ": serves_pwa_fields must be an array");
  if (!Array.isArray(fields)) return;
  assert(allowEmpty || fields.length > 0, errors, where + ": serves_pwa_fields must not be empty");
  for (const field of fields) {
    assert(allowedPwaFields.has(field), errors, where + ": unsupported PWA field " + field);
  }
}

function visibleText(record, keys) {
  return keys.map(function (key) { return record[key]; }).filter(Boolean).join("\n");
}

function assertNoLeakageText(text, errors, where) {
  for (const pattern of deniedTextPatterns) {
    assert(!pattern.test(text), errors, where + ": public text contains denied leakage pattern " + String(pattern));
  }
}

function main() {
  const errors = [];
  const runs = readJson(files.runs);
  const sources = readJson(files.sources);
  const recommendations = readJson(files.recommendations);

  assert(runs.dashboard_only === true, errors, "runs: dashboard_only must be true");
  assert(runs.write_policy && runs.write_policy.mode === "manual_intake_only", errors, "runs: write policy must be manual_intake_only");
  assert(runs.write_policy && runs.write_policy.can_write_reviewed_wiki === false, errors, "runs: must not write reviewed wiki");
  assert(runs.write_policy && runs.write_policy.can_write_runtime === false, errors, "runs: must not write runtime");
  assertArray(runs.runs, errors, "runs.runs must be an array");
  for (const run of list(runs.runs)) {
    const where = run.run_id || "run without id";
    assert(run.target_model === "Seedance", errors, where + ": target model must be Seedance");
    assertFields(run.target_pwa_fields, errors, where);
    assert(run.current_phase === "waiting_user_confirmation", errors, where + ": sample run must stop at waiting_user_confirmation");
    assert(run.repository_write_state === "not_written", errors, where + ": repository write state must be not_written");
    assert(run.runtime_write_state === "not_runtime", errors, where + ": runtime write state must be not_runtime");
    assertNoLeakageText(visibleText(run, ["title", "manual_gate"]), errors, where);
  }

  assert(sources.dashboard_only === true, errors, "sources: dashboard_only must be true");
  assert(sources.no_runtime_effect === true, errors, "sources: no_runtime_effect must be true");
  assertArray(sources.source_candidates, errors, "sources.source_candidates must be an array");
  const sourceIds = new Set();
  let chineseSourceCount = 0;
  let seedanceOfficialCount = 0;
  let keepCount = 0;
  let rejectedCount = 0;
  for (const source of list(sources.source_candidates)) {
    const where = source.source_id || "source without id";
    assert(/^src-[a-z0-9-]+$/.test(source.source_id || ""), errors, where + ": source_id must start with src-");
    assert(!sourceIds.has(source.source_id), errors, where + ": duplicate source_id");
    sourceIds.add(source.source_id);
    assert(Boolean(source.title), errors, where + ": title is required");
    assert(allowedSourceTypes.has(source.source_type), errors, where + ": unsupported source_type " + source.source_type);
    assert(["中文", "英文", "中文/英文"].includes(source.language), errors, where + ": language must be explicit");
    if (String(source.language).includes("中文")) chineseSourceCount += 1;
    assert(allowedConfidence.has(source.confidence), errors, where + ": unsupported confidence " + source.confidence);
    assert(allowedDecisions.has(source.decision), errors, where + ": unsupported decision " + source.decision);
    if (source.decision === "keep") keepCount += 1;
    if (source.decision === "reject") rejectedCount += 1;
    assertArray(source.risk_flags, errors, where + ": risk_flags must be an array");
    assertFields(source.serves_pwa_fields, errors, where, { allowEmpty: source.decision === "reject" });
    assertArray(source.serves_prompt_sections, errors, where + ": serves_prompt_sections must be an array");
    assertArray(source.target_pwa_actions, errors, where + ": target_pwa_actions must be an array");
    if (source.decision !== "reject") {
      assert(Boolean(source.source_url) && /^https:\/\//.test(source.source_url), errors, where + ": non-rejected source must keep a verified https URL");
      assert(source.serves_prompt_sections.length > 0, errors, where + ": non-rejected source must name prompt sections");
      assert(source.target_pwa_actions.length > 0, errors, where + ": non-rejected source must name PWA actions");
    }
    if (/seedance/i.test(String(source.title) + " " + String(source.source_url)) && source.source_type === "official_doc") {
      seedanceOfficialCount += 1;
    }
    assertNoLeakageText(visibleText(source, ["title", "source_type_label", "kb_gap", "pwa_relevance"]), errors, where);
  }
  assert(chineseSourceCount >= 4, errors, "sources: domestic/Chinese sources must be prioritized");
  assert(seedanceOfficialCount >= 1, errors, "sources: at least one Seedance official source is required");
  assert(keepCount >= 2, errors, "sources: at least two sources must be retainable");
  assert(rejectedCount >= 1, errors, "sources: at least one rejected example is required");

  assert(recommendations.dashboard_only === true, errors, "recommendations: dashboard_only must be true");
  assert(recommendations.no_runtime_effect === true, errors, "recommendations: no_runtime_effect must be true");
  assertArray(recommendations.recommendations, errors, "recommendations.recommendations must be an array");
  const recommendationIds = new Set();
  for (const recommendation of list(recommendations.recommendations)) {
    const where = recommendation.reviewed_wiki_id || "recommendation without id";
    assert(/^rw-[a-z0-9-]+$/.test(recommendation.reviewed_wiki_id || ""), errors, where + ": reviewed_wiki_id must start with rw-");
    assert(!recommendationIds.has(recommendation.reviewed_wiki_id), errors, where + ": duplicate recommendation id");
    recommendationIds.add(recommendation.reviewed_wiki_id);
    assert(Boolean(recommendation.chinese_title), errors, where + ": chinese_title is required");
    assert(/seedance/i.test(recommendation.target_model_context || ""), errors, where + ": target_model_context must include Seedance");
    assertFields(recommendation.serves_pwa_fields, errors, where);
    assertArray(recommendation.serves_prompt_sections, errors, where + ": serves_prompt_sections must be an array");
    assert(recommendation.serves_prompt_sections.length > 0, errors, where + ": prompt sections are required");
    assertArray(recommendation.target_pwa_actions, errors, where + ": target_pwa_actions must be an array");
    assert(recommendation.target_pwa_actions.length > 0, errors, where + ": target PWA actions are required");
    assertArray(recommendation.runtime_targets, errors, where + ": runtime_targets must be an array");
    assert(recommendation.runtime_targets.length > 0, errors, where + ": runtime targets are required for future mapping");
    assertArray(recommendation.source_candidate_refs, errors, where + ": source_candidate_refs must be an array");
    assert(recommendation.source_candidate_refs.length > 0, errors, where + ": at least one source ref is required");
    for (const ref of recommendation.source_candidate_refs || []) {
      assert(sourceIds.has(ref), errors, where + ": unresolved source candidate ref " + ref);
    }
    assertArray(recommendation.existing_kb_alignment_refs, errors, where + ": existing_kb_alignment_refs must be an array");
    assert(recommendation.existing_kb_alignment_refs.length > 0, errors, where + ": must align with existing KB");
    assert(allowedRecommendationStatuses.has(recommendation.status), errors, where + ": status must be proposal_only or waiting_user_confirmation");
    assert(!deniedRuntimeStatuses.has(recommendation.status), errors, where + ": status crosses manual confirmation boundary");
    assert(recommendation.write_state === "not_written", errors, where + ": write_state must be not_written");
    assert(recommendation.runtime_state === "not_runtime", errors, where + ": runtime_state must be not_runtime");
    assertNoLeakageText(visibleText(recommendation, ["reviewed_wiki_id", "chinese_title", "target_model_context"]), errors, where);
  }

  if (errors.length > 0) {
    console.error(JSON.stringify({ status: "failed", errors }, null, 2));
    process.exit(1);
  }

  console.log(JSON.stringify({
    status: "passed",
    runs: runs.runs.length,
    source_candidates: sources.source_candidates.length,
    recommendations: recommendations.recommendations.length,
    boundary: "manual intake only; no reviewed_wiki write; no runtime write",
  }, null, 2));
}

main();
