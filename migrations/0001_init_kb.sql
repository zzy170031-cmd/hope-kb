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

CREATE TABLE IF NOT EXISTS director_profile (
  machine_id TEXT PRIMARY KEY,
  导演名 TEXT NOT NULL,
  风格一句话 TEXT NOT NULL,
  视觉关键词 TEXT NOT NULL,
  叙事关键词 TEXT NOT NULL,
  适用场景 TEXT NOT NULL,
  禁用项 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS director_cut_sample (
  machine_id TEXT PRIMARY KEY,
  director_machine_id TEXT NOT NULL,
  cut标题 TEXT NOT NULL,
  场景 TEXT NOT NULL,
  镜头 TEXT NOT NULL,
  光线 TEXT NOT NULL,
  动作 TEXT NOT NULL,
  情绪 TEXT NOT NULL,
  提示词片段 TEXT NOT NULL,
  FOREIGN KEY (director_machine_id) REFERENCES director_profile(machine_id)
);

CREATE INDEX IF NOT EXISTS idx_director_cut_sample_director_machine_id
ON director_cut_sample(director_machine_id);

CREATE TABLE IF NOT EXISTS committee_template (
  machine_id TEXT PRIMARY KEY,
  模板名 TEXT NOT NULL,
  适用阶段 TEXT NOT NULL,
  参与角色 TEXT NOT NULL,
  决策规则 TEXT NOT NULL,
  输出格式 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS visual_language_term (
  machine_id TEXT PRIMARY KEY,
  术语 TEXT NOT NULL,
  分类 TEXT NOT NULL,
  定义 TEXT NOT NULL,
  使用提示 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS camera_term (
  machine_id TEXT PRIMARY KEY,
  术语 TEXT NOT NULL,
  分类 TEXT NOT NULL,
  定义 TEXT NOT NULL,
  使用提示 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS continuity_rule (
  machine_id TEXT PRIMARY KEY,
  规则 TEXT NOT NULL,
  触发条件 TEXT NOT NULL,
  判定 TEXT NOT NULL,
  示例 TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS prompt_template (
  machine_id TEXT PRIMARY KEY,
  模板名 TEXT NOT NULL,
  适用对象 TEXT NOT NULL,
  输入字段 TEXT NOT NULL,
  模板正文 TEXT NOT NULL,
  输出要求 TEXT NOT NULL
);
