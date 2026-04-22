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

CREATE TABLE IF NOT EXISTS golden_sample_library (
  machine_id TEXT PRIMARY KEY,
  sample_id TEXT NOT NULL,
  schema_version TEXT NOT NULL,
  source_fields TEXT NOT NULL,
  provenance TEXT NOT NULL,
  classification TEXT NOT NULL,
  fewshot TEXT NOT NULL,
  validator_evidence TEXT NOT NULL,
  negative_sample TEXT NOT NULL,
  v3_core_coverage TEXT NOT NULL,
  repair_mapping_planning TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_golden_sample_library_sample_id
ON golden_sample_library(sample_id);

CREATE TABLE IF NOT EXISTS golden_sample_field_coverage_rule (
  rule_id TEXT PRIMARY KEY,
  core TEXT NOT NULL,
  row_count INTEGER NOT NULL,
  source_sample_ids TEXT NOT NULL,
  required_source_fields TEXT NOT NULL,
  validator_evidence_fields TEXT NOT NULL,
  fewshot_gate TEXT NOT NULL,
  negative_sample_gate TEXT NOT NULL,
  coverage_summary TEXT NOT NULL,
  future_gate_owner TEXT NOT NULL,
  planning_only INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_golden_sample_field_coverage_core
ON golden_sample_field_coverage_rule(core);

CREATE TABLE IF NOT EXISTS golden_sample_failure_mapping (
  mapping_id TEXT PRIMARY KEY,
  sample_id TEXT NOT NULL,
  core TEXT NOT NULL,
  tier TEXT NOT NULL,
  usable_for_fewshot TEXT NOT NULL,
  negative_sample_signal INTEGER NOT NULL,
  planned_failure_codes TEXT NOT NULL,
  validator_evidence TEXT NOT NULL,
  source_field_refs TEXT NOT NULL,
  FOREIGN KEY (sample_id) REFERENCES golden_sample_library(sample_id)
);

CREATE INDEX IF NOT EXISTS idx_golden_sample_failure_mapping_sample_id
ON golden_sample_failure_mapping(sample_id);

CREATE TABLE IF NOT EXISTS golden_sample_repair_mapping (
  mapping_id TEXT PRIMARY KEY,
  sample_id TEXT NOT NULL,
  core TEXT NOT NULL,
  repair_planning_mode TEXT NOT NULL,
  linked_failure_mapping_id TEXT NOT NULL,
  planned_repair_inputs TEXT NOT NULL,
  future_repair_gate TEXT NOT NULL,
  planning_only INTEGER NOT NULL,
  FOREIGN KEY (sample_id) REFERENCES golden_sample_library(sample_id),
  FOREIGN KEY (linked_failure_mapping_id) REFERENCES golden_sample_failure_mapping(mapping_id)
);

CREATE INDEX IF NOT EXISTS idx_golden_sample_repair_mapping_sample_id
ON golden_sample_repair_mapping(sample_id);

CREATE TABLE IF NOT EXISTS golden_sample_source (
  source_id TEXT PRIMARY KEY,
  source_type TEXT NOT NULL,
  label TEXT NOT NULL,
  path TEXT NOT NULL,
  sha256 TEXT,
  applies_to TEXT NOT NULL,
  notes TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS golden_sample_provenance (
  provenance_id TEXT PRIMARY KEY,
  source_ids TEXT NOT NULL,
  generated_files TEXT NOT NULL,
  preservation_contract TEXT NOT NULL
);
