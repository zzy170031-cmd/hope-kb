const fs = require("fs");
const http = require("http");
const path = require("path");
const crypto = require("crypto");
const { spawnSync } = require("child_process");

const repoRoot = path.resolve(__dirname, "..");
const root = path.join(repoRoot, "web", "kb-flow-dashboard");
const runsRoot = path.join(repoRoot, "knowledge", "intake_runs");
const adapterSamplePath = path.join(repoRoot, "samples", "pwa-kb-adapter-output.sample.json");
const host = "127.0.0.1";
function parseEnvValue(value) {
  const trimmed = String(value || "").trim();
  if ((trimmed.startsWith("\"") && trimmed.endsWith("\"")) || (trimmed.startsWith("'") && trimmed.endsWith("'"))) return trimmed.slice(1, -1);
  return trimmed;
}
function loadSearchEnvFile() {
  const userProfile = process.env.USERPROFILE || process.env.HOME || "";
  const candidates = [
    process.env.HOPE_KB_SEARCH_ENV_FILE,
    process.env.HOPE_KB_DASHBOARD_ENV_FILE,
    userProfile ? path.join(userProfile, "Desktop", "换机用", "hope-kb-search.env") : "",
    userProfile ? path.join(userProfile, "Desktop", "hope-kb-search.env") : ""
  ].filter(Boolean);
  const file = candidates.find((candidate) => fs.existsSync(candidate));
  if (!file) return { loaded: false, checked: candidates };
  const lines = fs.readFileSync(file, "utf8").replace(/^\uFEFF/, "").split(/\r?\n/);
  let count = 0;
  lines.forEach((line) => {
    const match = line.match(/^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*?)\s*$/);
    if (!match) return;
    const key = match[1];
    if (process.env[key]) return;
    process.env[key] = parseEnvValue(match[2]);
    count += 1;
  });
  return { loaded: true, file, count };
}
const envFileStatus = loadSearchEnvFile();
let lastProviderTestResult = null;
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
function searchConfig() {
  const baseUrl = process.env.HOPE_KB_SEARCH_BASE_URL || process.env.HOPE_WEB_BASE_URL || "https://dashscope.aliyuncs.com/compatible-mode/v1";
  const enableSearch = !/^(0|false|off)$/i.test(process.env.HOPE_KB_ENABLE_SEARCH || "true");
  const requestedTimeoutMs = Number(process.env.HOPE_KB_SEARCH_TIMEOUT_MS || 30000);
  return {
    provider: process.env.HOPE_KB_SEARCH_PROVIDER || process.env.HOPE_WEB_PROVIDER || "qwen",
    baseUrl,
    endpoint: process.env.HOPE_KB_SEARCH_ENDPOINT || process.env.HOPE_WEB_ENDPOINT || "/chat/completions",
    model: process.env.HOPE_KB_SEARCH_MODEL || process.env.HOPE_WEB_MODEL || "qwen3.6-plus",
    key: process.env.HOPE_KB_SEARCH_KEY || process.env.HOPE_KB_API_KEY || process.env.HOPE_WEB_API_KEY || process.env.HOPE_TEXT_MODEL_API_KEY || process.env.DASHSCOPE_API_KEY || "",
    enableSearch,
    searchStrategy: process.env.HOPE_KB_SEARCH_STRATEGY || "turbo",
    enableThinking: /^(1|true|on)$/i.test(process.env.HOPE_KB_ENABLE_THINKING || "false"),
    timeoutMs: Number.isFinite(requestedTimeoutMs) && requestedTimeoutMs > 0 ? requestedTimeoutMs : 30000
  };
}
function hostOf(value) {
  try { return new URL(value).host; } catch { return ""; }
}
function endpointUrl(config) {
  return config.baseUrl.replace(/\/+$/, "") + "/" + config.endpoint.replace(/^\/+/, "");
}
function providerStatus(config, status, extra) {
  return Object.assign({
    status,
    provider: config.provider,
    model: config.model,
    base_url_host: hostOf(config.baseUrl),
    credential_state: config.key ? "configured" : "missing",
    web_search: config.enableSearch ? "enabled" : "disabled"
  }, extra || {});
}
function publicProviderStatus(extra) {
  const config = searchConfig();
  const latest = latestPackage();
  const latestSearch = latest && latest.search_plan && latest.search_plan.search_provider_status;
  return Object.assign({
    env_loaded: !!envFileStatus.loaded,
    env_file_name: envFileStatus.loaded ? path.basename(envFileStatus.file) : "",
    provider: config.provider,
    model: config.model,
    base_url_host: hostOf(config.baseUrl),
    credential_state: config.key ? "configured" : "missing",
    enable_search: !!config.enableSearch,
    search_strategy: config.searchStrategy,
    enable_thinking: !!config.enableThinking,
    timeout_ms: config.timeoutMs,
    last_test_result: lastProviderTestResult,
    latest_run_id: latest ? latest.run_id : "",
    latest_search_status: latestSearch ? latestSearch.status : "none",
    latest_search_reason: latestSearch ? latestSearch.reason : "",
    effective_search_mode: latestSearch ? latestSearch.status : "not_started"
  }, extra || {});
}
async function testProvider() {
  const config = searchConfig();
  if (!config.key) return { status: "failed", reason: "missing_credential", tested_at: new Date().toISOString() };
  if (typeof fetch !== "function") return { status: "failed", reason: "fetch_unavailable", tested_at: new Date().toISOString() };
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), Math.min(config.timeoutMs, 15000));
  try {
    const response = await fetch(endpointUrl(config), {
      method: "POST",
      signal: controller.signal,
      headers: { "content-type": "application/json", authorization: "Bearer " + config.key },
      body: JSON.stringify({
        model: config.model,
        messages: [
          { role: "system", content: "Return a short JSON object only." },
          { role: "user", content: "{\"ping\":\"hope-kb-provider-test\"}" }
        ],
        temperature: 0
      })
    });
    const body = await response.text();
    return {
      status: response.ok ? "passed" : "failed",
      reason: response.ok ? "ok" : "http_" + response.status,
      tested_at: new Date().toISOString(),
      http_status: response.status,
      response_bytes: body.length
    };
  } catch (error) {
    return {
      status: "failed",
      reason: error && error.name === "AbortError" ? "timeout" : "request_failed",
      tested_at: new Date().toISOString(),
      error_summary: String(error && error.message ? error.message : error).slice(0, 180)
    };
  } finally {
    clearTimeout(timer);
  }
}
function jsonFromModelText(text) {
  const trimmed = String(text || "").trim();
  if (!trimmed) throw new Error("model returned empty content");
  if (trimmed.startsWith("{") || trimmed.startsWith("[")) return JSON.parse(trimmed);
  const start = Math.min(...["{", "["].map((char) => trimmed.indexOf(char)).filter((index) => index >= 0));
  const end = Math.max(trimmed.lastIndexOf("}"), trimmed.lastIndexOf("]"));
  if (!Number.isFinite(start) || end <= start) throw new Error("model response did not contain JSON");
  return JSON.parse(trimmed.slice(start, end + 1));
}
function asArray(value) {
  return Array.isArray(value) ? value : [];
}
function asText(value, fallback) {
  const text = String(value || "").replace(/\s+/g, " ").trim();
  return text || fallback || "";
}
function asUrl(value) {
  const text = asText(value, "");
  if (!/^https?:\/\//i.test(text)) return "";
  return text;
}
function safeSourceId(value, index) {
  const id = slug(value || ("source-" + (index + 1))).replace(/^-+|-+$/g, "");
  return id.startsWith("src-") ? id : "src-" + id;
}
function decisionLabel(decision) {
  if (decision === "keep") return "保留";
  if (decision === "reject") return "拒绝";
  return "后续复核";
}
function normalizeSourceCandidate(raw, index, runId, acquisitionStep) {
  const fields = asArray(raw.serves_pwa_fields).filter((field) => targetFields.includes(field));
  const url = asUrl(raw.source_url || raw.url || raw.link);
  if (!url) return null;
  const riskFlags = asArray(raw.risk_flags).map((item) => asText(item, "")).filter(Boolean);
  const decision = fields.length ? (raw.decision === "reject" ? "reject" : "keep") : "future_qa";
  return {
    source_id: safeSourceId(raw.source_id || raw.id || raw.title, index),
    title: asText(raw.title, "未命名候选"),
    source_url: url,
    source_type_label: asText(raw.source_type_label || raw.source_type, "互联网候选"),
    language: asText(raw.language, "未标注"),
    confidence: asText(raw.confidence, "medium"),
    decision,
    decision_label: decisionLabel(decision),
    risk_flags: riskFlags,
    serves_pwa_fields: fields,
    target_pwa_actions: asArray(raw.target_pwa_actions).filter((action) => ["create_story_task", "generate_storyboard", "repair_storyboard", "validate_result", "export_result"].includes(action)),
    pwa_relevance: asText(raw.pwa_relevance || raw.summary, "需进一步确认其对 PWA 字段的帮助。"),
    kb_gap: asText(raw.kb_gap, ""),
    leakage_check: asText(raw.leakage_check, "summary_only_no_raw_source"),
    run_id: runId,
    acquisition_step: acquisitionStep
  };
}
function fallbackSourceCandidates(pkg, acquisitionStep) {
  return loadSources().map((item, index) => normalizeSourceCandidate(item, index, pkg.run_id, acquisitionStep)).filter(Boolean);
}
function searchPrompt(pkg) {
  return [
    "请为 Hope-KB 手动入库控制台生成小批量互联网搜索候选。",
    "只输出 JSON，不要 Markdown。不要输出原文摘录，不要输出密钥、本地路径、raw prompt、raw KB。",
    "候选必须能服务 PWA 分镜字段之一：visual_description, character_action, camera, shot_size, prompt_text。",
    "优先选择官方文档、主流采访、制作解析、可信教程。每个候选必须有可访问的 http/https source_url。",
    "输出最多 5 条候选，字段结构为：",
    JSON.stringify({
      source_candidates: [{
        source_id: "src-short-id",
        title: "候选标题",
        source_url: "https://example.com",
        source_type_label: "官方文档/主流采访/制作解析/可信教程",
        language: "中文/英文",
        confidence: "high/medium/low",
        decision: "keep/future_qa/reject",
        serves_pwa_fields: targetFields,
        target_pwa_actions: ["create_story_task", "generate_storyboard", "repair_storyboard"],
        pwa_relevance: "一句话说明如何改善 PWA 输出字段",
        kb_gap: "一句话说明填补的 KB 缺口",
        risk_flags: ["summary_only"],
        leakage_check: "summary_only_no_raw_source"
      }]
    })
  ].join("\n");
}
async function requestSourceCandidates(pkg, config, options) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), options.timeoutMs || config.timeoutMs);
  try {
    const payload = {
      model: config.model,
      messages: [
        { role: "system", content: searchPrompt(pkg) },
        { role: "user", content: JSON.stringify({ demand: pkg.demand, generated_queries: pkg.search_plan.generated_queries, required_answers: pkg.search_plan.required_answers }) }
      ],
      temperature: 0.2,
      max_tokens: 1200
    };
    if (/^(1|true|on)$/i.test(process.env.HOPE_KB_RESPONSE_FORMAT_JSON || "")) payload.response_format = { type: "json_object" };
    payload.enable_thinking = !!config.enableThinking;
    if (options.useSearch) {
      payload.enable_search = true;
      payload.search_options = { search_strategy: config.searchStrategy };
    }
    const response = await fetch(endpointUrl(config), {
      method: "POST",
      signal: controller.signal,
      headers: { "content-type": "application/json", authorization: "Bearer " + config.key },
      body: JSON.stringify(payload)
    });
    if (!response.ok) throw new Error("HTTP " + response.status + " " + response.statusText);
    const json = await response.json();
    const content = json && json.choices && json.choices[0] && json.choices[0].message && json.choices[0].message.content;
    const parsed = jsonFromModelText(content);
    return asArray(parsed.source_candidates || parsed.candidates)
      .map((item, index) => normalizeSourceCandidate(item, index, pkg.run_id, options.acquisitionStep))
      .filter(Boolean)
      .slice(0, 5);
  } finally {
    clearTimeout(timer);
  }
}
async function fetchLiveSourceCandidates(pkg) {
  const config = searchConfig();
  if (!config.key) {
    return {
      candidates: fallbackSourceCandidates(pkg, "registered_source_candidate_fallback"),
      status: providerStatus(config, "fixture_fallback", { reason: "missing_credential", candidate_count: 0 })
    };
  }
  if (typeof fetch !== "function") {
    return {
      candidates: fallbackSourceCandidates(pkg, "registered_source_candidate_fallback"),
      status: providerStatus(config, "fixture_fallback", { reason: "fetch_unavailable", candidate_count: 0 })
    };
  }
  try {
    const candidates = await requestSourceCandidates(pkg, config, { useSearch: config.enableSearch, acquisitionStep: config.enableSearch ? "hope_env_web_search" : "hope_env_model_search" });
    if (!candidates.length) {
      return {
        candidates: fallbackSourceCandidates(pkg, "registered_source_candidate_fallback"),
        status: providerStatus(config, "fixture_fallback", { reason: "no_url_bound_candidates", candidate_count: 0 })
      };
    }
    return {
      candidates,
      status: providerStatus(config, config.enableSearch ? "live_search" : "model_search", { reason: "ok", candidate_count: candidates.length })
    };
  } catch (error) {
    const errorSummary = asText(error && error.message, "unknown error").slice(0, 180);
    const timedOut = /aborted|abort|timeout|timed out/i.test(errorSummary);
    if (config.enableSearch) {
      try {
        const candidates = await requestSourceCandidates(pkg, config, { useSearch: false, acquisitionStep: "hope_env_model_fallback", timeoutMs: config.timeoutMs });
        if (candidates.length) {
          return {
            candidates,
            status: providerStatus(config, "model_fallback", { reason: timedOut ? "live_search_timeout_model_ok" : "live_search_failed_model_ok", error_summary: errorSummary, candidate_count: candidates.length })
          };
        }
      } catch (fallbackError) {
        return {
          candidates: fallbackSourceCandidates(pkg, "registered_source_candidate_fallback"),
          status: providerStatus(config, "fixture_fallback", { reason: timedOut ? "live_search_timeout_model_failed" : "live_search_failed_model_failed", error_summary: (errorSummary + " / " + asText(fallbackError && fallbackError.message, "model fallback failed")).slice(0, 180), candidate_count: 0 })
        };
      }
    }
    return {
      candidates: fallbackSourceCandidates(pkg, "registered_source_candidate_fallback"),
      status: providerStatus(config, "fixture_fallback", { reason: timedOut ? "live_search_timeout" : "live_search_failed", error_summary: errorSummary, candidate_count: 0 })
    };
  }
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
    kb_application: { applied: false, applied_at: "", command: "", validation_status: "not_run", output: null },
    artifacts: {
      draft_package: "knowledge/intake_runs/" + runId + "/intake-package.draft.json",
      confirmed_package: "knowledge/intake_runs/" + runId + "/intake-package.confirmed.json",
      reviewed_wiki: "knowledge/reviewed_wiki",
      wiki_to_runtime_mapping: "knowledge/mappings/wiki-to-runtime-mapping.v0.2.json",
      runtime_snapshot: "knowledge/runtime_snapshots/latest.candidate.json",
      pwa_adapter_output: "samples/pwa-kb-adapter-output.sample.json"
    }
  };
}
async function sourceStep(pkg) {
  const result = await fetchLiveSourceCandidates(pkg);
  pkg.source_candidates = result.candidates;
  pkg.search_plan.search_provider_status = result.status;
  pkg.search_plan.acquisition_mode = result.status.status === "live_search" ? "hope_env_web_search" : "registered_source_candidate_fallback";
  pkg.status = "sources_collected";
  return pkg;
}
function screenStep(pkg) {
  if (!pkg.source_candidates.length) {
    pkg.source_candidates = fallbackSourceCandidates(pkg, "registered_source_candidate_fallback");
    pkg.search_plan.search_provider_status = providerStatus(searchConfig(), "fixture_fallback", { reason: "screen_without_source_step", candidate_count: pkg.source_candidates.length });
  }
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
  const keptSources = pkg.source_candidates.filter((item) => keptIds.has(item.source_id));
  pkg.recommendations = recommendationCatalog.map((rec) => {
    const sourceRefs = rec.source_candidate_refs || [];
    const catalogMatch = sourceRefs.some((id) => keptIds.has(id));
    const fieldMatch = keptSources.some((source) => (source.serves_pwa_fields || []).some((field) => (rec.serves_pwa_fields || []).includes(field)));
    const activatedRefs = catalogMatch ? sourceRefs.filter((id) => keptIds.has(id)) : keptSources.filter((source) => (source.serves_pwa_fields || []).some((field) => (rec.serves_pwa_fields || []).includes(field))).map((source) => source.source_id).slice(0, 3);
    return Object.assign({}, rec, { source_candidate_refs: activatedRefs, status: catalogMatch || fieldMatch ? "ready_for_confirmation" : "future_qa", write_state: "not_written", runtime_state: "not_runtime" });
  });
  pkg.status = "recommendations_ready";
  return pkg;
}
function confirmableRecommendation(rec) {
  if (!rec || !rec.reviewed_wiki_id) return false;
  if (rec.status === "ready_for_confirmation") return true;
  return rec.status === "confirmed" && rec.write_state !== "written" && rec.runtime_state !== "runtime";
}
function confirmStep(pkg, body) {
  if (!pkg.recommendations.length) recommendationStep(pkg);
  const ready = new Set(pkg.recommendations.filter(confirmableRecommendation).map((rec) => rec.reviewed_wiki_id));
  const requested = asArray(body && (body.accepted_reviewed_wiki_ids || body.accepted_ids || body.reviewed_wiki_ids)).map((id) => asText(id, "")).filter(Boolean);
  if (!requested.length) throw new Error("No recommendations selected for confirmation.");
  const accepted = Array.from(new Set(requested)).filter((id) => ready.has(id));
  const invalid = requested.filter((id) => !ready.has(id));
  if (invalid.length) throw new Error("Selected recommendations are not confirmable: " + invalid.join(", "));
  if (!accepted.length) throw new Error("No confirmable recommendations selected.");
  pkg.recommendations = pkg.recommendations.map((rec) => {
    if (!ready.has(rec.reviewed_wiki_id)) return rec;
    return Object.assign({}, rec, { status: accepted.includes(rec.reviewed_wiki_id) ? "confirmed" : "ready_for_confirmation" });
  });
  pkg.confirmation = { confirmed: true, confirmed_at: new Date().toISOString(), confirmed_by: "dashboard_manual_button", accepted_reviewed_wiki_ids: accepted, available_reviewed_wiki_ids: Array.from(ready) };
  pkg.status = "confirmed_package";
  pkg.kb_application.command = "node scripts/apply-confirmed-intake-package.js --package " + path.relative(repoRoot, confirmedPath(pkg.run_id));
  return pkg;
}
function sanitizeProcessText(value) {
  return asText(value, "")
    .replace(/[A-Za-z]:\\[^\s"',}]+/g, "[local_path]")
    .replace(/Bearer\s+[A-Za-z0-9._-]{20,}/g, "Bearer [redacted]")
    .replace(/sk-[A-Za-z0-9]{12,}/g, "sk-[redacted]")
    .slice(0, 1200);
}
function defaultPwaRoot() {
  const userProfile = process.env.USERPROFILE || process.env.HOME || "";
  return userProfile ? path.join(userProfile, "Documents", "New project", "hope-web-pwa-inspect") : "";
}
function configuredPwaRoot() {
  const candidate = process.env.HOPE_KB_PWA_ROOT || process.env.HOPE_WEB_PWA_ROOT || defaultPwaRoot();
  return candidate ? path.resolve(candidate) : "";
}
function fileHash(file) {
  if (!fs.existsSync(file)) return "";
  return crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex").toUpperCase();
}
function shortHash(hash) {
  return hash ? hash.slice(0, 12) : "";
}
function pwaStatus() {
  const pwaRoot = configuredPwaRoot();
  const publicTarget = path.join(pwaRoot, "public", "kb", "latest.json");
  const distTarget = path.join(pwaRoot, "dist", "kb", "latest.json");
  const sampleHash = fileHash(adapterSamplePath);
  const publicHash = fileHash(publicTarget);
  const distHash = fileHash(distTarget);
  const rootExists = !!pwaRoot && fs.existsSync(pwaRoot);
  const publicSynced = !!sampleHash && sampleHash === publicHash;
  const distSynced = !!sampleHash && sampleHash === distHash;
  return {
    status: rootExists && publicSynced && distSynced ? "synced" : "pending",
    pwa_root_configured: !!pwaRoot,
    pwa_root_exists: rootExists,
    pwa_root_label: pwaRoot ? path.basename(pwaRoot) : "",
    sample_exists: fs.existsSync(adapterSamplePath),
    sample_hash: shortHash(sampleHash),
    public_latest_exists: fs.existsSync(publicTarget),
    public_latest_hash: shortHash(publicHash),
    public_latest_synced: publicSynced,
    dist_latest_exists: fs.existsSync(distTarget),
    dist_latest_hash: shortHash(distHash),
    dist_latest_synced: distSynced,
    targets: ["public/kb/latest.json", "dist/kb/latest.json"],
    checked_at: new Date().toISOString()
  };
}
function syncPwaLatest() {
  const pwaRoot = configuredPwaRoot();
  if (!pwaRoot || !fs.existsSync(pwaRoot)) throw new Error("PWA root not found.");
  const result = spawnSync(process.execPath, [path.join(repoRoot, "scripts", "sync-pwa-kb-latest.js"), "--pwa-root", pwaRoot], { cwd: repoRoot, encoding: "utf8" });
  let parsed = null;
  try { parsed = JSON.parse(result.stdout); } catch {}
  return {
    status: result.status === 0 && !result.error ? "passed" : "failed",
    exit_status: result.status,
    error_summary: result.error ? sanitizeProcessText(result.error.message) : "",
    adapter_check_passed: !!(parsed && parsed.adapter_check && parsed.adapter_check.passed),
    copy_passed: !!(parsed && parsed.copy && parsed.copy.passed),
    pwa_test_passed: !!(parsed && parsed.pwa_test && parsed.pwa_test.passed),
    targets: parsed && Array.isArray(parsed.targets) ? parsed.targets : ["public\\kb\\latest.json", "dist\\kb\\latest.json"],
    stdout_summary: sanitizeProcessText(result.stdout),
    stderr_summary: sanitizeProcessText(result.stderr),
    synced_at: new Date().toISOString(),
    pwa: pwaStatus()
  };
}
function applyStep(pkg) {
  if (!pkg.confirmation || pkg.confirmation.confirmed !== true) throw new Error("Package must be confirmed before apply.");
  if (!asArray(pkg.confirmation.accepted_reviewed_wiki_ids).length) throw new Error("Package confirmation has no accepted reviewed_wiki ids.");
  writePackage(pkg, true);
  const result = spawnSync(process.execPath, [path.join(repoRoot, "scripts", "apply-confirmed-intake-package.js"), "--package", confirmedPath(pkg.run_id)], { cwd: repoRoot, encoding: "utf8" });
  pkg.kb_application = { applied: result.status === 0, applied_at: new Date().toISOString(), command: "node scripts/apply-confirmed-intake-package.js --package " + path.relative(repoRoot, confirmedPath(pkg.run_id)), validation_status: result.status === 0 ? "passed" : "failed", output: { stdout_summary: sanitizeProcessText(result.stdout), stderr_summary: sanitizeProcessText(result.stderr), status: result.status } };
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
    if (request.method === "GET" && url.pathname === "/api/provider/status") return sendJson(response, 200, publicProviderStatus());
    if ((request.method === "GET" || request.method === "POST") && url.pathname === "/api/provider/test") {
      lastProviderTestResult = await testProvider();
      return sendJson(response, 200, publicProviderStatus({ last_test_result: lastProviderTestResult }));
    }
    if (request.method === "GET" && url.pathname === "/api/pwa/status") return sendJson(response, 200, pwaStatus());
    if (request.method === "POST" && url.pathname === "/api/pwa/sync") return sendJson(response, 200, syncPwaLatest());
    if (request.method === "GET" && url.pathname === "/api/intake/latest") return sendJson(response, 200, { package: latestPackage() });
    if (request.method === "POST" && url.pathname === "/api/intake/start") { const pkg = makePackage(await readBody(request)); writePackage(pkg, false); return sendJson(response, 200, { package: pkg }); }
    const match = url.pathname.match(/^\/api\/intake\/([^/]+)\/(sources|screen|recommendations|confirm|apply)$/);
    if (!match || request.method !== "POST") return sendJson(response, 404, { error: "api not found" });
    const runId = match[1];
    const action = match[2];
    let pkg = readPackage(runId);
    if (!pkg) return sendJson(response, 404, { error: "run not found", run_id: runId });
    const actionBody = await readBody(request);
    if (action === "sources") pkg = await sourceStep(pkg);
    if (action === "screen") pkg = screenStep(pkg);
    if (action === "recommendations") pkg = recommendationStep(pkg);
    if (action === "confirm") pkg = confirmStep(pkg, actionBody);
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
server.listen(port, host, () => {
  const envNote = envFileStatus.loaded ? " env:file(" + envFileStatus.count + " vars)" : " env:process";
  console.log("Hope-KB flow dashboard: http://" + host + ":" + port + "/" + envNote);
});
