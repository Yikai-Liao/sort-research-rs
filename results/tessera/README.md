# Tessera 实验记录

各轮测试按比较对象和源码版本独立保存。

| 记录 | 内容 | 测试范围 |
| --- | --- | --- |
| [安全修复与性能验收](safety-20261105/README.md) | 可 Copy／不可 Copy 记录与不确定 key 的内存安全修复；配对性能矩阵、失败候选与最终安全验证。 | release、ASan、Miri；Tessera 与独立 MSD 的 45 工况配对计时。 |
| [外部项目 benchmark：StarRocks 与 ClickHouse](public-projects-20261105/README.md) | StarRocks PR 原 Google Benchmark 用例；ClickHouse 仓库自带 radix_sort 示例测试；加入当前 Tessera 对照。 | 同一台机器上的外部工作负载筛选。 |
| [清理与冻结版本验证](cleanup/README.md) | 正式冻结版相对 `2ef2415` 的配对性能、安全验证和历史 Block 对照。 | 使用 Tessera 自身的 `radix_ab` harness。 |
| [早期逐轮历史](history.csv) | `38bd631`、`6640ab3`、`bad4f09` 的早期整数排序测量。 | 历史候选数据。 |
