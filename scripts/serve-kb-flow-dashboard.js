const fs = require("fs");
const http = require("http");
const path = require("path");
const { spawnSync } = require("child_process");

const repoRoot = path.resolve(__dirname, "..");
const root = path.join(repoRoot, "web", "kb-flow-dashboard");
const runsRoot = path.join(repoRoot, "knowledge", "intake_runs");
const host = "127.0.0.1";
const port = Number(process.env.HOPE_KB_DASHBOARD_PORT || 5179);

const contentTypes = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml; charset=utf-8",
};
const targetFields = ["visual_description", "character_action", "camera", "shot_size", "prompt_text"];
const writePolicy = {
  manual_intake_only: true,
  requires_user_confirmation: true,
  dashboard_may_write_reviewed_wiki: false,
  dashboard_may_write_runtime: false,
  apply_script_required: true,
};
const searchCatalog = {
  seedance_anime_prompt: ["Seedance anime prompt storyboard camera action", "Seedance video prompt shot movement duration", "Seedance 多镜头 动漫 分镜 提示词"],
  pwa_storyboard_prompt: ["PWA storyboard prompt_text visual_description character_action camera", "AI video storyboard prompt fields camera shot size action"],
  domestic_anime_action: ["国漫 动作 分镜 运镜 景别 制作解析", "动画 动作戏 分镜 镜头语言 角色动作连续性"],
  anime_camera_language: ["动画 镜头语言 景别 运镜 构图 分镜", "anime camera language shot size movement storyboard"],
  repair_strategy: ["AI video prompt negative constraints repair strategy", "视频生成 提示词 负面约束 修复 分镜"],
};
const recommendationCatalog = [
  { reviewed_wiki_id: "rw-visual-master-consistency", chinese_title: "视觉母版一致性规则", serves_pwa_fields: ["visual_description", "character_action", "camera", "prompt_text"], serves_prompt_sections: ["主体", "画面", "风格", "约束"], target_pwa_actions: ["create_story_task", "generate_storyboard", "repair_storyboard"], runtime_targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules", "kb_context_summary"], source_candidate_refs: ["src-seedance-official-overview", "src-nezha-production-chinanews"], existing_kb_alignment_refs: ["rw-writing-continuity-core", "rw-director-scheduling-core"] },
  { reviewed_wiki_id: "rw-camera-language-grammar", chinese_title: "镜头语言语法规则", serves_pwa_fields: ["camera", "shot_size", "visual_description", "prompt_text"], serves_prompt_sections: ["运镜", "景别", "画面", "节奏"], target_pwa_actions: ["create_story_task", "generate_storyboard", "repair_storyboard"], runtime_targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules"], source_candidate_refs: ["src-seedance-official-overview", "src-nezha2-storyboard-interview-thepaper"], existing_kb_alignment_refs: ["rw-shot-intent-taxonomy", "rw-director-scheduling-core"] },
  { reviewed_wiki_id: "rw-material-light-air-physicality", chinese_title: "材质光影空气物理反馈规则", serves_pwa_fields: ["visual_description", "character_action", "prompt_text", "negative_constraints"], serves_prompt_sections: ["画面", "动作", "约束"], target_pwa_actions: ["generate_storyboard", "repair_storyboard", "validate_result"], runtime_targets: ["director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"], source_candidate_refs: ["src-seedance-tech-report", "src-nezha-production-chinanews"], existing_kb_alignment_refs: ["rw-scene-expression-visible-action", "rw-prompt-text-boundary"] },
  { reviewed_wiki_id: "rw-expression-physicalization", chinese_title: "表情表演物理化规则", serves_pwa_fields: ["character_action", "visual_description", "prompt_text"], serves_prompt_sections: ["动作", "画面", "节奏"], target_pwa_actions: ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result"], runtime_targets: ["director_rule_packs", "selected_kb_rules", "negative_constraints", "kb_context_summary"], source_candidate_refs: ["src-nezha2-storyboard-interview-thepaper", "src-seedance-tech-report"], existing_kb_alignment_refs: ["rw-scene-expression-visible-action", "rw-validation-no-pseudo-success"] },
  { reviewed_wiki_id: "rw-transition-motion-dynamics", chinese_title: "转场与动势调度规则", serves_pwa_fields: ["camera", "shot_size", "prompt_text"], serves_prompt_sections: ["运镜", "景别", "节奏"], target_pwa_actions: ["create_story_task", "generate_storyboard", "repair_storyboard"], runtime_targets: ["director_rule_packs", "scene_mappings", "selected_kb_rules"], source_candidate_refs: ["src-seedance-tech-report", "src-nezha2-storyboard-interview-thepaper"], existing_kb_alignment_refs: ["rw-director-scheduling-core", "rw-duration-density-rules"] },
];
function sendJson(response, status, value) {
  response.writeHead(status, { "content-type": "application/json; charset=utf-8", "cache-control": "no-store" });
  response.end(JSON.stringify(value, null, 2));
}
function readBody(request) {
  return new Promise((resolve, reject) => {
    let data = "";
    request.on("data", (chunk) => { data += chunk; if (data.length > 1024 * 1024) request.destroy(); });
    request.on("end", () => { try { resolve(data ? JSON.parse(data) : {}); } catch (error) { reject(error); } });
    request.on("error", reject);
  });
}
function slug(value) {
  return String(value || "intake").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, 48) || "intake";
}
function runDir(runId) { return path.join(runsRoot, runId); }
function draftPath(runId) { return path.join(runDir(runId), "intake-package.draft.json"); }
function confirmedPath(runId) { return path.join(runDir(runId), "intake-package.confirmed.json"); }
function writePackage(pkg, confirmed) {
  fs.mkdirSync(runDir(pkg.run_id), { recursive: true });
  fs.writeFileSync(confirmed ? confirmedPath(pkg.run_id) : draftPath(pkg.run_id), JSON.stringify(pkg, null, 2) + "\n", "utf8");
}
function readPackage(runId) {
  const c = confirmedPath(runId);
  const d = draftPath(runId);
  const file = fs.existsSync(c) ? c : d;
  if (!fs.existsSync(file)) return null;
  return JSON.parse(fs.readFileSync(file, "utf8"));
}
function latestPackage() {
  if (!fs.existsSync(runsRoot)) return null;
  const dirs = fs.readdirSync(runsRoot, { withFileTypes: true }).filter((d) => d.isDirectory()).map((d) => ({ name: d.name, time: fs.statSync(path.join(runsRoot, d.name)).mtimeMs })).sort((a, b) => b.time - a.time);
  return dirs.length ? readPackage(dirs[0].name) : null;
}
function loadSources() {
  const file = path.join(root, "source-candidates.sample.json");
  const parsed = JSON.parse(fs.readFileSync(file, "utf8"));
  return Array.isArray(parsed.source_candidates) ? parsed.source_candidates : [];
}
function makePackage(body) {
  const runId = "intake-" + new Date().toISOString().replace(/[-:TZ.]/g, "").slice(0, 14) + "-" + slug(body.title || body.search_focus);
  const focus = body.search_focus || body.focus || "seedance_anime_prompt";
  return {
    schema_version: "hope-kb-manual-intake-package/v0.2",
    run_id: runId,
    status: "request_recorded",
    write_policy: writePolicy,
    demand: { title: body.title || "Seedance anime storyboard prompt improvement", search_focus: focus, target_model: "Seedance", target_output: ["PWA storyboard rows", "prompt_text"], target_pwa_fields: targetFields, created_at: new Date().toISOString() },
    search_plan: { acquisition_mode: "manual_web_search_or_registered_source_candidates", live_search_connector: "external_to_dashboard", generated_queries: searchCatalog[focus] || searchCatalog.seedance_anime_prompt, allowed_source_types: ["official_doc", "mainstream_interview", "production_breakdown", "community_tutorial"], required_answers: ["improves which PWA field", "improves which prompt_text section", "maps to which PWA action", "fills which KB gap", "leakage risk"] },
    source_candidates: [],
    screening_results: [],
    recommendations: [],
    confirmation: { confirmed: false, confirmed_at: "", confirmed_by: "", accepted_reviewed_wiki_ids: [] },
    kb_application: { applied: false, applied_at: "", command: "", validation_status: "not_run", output: null }
  };
}
function sourceStep(pkg) {
  const candidates = loadSources().map((item) => Object.assign({}, item, { run_id: pkg.run_id, acquisition_step: "registered_source_candidate" }));
  pkg.source_candidates = candidates;
  pkg.status = "sources_collected";
  return pkg;
}
function screenStep(pkg) {
  if (!pkg.source_candidates.length) sourceStep(pkg);
  pkg.screening_results = pkg.source_candidates.map((item) => {
    const fields = item.serves_pwa_fields || [];
    const servesPwa = fields.some((field) => targetFields.includes(field));
    const leakageOk = String(item.leakage_check || "").includes("no_") || String(item.leakage_check || "").includes("summary");
    const keep = item.decision === "keep" && servesPwa && leakageOk;
    return { source_id: item.source_id, serves_pwa_fields: servesPwa, serves_seedance_prompt: fields.includes("prompt_text") || fields.includes("camera") || fields.includes("character_action"), duplicate_strategy: keep ? "merge_or_new_rule" : "future_qa_or_reject", runtime_target_possible: keep, leakage_risk_ok: leakageOk, result: keep ? "keep" : (item.decision === "future_qa" ? "future_qa" : "reject") };
  });
  pkg.status = "screened";
  return pkg;
}
function recommendationStep(pkg) {
  if (!pkg.screening_results.length) screenStep(pkg);
  const keptIds = new Set(pkg.screening_results.filter((item) => item.result === "keep").map((item) => item.source_id));
  pkg.recommendations = recommendationCatalog.map((rec) => Object.assign({}, rec, { status: rec.source_candidate_refs.some((id) => keptIds.has(id)) ? "ready_for_confirmation" : "future_qa", write_state: "not_written", runtime_state: "not_runtime" }));
  pkg.status = "recommendations_ready";
  return pkg;
}
function confirmStep(pkg) {
  if (!pkg.recommendations.length) recommendationStep(pkg);
  const accepted = pkg.recommendations.filter((rec) => rec.status === "ready_for_confirmation").map((rec) => rec.reviewed_wiki_id);
  pkg.recommendations = pkg.recommendations.map((rec) => accepted.includes(rec.reviewed_wiki_id) ? Object.assign({}, rec, { status: "confirmed" }) : rec);
  pkg.confirmation = { confirmed: true, confirmed_at: new Date().toISOString(), confirmed_by: "dashboard_manual_button", accepted_reviewed_wiki_ids: accepted };
  pkg.status = "confirmed_package";
  pkg.kb_application.command = "node scripts/apply-confirmed-intake-package.js --package " + path.relative(repoRoot, confirmedPath(pkg.run_id));
  return pkg;
}
function applyStep(pkg) {
  if (!pkg.confirmation || pkg.confirmation.confirmed !== true) throw new Error("Package must be confirmed before apply.");
  writePackage(pkg, true);
  const result = spawnSync(process.execPath, [path.join(repoRoot, "scripts", "apply-confirmed-intake-package.js"), "--package", confirmedPath(pkg.run_id)], { cwd: repoRoot, encoding: "utf8" });
  pkg.kb_application = { applied: result.status === 0, applied_at: new Date().toISOString(), command: "node scripts/apply-confirmed-intake-package.js --package " + path.relative(repoRoot, confirmedPath(pkg.run_id)), validation_status: result.status === 0 ? "passed" : "failed", output: { stdout: result.stdout.trim(), stderr: result.stderr.trim(), status: result.status } };
  pkg.status = result.status === 0 ? "applied_to_kb" : "failed";
  writePackage(pkg, true);
  if (result.status !== 0) throw new Error(result.stderr || result.stdout || "apply failed");
  return pkg;
}
function resolveRequestPath(urlPath) {
  const requested = urlPath === "/" ? "index.html" : urlPath.replace(/^\/+/, "");
  const filePath = path.resolve(root, requested);
  if (!filePath.startsWith(root + path.sep) && filePath !== root) return null;
  return filePath;
}
async function handleApi(request, response, url) {
  try {
    if (request.method === "GET" && url.pathname === "/api/intake/latest") return sendJson(response, 200, { package: latestPackage() });
    if (request.method === "POST" && url.pathname === "/api/intake/start") { const pkg = makePackage(await readBody(request)); writePackage(pkg, false); return sendJson(response, 200, { package: pkg }); }
    const match = url.pathname.match(/^\/api\/intake\/([^/]+)\/(sources|screen|recommendations|confirm|apply)$/);
    if (!match || request.method !== "POST") return sendJson(response, 404, { error: "api not found" });
    const runId = match[1];
    const action = match[2];
    let pkg = readPackage(runId);
    if (!pkg) return sendJson(response, 404, { error: "run not found", run_id: runId });
    if (action === "sources") pkg = sourceStep(pkg);
    if (action === "screen") pkg = screenStep(pkg);
    if (action === "recommendations") pkg = recommendationStep(pkg);
    if (action === "confirm") pkg = confirmStep(pkg);
    if (action === "apply") pkg = applyStep(pkg);
    writePackage(pkg, action === "confirm" || action === "apply");
    return sendJson(response, 200, { package: pkg });
  } catch (error) {
    return sendJson(response, 500, { error: String(error && error.message ? error.message : error) });
  }
}
const server = http.createServer((request, response) => {
  const url = new URL(request.url, "http://" + host + ":" + port);
  if (url.pathname.startsWith("/api/")) { handleApi(request, response, url); return; }
  const filePath = resolveRequestPath(url.pathname);
  if (!filePath) { response.writeHead(403, { "content-type": "text/plain; charset=utf-8" }); response.end("forbidden"); return; }
  fs.readFile(filePath, (error, data) => {
    if (error) { response.writeHead(404, { "content-type": "text/plain; charset=utf-8" }); response.end("not found"); return; }
    response.writeHead(200, { "content-type": contentTypes[path.extname(filePath)] || "application/octet-stream", "cache-control": "no-store" });
    response.end(data);
  });
});
server.listen(port, host, () => { console.log("Hope-KB flow dashboard: http://" + host + ":" + port + "/"); });
