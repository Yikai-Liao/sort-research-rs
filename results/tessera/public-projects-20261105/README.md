# 外部项目 benchmark：StarRocks 与 ClickHouse

2026-11-05，测试 Tessera `6804fcb` 在两个外部项目原有排序负载上的表现。测试入口、数据生成和计时来自上游，Tessera 通过 Rust 静态库接入 C++。

StarRocks 的结果支持选择性接入：Int64 均匀随机键和百万行批次值得推进；4K–64K 行 Int32、Float 随机键保留现有 PR 实现。ClickHouse UInt64 原示例中，65536、100 万和 1000 万行的配对 Tessera/LSD 比值分别为 0.553、0.267 和 0.348，均有 9/9 个 block 更快；4096 行结果接近且波动较大。完整数值见 [结果表](tables.md)。

## 比较对象和环境

| 项目 | 固定版本 | 测试入口 |
| --- | --- | --- |
| Tessera | 本地 `6804fcb`，engine `e81e08fb…`、block core `f2fafad8…` | 原 engine/core 源码快照；C ABI 导出 4 种 StarRocks 记录和 UInt64 键入口 |
| StarRocks | PR #81472，`890b8fffcd639c2cd70b1acbab799794a71d4c62` | [`be/src/bench/radix_sort_bench.cpp`](https://github.com/StarRocks/starrocks/blob/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/bench/radix_sort_bench.cpp) |
| ClickHouse | `c8c7411269eb045ef70cf6e90dbc997d60dfa76f` | [`src/Common/examples/radix_sort.cpp`](https://github.com/ClickHouse/ClickHouse/blob/c8c7411269eb045ef70cf6e90dbc997d60dfa76f/src/Common/examples/radix_sort.cpp) |

机器为 KVM 上的 Intel Xeon Gold 6140，所有计时固定 CPU 2、单线程串行执行。C++ 使用 Clang 19.1.7、`-O3 -march=native -DNDEBUG`，Rust 使用 `1.101.0-nightly (a30aa9064 2026-10-08)`、`target-cpu=native`、thin LTO。Google Benchmark 为 1.9.4。完整版本、源码哈希和环境见 [manifest.json](manifest.json)。

## StarRocks

保留原 benchmark 的生成器、输入种子 `0x5EED1234`、原排序函数和计时循环，并增加 `BM_Tessera`、`BM_TesseraStable` 和同一 PR 函数的 `AA` 注册。

- 类型：Int32、Int64、Float、Double；记录均为原 `InlinePermuteItem<T> { key, uint32_t index }` 布局。
- 分布：UniformRandom、AlreadySorted、ReverseSorted、LowCardinality、NonNegLowCard、NearlySorted、SortedPrefix90、Zipfian。
- 行数：256、1024、4096、65536、1000000。
- 原有对照：pdqsort + tie、PR radix + tie、PR radix 每次分配 scratch，以及低基数稳定排序与行号输出。

Tessera 完成排序后调用 PR 的相同 tie 检测函数；稳定排序用例也执行原 `restore_inline_permutation`。原循环中的输入 `memcpy`、tie 初始化和 Tessera 内部分配都计入时间。PR 的 warm scratch 在循环外预分配，cold scratch 在循环内按需分配。

完整轮覆盖 160 个类型/分布/尺寸组合，另有 20 个低基数稳定排序组合；共 860 个注册用例、每个 5 次测量。关键的随机和低基数组合做了 9 次长测，每个测量段目标为 60 ms。两轮均启用 Google Benchmark 随机交错。

表中 StarRocks 比值为 `median(Tessera CPU ns) / median(PR CPU ns)`，小于 1 表示 Tessera 更快。完整轮 160 个组合中有 69 个比值小于 1；按尺寸分组为 256 行 7/32、1024 行 6/32、4096 行 9/32、65536 行 19/32、100 万行 28/32。这个分布体现了批次大小对取舍的影响。

长测的 A/A 比值范围为 0.956–1.054，中位数为 0.993。4K 和 64K 行 Int32、Float 随机键的回退，以及 Int64 随机键的收益均在长测中保留。

每次启动在计时外校验 480 项组合：160 个输入分别由 PR warm、PR cold 和 Tessera 排序，逐行对照 `std::stable_sort` 的行号顺序，并检查完整 tie 数组。所有校验通过。

## ClickHouse

使用仓库自带 `Common/examples/radix_sort.cpp`，保留原 `pcg64` 生成器、`Stopwatch` 计时、方法分派和排序检查。运行原方法 1（std::sort）、2（LSD radix）、5（pdqsort），新增方法 9（Tessera）。数据为 UInt64 键数组，尺寸为 4096、65536、100 万和 1000 万。

每个尺寸运行 9 个随机顺序 block，每个 block 的种子固定且各算法相同，含两次相同 LSD 基线。统计量为逐 block `Tessera / mean(LSD A1, LSD A2)` 的中位数；A/A 为逐 block `LSD A2 / LSD A1`。原计时段只包含排序。新增的精确参考排序在计时外执行，每次输出与参考数组逐项相等。

ClickHouse 这组测量是 UInt64 原地排序。

## 接入方式

StarRocks 原 benchmark 的项目头文件依赖由 [sr_shim.h](sr_shim.h) 衔接。比较器、SIMD 搜索、raw vector resize 和 radix 函数体保存在原始快照中；日期和字符串类型只提供声明。新增对照集中在 [sr_additions.inc](sr_additions.inc)。

ClickHouse 使用 [ch_shim.h](ch_shim.h) 衔接基础类型、目标 CPU 和分配接口；排序核保留原函数体，allocator 的数据库记账调用替换为 malloc/free。原示例的随机种子改为可固定的环境变量，输出精度增至 9 位，并增加计时外精确校验。

尚未测量 StarRocks 日期和字符串类型、ClickHouse MSD 方法与 `Common/benchmarks/radix_sort.cpp` 的 `getPermutation` 用例，以及数据库 nullable、多列、Top-K、取消流程和内存峰值；`getPermutation` 用例需构建完整 ClickHouse。

## 数据和复现

| 文件 | 内容 |
| --- | --- |
| [starrocks-full.json](starrocks-full.json) | 完整轮 Google Benchmark 原始测量与聚合值 |
| [starrocks-full-summary.csv](starrocks-full-summary.csv) | 160 个组合的 CPU 耗时中位数、PR warm/cold、pdqsort、A/A |
| [starrocks-full-stable.csv](starrocks-full-stable.csv) | 20 个低基数稳定排序组合 |
| [starrocks-confirm.json](starrocks-confirm.json) | 关键组合的 9 次长测原始结果 |
| [starrocks-confirm-summary.csv](starrocks-confirm-summary.csv) | 长测汇总 |
| [starrocks-confirm-stable.csv](starrocks-confirm-stable.csv) | 稳定排序长测汇总 |
| [clickhouse-rounds.csv](clickhouse-rounds.csv) | ClickHouse 每次调用、顺序、种子、计时与退出状态 |
| [clickhouse-outputs.jsonl](clickhouse-outputs.jsonl) | ClickHouse 原 CLI 输出及参考校验结果 |
| [clickhouse-summary.csv](clickhouse-summary.csv) | ClickHouse 配对比值、A/A 和逐方法中位数 |
| [commands.json](commands.json) | 实际构建和运行命令 |
| [vendor/](vendor/) | 固定版本的上游源码和 Tessera 源码快照 |

执行 `build.sh` 重建两个入口，`run_starrocks.sh` 运行 StarRocks 两轮，`python3 run_ch.py` 运行 ClickHouse。`summarize.py` 从原始 Google Benchmark JSON 重建 CSV。各轮数据独立保存，比较时使用各自的分母和统计量。
