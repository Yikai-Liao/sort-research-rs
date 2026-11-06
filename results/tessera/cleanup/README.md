# Tessera 清理：最终实现与验证

基线为 `2ef2415`。Probe/try_fold 最终版已冻结，engine SHA-256 为 `e81e08fba9c49830a2a56d84567bb57add328575bbcc5330619537045ef58de4`；完整性能矩阵、补测和安全验证已完成。源码提交、构建和 ELF 信息及本地证据归档路径与哈希见 [provenance.json](provenance.json)；历史候选、逐轮数据和取舍见[筛选账本](archive/README.md)。

## 实现取舍

清理删除实验用 `Policy`、叶子消融入口、环境调参与未采用的实验分支，保留稳定的 MSD、LSD、插入、等值和离群 run 路径。`Digit { shift, width }` 描述各轮窗口；递归与共享 block 接口使用 `u128` 变化位 mask。digit 分区在 compact 时处理能放进 tile 的桶，等值分区在 compact 后处理两侧。采样众数通过 `chunk_by` 选择最大等值组。

预算插入共用一个核：窗口扫描有序段，对已比较的首个逆序对直接交换，再沿可变前缀反向插入。根节点以键副本估价，估价后释放该缓冲再处理记录。新候选的 Probe 用已证明边界的 unchecked 索引，保持采样点；顺序确认通过首个不等键确定方向，再以 `try_fold` 检查该方向。

block 在 key panic 时由桶的已写数和完整输入块记录数重建块内偏移，恢复完整记录集；没有 `Drop` 的记录也可能持有独占引用。tile 的 `RestoreTile` 用 `source` 指向上一整趟成功后的完整记录集，必要时复制回 tile。公开 `Unsigned` 保留基线的 `Shl<Self>` / `Shr<Self>` 约束，内部移位量转为键类型。

## 代码规模和审查

lizard 1.25.0 对比基线与正式冻结源码中 `#[cfg(test)]` 之前的生产代码。NLOC 排除空行和注释；函数拆分会影响平均圈复杂度（CCN）。

| 文件 | NLOC 基线 → 冻结版 | 函数数 基线 → 冻结版 | 平均 CCN 基线 → 冻结版 | 最大 CCN 基线 → 冻结版 |
| --- | ---: | ---: | ---: | ---: |
| Tessera engine | 894 → 627 | 41 → 28 | 4.7 → 4.5 | 22 → 18 |
| 共享 block core | 643 → 593 | 29 → 24 | 3.4 → 3.5 | 17 → 15 |

两文件合计 1537 → 1220 NLOC。冻结版 engine SHA-256 为 `e81e08fba9c49830a2a56d84567bb57add328575bbcc5330619537045ef58de4`，core 为 `f2fafad8d1a7b0e859fa53c207e9068da8154c60e83f38c07df2ee0cfbd83b96`；[complexity.csv](complexity.csv) 保存逐函数数据。这两个生产文件中有 7 个函数的 CCN 大于 6；最终独立审查接受其完整职责，最终审查提出的两处注释已修正，执行代码未变。

## 最终性能对照

同一 harness 下以 `2ef2415` 为基线：A1、A2 是同一基线 ELF 的两次运行，B 为最终候选。AB 是每周期 B 中位数除以两端 A 中位数的平均值，再取五周期中位数；AA 为 A2/A1 的五周期中位数。完整矩阵覆盖 162 cell（Tessera 136、Block 26），共 7290 样本、195 次调用；自动回退筛选及 17 个强制对照组成 28 cell 长复查，共 80520 样本、420 次调用，每 arm 7–1975 轮。数据和条件见 [完整汇总](final-summary.csv)、[逐轮数据](final-rounds.csv)、[配置](final-manifest.json)，以及[复查汇总](recheck-summary.csv)、[逐轮数据](recheck-rounds.csv)、[配置](recheck-manifest.json)。

u64 sparse8 100k 长复查 AB 为 1.05008，五周期均大于 1，AA 为 1.04995。AA 不能用于直接抵消 AB；以 B / max(A1,A2) 逐周期计算，中位数为 1.03613，4/5 周期慢于两端。随后增加 15 个随机顺序 block、每 arm 201 轮的六臂专项对照，共 18090 样本、90 次调用：最终/基线为 1.01520（10/15 block 大于 1）、基线 AA 为 1.01341、最终 AA 为 1.00036，最终/typed 为 0.98980。长复查的 5% 与旧 typed 的 6.3% 幅度在本次随机对照中未重现；仍有偏慢信号，无法据此认定性能等同或全部归于噪声，根因未确证。随机数据的 arm 与汇总别名见[映射](archive/sparse-final-randomized-labels.json)。整进程性能计数涵盖克隆、参考排序和输出等工作；最终/基线的 task-clock、cycles、instructions 中位比分别为 1.02273、1.02228、1.00136。

