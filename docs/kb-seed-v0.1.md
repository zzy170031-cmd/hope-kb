# hope-kb v0.1 Seed Pack

## 内容清单

- 7 个导演 profile
- 7 组导演核心内核
- 7 条导演规则
- 7 组导演场景适配
- 7 条代表 cut 样例
- 7 条角色定义
- 4 套 committee 模板
- 8 条 handoff 规则
- 7 条风格合并规则
- 20 条视觉语言术语
- 15 条中文摄影术语
- 3 条连续性规则
- 8 条转场术语
- 4 条故事结构模板
- 5 条漫剧结构规则
- 6 条角色弧线模式
- 5 条对白风格规则
- 10 条 prompt templates
- 6 条导出模板
- 6 条经典案例示例
- 1 份 snapshot manifest

## 示例记录

### 导演 profile

- `director_01`：`今石洋之`
- `director_02`：`荒木哲郎`
- `director_03`：`朴性厚`
- `director_04`：`新海诚`
- `director_05`：`山田尚子`
- `director_06`：`汤浅政明`
- `director_07`：`今敏`

### 代表 cut

- `cut_01`：高楼坠落前的蓄力
- `cut_02`：墙内通道的群像压迫
- `cut_03`：近身闪避后的反击
- `cut_04`：雨后天桥上的回望
- `cut_05`：桌下轻碰的脚尖
- `cut_06`：走廊突然扭曲成奔流
- `cut_07`：镜中人先转头

### committee 模板

- `committee_01`：动作冒险组
- `committee_02`：日常治愈组
- `committee_03`：悬疑心理组
- `committee_04`：史诗群像组

### 导演核心内核

- 今石洋之：爆发、透视、能量释放
- 荒木哲郎：群像、压迫、危机推进
- 朴性厚：近身、打击、动作轴线
- 新海诚：景观、光线、情绪托举
- 山田尚子：低视点、微表演、安静留白
- 汤浅政明：意识流、变形、情绪外化
- 今敏：错位、镜像、匹配剪接

### 视觉语言术语

- `visual_01`：高反差
- `visual_02`：低饱和
- `visual_03`：逆光轮廓
- `visual_04`：前景遮挡

### 中文摄影术语

- `camera_01`：大全景
- `camera_02`：远景
- `camera_03`：中景
- `camera_04`：近景

### 连续性规则

- `cont_01`：人物服装连续
- `cont_02`：空间方位连续
- `cont_03`：时间光线连续

### Prompt templates

- `prompt_01`：梗概 -> Story
- `prompt_02`：Story -> Outline / Beats
- `prompt_03`：Story -> Screenplay
- `prompt_04`：Screenplay -> RenderSegments
- `prompt_05`：RenderSegment -> Cuts
- `prompt_06`：Cut -> Layout Prompt
- `prompt_07`：Layout -> Render Prompt
- `prompt_08`：Dialogue Refine
- `prompt_09`：Repair Pass
- `prompt_10`：Export Summary

### 经典案例示例

- `case_01`：45 秒追击钩子短片
- `case_02`：10 分钟初遇单集
- `case_03`：60 分钟多集群像项目
- `case_04`：镜中错位悬疑段
- `case_05`：桌下心动微表演段
- `case_06`：近身反击动作段

## 中文 token 字段完整性检查

已满足 v0.1 最小基线：

- 内容字段默认中文
- 不带语言后缀
- 通过 `machine_id` 保持稳定引用
- snapshot 元数据已冻结
- content hash 字段已冻结
- seed import format 字段已冻结

## Day 2-3 待补内容

- director profile 的更细粒度代表作扩写
- committee 模板的更复杂 role 覆盖
- 视觉语言术语的同义词映射
- 摄影术语与镜头尺寸的更细分映射
- 连续性规则的更完整失败样例
- prompt template 的更多导演差异化版本
- director_scene_affinity 的更细粒度场景类型扩写
- handoff 规则的更多跨风格例子
- 真实 benchmark 样例的补充校验

## Import rule

- 以 `manifest.json` 作为入口
- 以 `machine_id` 作为稳定主键
- 以中文主字段作为内容展示层
- 新内容只追加，不重排既有 `machine_id`
