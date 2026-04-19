CREATE TABLE IF NOT EXISTS snapshot_meta (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_version TEXT NOT NULL,
  snapshot_name TEXT NOT NULL,
  content_hash TEXT NOT NULL,
  hash_algo TEXT NOT NULL,
  seed_import_format TEXT NOT NULL,
  imported_at TEXT NOT NULL DEFAULT (datetime('now')),
  note TEXT
);

CREATE TABLE IF NOT EXISTS committee_role_definition (
  machine_id TEXT PRIMARY KEY,
  role_code TEXT NOT NULL,
  display_name TEXT NOT NULL,
  responsibility TEXT NOT NULL,
  v0_1_activation_mode TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS director_profile (
  machine_id TEXT PRIMARY KEY,
  导演名 TEXT NOT NULL,
  category TEXT NOT NULL,
  director_role_tags TEXT NOT NULL,
  core_kernel_summary TEXT NOT NULL,
  narrative_kernel TEXT NOT NULL,
  shot_kernel TEXT NOT NULL,
  performance_kernel TEXT NOT NULL,
  rhythm_kernel TEXT NOT NULL,
  transition_kernel TEXT NOT NULL,
  hard_lock_kernel TEXT NOT NULL,
  visual_style_tokens TEXT NOT NULL,
  composition_traits TEXT NOT NULL,
  character_shape_bias TEXT NOT NULL,
  panel_rhythm_bias TEXT NOT NULL,
  color_lighting_mood TEXT NOT NULL,
  emotion_expression_style TEXT NOT NULL,
  negative_prompt_defaults TEXT NOT NULL,
  hard_lock_safe_fields TEXT NOT NULL,
  handoff_compatibility TEXT NOT NULL,
  signature_camera_works TEXT NOT NULL,
  signature_transitions TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS director_rule (
  machine_id TEXT PRIMARY KEY,
  director_machine_id TEXT NOT NULL,
  visual_non_negotiables TEXT NOT NULL,
  camera_rules TEXT NOT NULL,
  dialogue_rules TEXT NOT NULL,
  scene_rules TEXT NOT NULL,
  transition_rules TEXT NOT NULL,
  anti_patterns TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL,
  FOREIGN KEY (director_machine_id) REFERENCES director_profile(machine_id)
);

CREATE INDEX IF NOT EXISTS idx_director_rule_director_machine_id
ON director_rule(director_machine_id);

CREATE TABLE IF NOT EXISTS director_cut_sample (
  machine_id TEXT PRIMARY KEY,
  director_machine_id TEXT NOT NULL,
  scene_type TEXT NOT NULL,
  committee_role_fit TEXT NOT NULL,
  cut标题 TEXT NOT NULL,
  场景 TEXT NOT NULL,
  镜头 TEXT NOT NULL,
  光线 TEXT NOT NULL,
  动作 TEXT NOT NULL,
  情绪 TEXT NOT NULL,
  layout_prompt TEXT NOT NULL,
  render_prompt TEXT NOT NULL,
  hard_lock_focus TEXT NOT NULL,
  continuity_focus TEXT NOT NULL,
  why_it_matches_director TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL,
  FOREIGN KEY (director_machine_id) REFERENCES director_profile(machine_id)
);

CREATE INDEX IF NOT EXISTS idx_director_cut_sample_director_machine_id
ON director_cut_sample(director_machine_id);

CREATE TABLE IF NOT EXISTS director_reference_set (
  machine_id TEXT PRIMARY KEY,
  director_machine_id TEXT NOT NULL,
  代表作品 TEXT NOT NULL,
  公开锚点 TEXT NOT NULL,
  用途说明 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL,
  FOREIGN KEY (director_machine_id) REFERENCES director_profile(machine_id)
);

CREATE INDEX IF NOT EXISTS idx_director_reference_set_director_machine_id
ON director_reference_set(director_machine_id);

CREATE TABLE IF NOT EXISTS director_scene_affinity (
  machine_id TEXT PRIMARY KEY,
  director_machine_id TEXT NOT NULL,
  scene_affinity TEXT NOT NULL,
  detailed_scene_affinity TEXT NOT NULL,
  preferred_scene_types TEXT NOT NULL,
  non_fit_scene_types TEXT NOT NULL,
  selection_note TEXT NOT NULL,
  dispatch_hint TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL,
  FOREIGN KEY (director_machine_id) REFERENCES director_profile(machine_id)
);

CREATE INDEX IF NOT EXISTS idx_director_scene_affinity_director_machine_id
ON director_scene_affinity(director_machine_id);

CREATE TABLE IF NOT EXISTS committee_template (
  machine_id TEXT PRIMARY KEY,
  模板名 TEXT NOT NULL,
  chief_director_id TEXT,
  default_roles TEXT NOT NULL,
  适用阶段 TEXT NOT NULL,
  使用场景 TEXT NOT NULL,
  决策规则 TEXT NOT NULL,
  输出格式 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS committee_handoff_rule (
  machine_id TEXT PRIMARY KEY,
  from_role TEXT NOT NULL,
  to_role TEXT NOT NULL,
  transition_type TEXT NOT NULL,
  buffer_guidance TEXT NOT NULL,
  continuity_notes TEXT NOT NULL,
  before_cut_pattern TEXT NOT NULL,
  after_cut_pattern TEXT NOT NULL,
  buffer_cut_pattern TEXT NOT NULL,
  applicable_director_pairs TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS committee_style_merge_rule (
  machine_id TEXT PRIMARY KEY,
  role_code TEXT NOT NULL,
  precedence_order INTEGER NOT NULL,
  overridable_fields TEXT NOT NULL,
  non_overridable_fields TEXT NOT NULL,
  merge_note TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS visual_language_term (
  machine_id TEXT PRIMARY KEY,
  术语 TEXT NOT NULL,
  分类 TEXT NOT NULL,
  prompt_token TEXT NOT NULL,
  定义 TEXT NOT NULL,
  usage_rule TEXT NOT NULL,
  example_usage TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS camera_term (
  machine_id TEXT PRIMARY KEY,
  术语 TEXT NOT NULL,
  prompt_token TEXT NOT NULL,
  aliases TEXT NOT NULL,
  定义 TEXT NOT NULL,
  usage_rule TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS continuity_rule (
  machine_id TEXT PRIMARY KEY,
  规则 TEXT NOT NULL,
  触发条件 TEXT NOT NULL,
  判定 TEXT NOT NULL,
  示例 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transition_term (
  machine_id TEXT PRIMARY KEY,
  术语 TEXT NOT NULL,
  prompt_token TEXT NOT NULL,
  定义 TEXT NOT NULL,
  使用规则 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS story_structure_template (
  machine_id TEXT PRIMARY KEY,
  duration_band TEXT NOT NULL,
  structure_name TEXT NOT NULL,
  beat_count_range TEXT NOT NULL,
  hook_rule TEXT NOT NULL,
  turn_rule TEXT NOT NULL,
  ending_rule TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS manga_structure_rule (
  machine_id TEXT PRIMARY KEY,
  规则名 TEXT NOT NULL,
  规则说明 TEXT NOT NULL,
  执行提示 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS character_arc_pattern (
  machine_id TEXT PRIMARY KEY,
  模式名 TEXT NOT NULL,
  说明 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dialogue_style_rule (
  machine_id TEXT PRIMARY KEY,
  风格名 TEXT NOT NULL,
  适用场景 TEXT NOT NULL,
  规则说明 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS prompt_template (
  machine_id TEXT PRIMARY KEY,
  stage TEXT NOT NULL,
  name TEXT NOT NULL,
  模板名 TEXT NOT NULL,
  适用对象 TEXT NOT NULL,
  required_inputs TEXT NOT NULL,
  输入字段 TEXT NOT NULL,
  body TEXT NOT NULL,
  模板正文 TEXT NOT NULL,
  expected_output_schema TEXT NOT NULL,
  输出要求 TEXT NOT NULL,
  target_model_family TEXT NOT NULL,
  is_structured_output INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS export_template (
  machine_id TEXT PRIMARY KEY,
  sheet_name TEXT NOT NULL,
  显示标题 TEXT NOT NULL,
  用途 TEXT NOT NULL,
  核心字段 TEXT NOT NULL,
  列定义 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS classic_case_example (
  machine_id TEXT PRIMARY KEY,
  案例名 TEXT NOT NULL,
  案例类型 TEXT NOT NULL,
  目标时长 TEXT NOT NULL,
  推荐委员会 TEXT NOT NULL,
  推荐导演组合 TEXT NOT NULL,
  场景概要 TEXT NOT NULL,
  结构提示 TEXT NOT NULL,
  验证价值 TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS failure_pattern (
  machine_id TEXT PRIMARY KEY,
  failure_code TEXT NOT NULL,
  failure_name TEXT NOT NULL,
  failure_category TEXT NOT NULL,
  symptom TEXT NOT NULL,
  common_causes TEXT NOT NULL,
  detection_hint TEXT NOT NULL,
  repair_strategy TEXT NOT NULL,
  affected_layers TEXT NOT NULL,
  validator_hint TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_notes TEXT NOT NULL,
  confidence_level TEXT NOT NULL,
  last_reviewed_at TEXT NOT NULL
);
