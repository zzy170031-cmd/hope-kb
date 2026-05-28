# Hope-KB 知识库

Hope-KB 是 Hope AI 动漫化 PWA 的产品知识调度中枢。

它不直接生成用户最终文本，也不替代 PWA 或模型服务。它负责把经过来源核验、用户确认和安全处理的知识，整理成 PWA 可以稳定消费的 `summary-only` 知识快照，辅助 Hope PWA 从用户文本生成专业的动漫化剧本、分镜字段和分镜提示词。

## 产品定位

Hope PWA 的目标是把用户提供的简短文本、故事梗概或剧本片段，转成可用于 AI 动漫化生产的结构化结果：

- 扩写故事或改写剧本
- 创建分镜任务
- 生成分镜字段
- 编译分镜提示词
- 校验字段边界与导出安全
- 输出可交付的分镜提示词表格

Hope-KB 在这条链路中负责知识侧能力：

```text
知识图谱联网搜索
-> 候选知识与来源证据
-> 用户确认
-> KB reviewed_wiki 入库
-> wiki-to-runtime mapping
-> runtime snapshot
-> summary-only PWA adapter
-> PWA 生成 / 校验 / 导出
```

## 当前知识覆盖

当前 KB 已围绕 AI 动漫化产品链路沉淀多类规则包，包括：

- 文本意图与故事任务路由
- 人物身份锁定与角色关系保真
- 角色视觉连续性
- 场景、空间、道具连续性
- 写作扩写与可见动作化
- 导演调度、运镜、景别与镜头目的
- 分镜拆分、单镜头保留与时长密度
- 光影、材质、空气感与色彩连续性
- 动画表演、关键姿态、动作预备与跟随
- 对白 / 旁白依据锁与分配规则
- 分镜提示词编译边界
- 字段级失败修复与导出安全
- raw KB、来源证据、内部规则和密钥泄漏阻断

这些知识不会以原始资料或长 prompt 模板直接下发给 PWA。PWA 只消费安全摘要、规则指令和字段约束。

## PWA 字段边界

Hope PWA 的最终分镜字段固定为：

- 人物
- 运镜
- 景别
- 画面描述
- 角色动作
- 对白/旁白
- 分镜提示词
- 时长(秒)

`序号` 由 PWA 控制，`操作` 是界面控制列。`note`、`status`、来源证据、规则编号、hash、内部调试信息不得进入最终用户字段或导出内容。

## 知识入库原则

每轮知识必须先经过以下流程：

```text
下发需求
-> 联网搜索
-> 候选来源证据
-> 用户在 5179 页面确认
-> KB 去重与入库
-> runtime snapshot
-> PWA adapter
-> PWA 同步与验证
```

没有可核验来源的候选，不能进入 KB 去重确认包。来源证据只允许停留在图谱候选页、intake package、dedupe report 和 KB 审核层，不得进入 runtime、PWA prompt、UI 最终字段或导出文件。

## 安全边界

禁止下放到 PWA 或导出的内容：

- raw source
- source_register
- source_refs
- prompt_body
- OCR 原文
- sample id
- rule id
- hash
- 本地路径
- API key、token、secret
- 真实导演、工作室、电影、游戏、IP 专名作为 runtime 风格开关
- 长 prompt 万能模板
- 自动编造对白模板

KB 的职责是把知识转成产品字段约束，而不是把资料堆进提示词。

## 主要目录

- `knowledge/reviewed_wiki/`：已审核知识条目
- `knowledge/intake_runs/`：每轮候选、确认包和去重报告
- `knowledge/mappings/`：reviewed_wiki 到 runtime rule pack 的映射
- `knowledge/runtime_snapshots/`：PWA runtime 使用的候选快照
- `samples/pwa-kb-adapter-output.sample.json`：PWA 消费的 summary-only adapter 样例
- `samples/runtime-kb-snapshot.sample.json`：runtime snapshot 样例
- `scripts/`：候选验证、入库、adapter 构建和同步脚本
- `web/kb-flow-dashboard/`：5179 知识图谱与确认流程页面
- `docs/`：治理、契约、进度和产品链路文档

## 常用验证

```powershell
node scripts\validate-manual-intake-package.js --package <path-to-intake-package.json>
node scripts\apply-confirmed-intake-package.js --package <path-to-confirmed-package.json>
node scripts\build-pwa-kb-adapter-output.js --check
powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\validate-kb-runtime-prototype.ps1
git diff --check
```

## 与 Hope PWA 的关系

Hope-KB 只提供知识快照和规则约束。PWA 仍然负责：

- 用户输入
- provider / API 配置
- 千问文本生成
- 分镜任务创建
- 字段生成与编辑
- prompt_text 编译
- validator / export
- 桌面便携包交付

KB 与 PWA 的分工边界是 Hope 产品稳定性的核心：KB 负责可靠知识，PWA 负责用户可见产出。
