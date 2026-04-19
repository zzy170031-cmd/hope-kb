# hope-kb 导入与运行时说明 v0.1

这份文档说明 `hope-kb` 在 v0.1 阶段如何从 seed bundle 进入 SQLite snapshot。

## 1. 当前目标

v0.1 不要求在本仓库里直接完成完整的产品级导入器实现，但要求以下 contract 先冻结：

- seed bundle 有明确 `bundle_order`
- 每类 seed 有明确 `import_map`
- manifest 有真实 `record_counts`
- bundle 有真实 `content_hash`
- 可以独立跑一轮 seed 校验

## 2. 当前导入链组成

当前已经具备：

- `seed/v0.1/manifest.json`
  - 记录 bundle 顺序、hash、record_counts
- `seed/v0.1/import_map.json`
  - 定义 seed 文件到 SQLite 表的映射
- `migrations/0001_init_kb.sql`
  - 定义 v0.1 的目标表结构
- `scripts/validate-seed-bundle.ps1`
  - 校验 seed 文件、计数、machine_id 与 bundle hash
- `scripts/build-kb-snapshot.py`
  - 按 manifest、import_map 和 migration 生成 SQLite snapshot

## 3. 导入器下一步应该做什么

正式导入器落地时，建议严格按以下顺序：

1. 创建空 SQLite 文件
2. 执行 `migrations/0001_init_kb.sql`
3. 读取 `manifest.json`
4. 按 `bundle_order` 逐文件加载
5. 按 `import_map.json` 将数据写入目标表
6. 写入 `snapshot_meta`
7. 跑导入后校验：
   - record_counts 对齐
   - foreign key 合法
   - hash 与 manifest 一致

## 3.1 当前可直接使用的构建命令

当前仓库已经补入最小可用的 snapshot builder，可以直接运行：

```powershell
python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb
```

默认输出：

- `E:\codex\hope-kb\snapshots\hope-kb-v0.1.sqlite3`

builder 的约束：

- 只消费 `seed/v0.1`
- 按 `manifest.json` 和 `import_map.json` 生成
- 自动写入 `snapshot_meta`
- JSON 中的数组/对象按中文 UTF-8 文本序列化后入库
- 不反写 seed 文件

运行前提补充：

- 需要本机存在**真实可用**的 Python 运行时
- 如果当前环境里的 `python` / `python3` 只是 Windows Store alias，占位不可执行，则需要先准备可用解释器
- builder 已经是仓库正式脚本，但是否能在当前机器直接跑通，取决于本机 Python 运行时是否已安装

## 4. v0.1 的约束

- 所有内容字段默认中文
- `machine_id` 是唯一主键
- 任何导入器都不得改写 seed 内容本身
- 任何运行时 overlay 都不应反写系统 snapshot

## 5. 当前状态

就 v0.1 而言，`hope-kb` 已经完成：

- 内容层种子包
- 导入映射层
- 失败模式库
- schema 对齐
- seed 校验脚本

也就是说，当前缺的已经不是“知识库内容定义”，而是后续由产品仓库消费这套 bundle 的装载实现。

而在本仓库内部，v0.1 已经从“只有内容种子包”推进到“内容种子包 + 校验链 + snapshot 构建脚本”。