长复查中，record128 zipf13 100k 为 1.04421、AA 0.99541，周期波动较大。Block u128 all_equal 的完整矩阵约慢 8.18% 未在长复查重现，后者为 0.97971 且五周期均小于 1。降序 u32、u64、record16 的 AB 分别为 0.78342、0.78235、0.86938；强制近序 cell 中最大为 u64 10M 的 0.95821。Block random_wide 1M 的 u32 为 0.83021、u64 为 1.01967，后者仅 3/5 周期大于 1。

## 安全验证

正式冻结版 release 通过 503 个注册测试函数（lib 21、两个功能 suite 各 233、owned 3、views 13），其中不支持的 capability suite 会提前返回；ASan 通过 35 项（lib 19、owned 3、views 13）；Miri 通过 15 项（views 13、两个 tile panic 恢复各 1）。六条命令退出码均为 0；源码、命令和日志哈希见 [validation.json](validation.json)。测试固定 CPU 4，使用 nightly `rustc 1.101.0-nightly (a30aa9064 2026-10-08)`。性能测试固定 CPU 2。

采样与 scatter 索引筛选和实际 ELF 的 LLVM-MCA 结果见[热点审计](archive/index-audit.md)。前一版 typed 的完整配对和安全验证见[历史账本](archive/README.md)。

## 历史 Block 对照

另用补充 ELF `819fba5d…`，在同一新 harness 中加入 `429f6f7` 的初始 Block 与 typed 版 Block，比较 44 cell、5 周期，共 27825 条样本、660 次调用。AB 是 typed Block / 初始 Block，A1 和 A2 都是同一 ELF 内的初始 Block；轮数规则、源码哈希及命令见[配置](original-block-manifest.json)，完整数据见[汇总](original-block-summary.csv)和[轮次](original-block-rounds.csv)。此对照使用 typed 版 `787fb718…`，不包含新 Probe 候选。补充 crate 使用独立锁文件，依赖版本与主 benchmark ELF 不同；两个算法在该 ELF 内共享输入与依赖。

random_wide 1M 的 AB：u32 0.9477、u64 1.0600、u128 1.0021、record16 1.0018、record128 1.0255。以下 100k cell 的五周期 AB 均大于 1，表中 AA 显示基线自身波动；这些结果表明相对最初 Block 仍有较慢的场景。

| 类型与形状 | AB | AA |
| --- | ---: | ---: |
| u32 random_wide | 1.1023 | 0.9935 |
| u32 sparse8 | 1.0446 | 0.9993 |
| u32 ascending | 1.0226 | 1.0091 |
| u128 zipf13 | 1.0975 | 0.9035 |
| u128 random_sorted99 | 1.0609 | 1.0188 |
| record16 random_wide | 1.0677 | 0.9848 |
| record128 random_wide | 1.0412 | 1.0140 |

## 复现命令

各行独立执行：

```sh
taskset -c 4 cargo test --release --features rust_tessera_radix,rust_msd_radix --lib
taskset -c 4 cargo test --release --features rust_tessera_radix,rust_msd_radix --test block_lsd_functionality --test tessera_functionality --test radix_views --test radix_owned
CARGO_TARGET_DIR=logs/tessera-cleanup/asan-target RUSTFLAGS='-Zsanitizer=address' taskset -c 4 cargo test --release --target x86_64-unknown-linux-gnu --features rust_tessera_radix --lib --test radix_views --test radix_owned
taskset -c 4 cargo +nightly miri test --features rust_tessera_radix --test radix_views
taskset -c 4 cargo +nightly miri test --features rust_tessera_radix --lib other::rust_tessera_radix::engine::tests::panic_preserves_records
taskset -c 4 cargo +nightly miri test --features rust_tessera_radix --lib other::rust_tessera_radix::engine::tests::split_panic_recovers
```

性能二进制以 `cargo bench --features rust_tessera_radix --bench radix_ab --no-run` 构建，使用默认 bench profile、thin LTO 和普通 target，未设置 `RUSTFLAGS`。

## 既有静态风险

Tessera 的 `counting_pass` 在 `Key` 违反确定性契约时可能留下未初始化输出槽；该基线风险尚未复现。独立的 MSD `permute` 在 `ptr::read` 取出拥有资源的记录后若 `Key` panic，缺少恢复保护；该路径不属于 Tessera。
