# Block LSD 收尾记录

代码仓库是 `/root/code/sort-research-rs`，fork 为
`Yikai-Liao/sort-research-rs`。本轮在独立 worktree 中工作，主工作区的
`exp/radix-opt` 分支及未提交的 Tessera 修改保持原样。

## 本轮交付

代码候选为 `22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7`，父提交为
`429f6f775bb07fdf1a6921b523f5442d47dab167`，分支仍是 `feat/block-lsd`。

- 取消两处 `needs_drop` 条件，所有记录都更新搬运偏移，并在输入尚未恢复时执行 compact。
  无析构类型也可能包含 `&mut`，因此不能用是否需要析构判断恢复是否必要。
- `radix_views` 在销毁数据之前检查实际恢复后的 ID 和 payload；析构计数按原始 ID 检查，
  每个 ID 都必须恰好一次，避免总次数相同却有记录丢失/重复。
- 新增 `panic_preserves_exclusive_borrows_without_drop`：无 Drop、含 `&mut u64` 的记录，
  包含 100 条记录的报告反例和 17 条记录的小输入；预扫描后第二次 scatter 键提取触发
  panic，检查原 panic payload、完整 ID 集合、键与 payload 对应关系，并通过每个独占引用读写。
- 修正键契约、README 和预扫描注释；移除无依据的最终性能概括。
- 保留独立的 `70557f3` ipnsort nightly 兼容提交及现有历史，未 squash 或改写已发布提交。
  supports_type / supports_sort_by 和内存 benchmark 继续留在同一代码系列中。

附件分支 `block-lsd-artifacts` 追加收尾提交；它保留原孤儿根提交 `c306e53`，与代码分支隔离。
历史根提交有 136 个受跟踪文件，文件内容合计 1,611,160 字节；不是 318 个文件。
本轮新增提交会改变当前文件数，该统计仅描述历史根提交。

## 实验材料处理

`results/block_lsd.json` 的 106 项原始导出保持字节不变。全部文本汇总、HTML 和 PNG
从该 JSON 重新生成；median 与 mean 分列，文案和图上都标明历史结果，不是修复后的数字。
新增 `results/provenance.json` 记录源文件 SHA-256、修复提交号及逐项来源分类。
再生成得到 98 份 HTML 和 3 张 PNG；比原包多 7 份此前遗漏的 HTML。
`check_saved.py` 逐点确认 HTML 内 106 个 single_size、106 个 scaling、204 个 direct_versus
数据点与 JSON median 或其派生比值对应。源 JSON SHA-256 为
`78abf9088f7f35990f35ad6070dd07895d27068f959e08bfa7e5c437f523019c`。

82 项 random 的 JSON mean/slope 及置信区间与干净日志的舍入统计量对应。日志没有原始
samples，不能仅靠日志重算 median；median 的来源是 JSON。cold u64 10M 的 block LSD
和 radsort 两行源路径已改为 `logs/bench3.log`，另外两行仍是 `bench4.log`。

24 项 ascending / descending / random_p5 仅保留为已保存 baseline，原始测量日志未找回。
`pattern-bench.log` 是相对于 baseline 的后续复跑，不能冒充原始日志。上游回复与 PR
草稿不再列这些 pattern 数字；完整归档及图仍保留，并标明来源限制。

各轮测量没有记录精确源码提交，不能把此前分支头 `429f6f7` 当成所有结果的已证实来源。
`c-comparison/` 也保留为历史材料，不能证明修复后与 C/C++ 的最终性能关系。

## 已修正文案

有序 u64 使用 `(i + 2^31) * (2^31 - 1)` 转换，100k/1M 的八个字节均变化，
不能解释为从八趟减到三趟。只有前 1025 个键让每一位都变化时预扫描才能提前结束；
本仓库转换后的 u64 有恒定高位，需要全扫描。

cold 路径扰乱 CPU 预测状态，并不刻意测量主存加载；hot 也经批次 setup 生成输入。
固定 512 的只是 scratch 大小，块排列元数据仍随 n 增长；自适应策略才使两项一起为
O(sqrt(n))。vendor 已获维护者支持，不再作为待确认问题。

旧 random 结果中，大多数中大规模测点快于 radsort；hot u64 100k 为 2.35 对 2.34 ms，
接近持平。128B 记录在 1M 相对 std 的 cold/hot 为约 0.81x/0.92x，100k 接近持平；
约 0.38x 属于 f64。10k 的胜负取决于类型，10k 以下没有耗时数据，不能概括为都输给比较排序。

## 验证边界及后续

按用户要求，本轮不编译、不运行排序代码、测试、Miri、ASan 或 benchmark。
本轮做了 Rust 格式化、源码/差异检查、日志与 JSON 统计量比对，以及保存数据的图表再生成。
用户提供的审查报告记载了旧代码的 Miri UB 和临时修复通过；本轮没有重新复现。

历史 release、ASan 和 Miri 通过记录都先于这次回归修正，不能称为当前候选已通过。
Block LSD 标准套件注册 233 项，79 项进入测试主体，154 项按能力声明或既有规则提前返回。
新增回归后，其他相关测试二进制预期为 8 + 2 + 7 项；预期计数不是执行结果。

发布上游回复/正式 PR 前仍需：

1. 在修复候选上运行 release 回归，明确区分注册数、进入主体数及跳过数。
2. 针对新回归运行 Miri，再检查既有 ownership 测试；按需重跑 ASan。
3. 仅重测准备引用的少量代表性结果，记录源码提交、环境、原始日志与导出 JSON。

本轮只向 fork 推送代码和附件的新提交。issue 回复及 PR body 保留为草稿，未发布消息、未开 PR。
草稿位置为 `drafts/issue-reply.md` 和 `drafts/pr-body.md`。后续无需继续扩大优化研究。
