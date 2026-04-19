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

当前推荐产物名：

- `hope-kb-v0.1.sqlite3`

推荐生成方式：

```powershell
python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb
```

说明：

- snapshot 产物不手写
- snapshot 由 builder 重新生成
- `.sqlite3` 文件默认不纳入版本控制
