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
