# snapshots

This directory stores generated read-only snapshot artifacts for `hope-kb`.

v0.1 约定：

- snapshot 由 seed bundle + migrations 生成
- snapshot 不手写编辑
- snapshot 的来源必须能追溯到：
  - `seed/v0.1/manifest.json`
  - `seed/v0.1/import_map.json`
  - `migrations/0001_init_kb.sql`

当前目录可以为空；当真正生成 SQLite snapshot 时，再把产物写在这里。
