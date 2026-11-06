# Tessera 独立仓库计划

- 日期：2026-11-06
- 迁移来源：`Yikai-Liao/sort-research-rs` 分支 `exp/radix-opt`，commit `6c0877f`
- 新仓库：<https://github.com/Yikai-Liao/tessera-sort>（公开，BSD-2-Clause，全英文）
- 状态：P0 已完成（仓库骨架、README、logo、语义草案、设计与 benchmark 文档、CI、贡献规范）；P1 Stable 移植尚未开始。第 10 节列出待定事项。

sort-research-rs 只是借来做实验的仓库。Tessera 的引擎和键编码是自己写的，Block 核心改编自 Robert Clausecker 的 BSD-2-Clause 实现。新仓库只迁移这两部分，测试和 benchmark 重写。

## 1. 目标与范围

**0.1 的交付标准：** 用户可以用 Stable Rust 从 crates.io 安装并调用 Tessera；新仓库自己的 benchmark 能证明，它的排序结果和性能与研究版 `6c0877f` 一致。

| 进入 0.1 | 留到以后 |
|---|---|
| Rust crate `tessera-sort`，Stable，依赖 `std` | C++17 header-only 实现 |
| `stable_sort` 系列接口和 `Workspace` | 真正的不稳定内核 |
| 语义文档、性质测试、与 std 结果对照的测试 | 跨语言 runner 和 C++ 对照算法 |
| Rust 原生 benchmark runner、raw 结果格式和报告脚本 | `no_std + alloc` |
| Miri 与 ASan 验证 | vcpkg / Conan |

Go 实现不列入计划，有真实需求时再评估。

## 2. 名称与许可

| 项目 | 决定 |
|---|---|
| GitHub 仓库 | `Yikai-Liao/tessera-sort` |
| crates.io 包名 | `tessera-sort`（代码中写 `tessera_sort`）。2026-11-06 查询时未被占用；`tessera` 已被一个几何计算包占用 |
| C++ 命名空间（以后） | `tessera` |
| 许可证 | 整个仓库只用一份 **BSD-2-Clause** |

只用 BSD-2-Clause 的原因：`core.rs` 及将来的 C++ 移植是 Clausecker `radixsort_permuted.c` 的派生作品，必须保留他的 BSD-2 版权和许可通知。主协议选 Apache-2.0 或 MIT，仓库都会变成混合协议。

根目录 `LICENSE` 放 BSD-2-Clause 全文，并写两行版权：

```
Copyright (c) 2025, 2026 Robert Clausecker <clausecker@zib.de>
Copyright (c) 2026 Yikai Liao <lyk-boya@outlook.com>
```

每个源文件第一行写 `// SPDX-License-Identifier: BSD-2-Clause`。`core.rs` 保留原有的完整通知和上游 revision 链接。README 里引用论文 [Parallel O(sqrt(n)) Overhead LSD Radix Sort](https://arxiv.org/abs/2607.05302) 和上游 C 实现。

**保持单一协议的约束：**

- 不复制 sort-research-rs 的 `sort_test_tools`、`ffi_types`、patterns 和 bench harness。它们是 Voultapher 的 Apache-2.0 代码。
- 不提交第三方排序源码。Rust 对照算法通过 crates.io 依赖引入；以后的 C++ 对照算法用 CMake `FetchContent` 按固定 commit 拉取。仓库里只放自己写的 adapter。
- StarRocks 和 ClickHouse 集成 benchmark 包含上游代码，不放进本仓库（见 7.10）。

发布前告知 Clausecker，说明这个项目建立在他的 block 算法上。

**贡献规范（已落地）：** DCO 签名（PR 由 CI 逐 commit 检查）；接受 AI 辅助的贡献，要求在 PR 中披露工具、提交者本人读懂并测试、只有人能加 `Signed-off-by`；行为准则以引用方式采用 Contributor Covenant 3.0，不复制原文以保持单一协议；`AGENTS.md` 给编程 agent 的规则。

## 3. 源码迁移清单

| 研究仓库中的文件 | 处理方式 |
|---|---|
| `src/other/rust_tessera_radix/engine.rs` | 迁移为 `engine.rs`，改 import |
| `src/other/rust_block_lsd/core.rs` | 迁移为 `block.rs`，保留 BSD 通知 |
| `src/other/radix_support.rs` | 拆分：`Key`、`Unsigned` 和键编码进 `key.rs`；`RecordType` 的 specialization 用第 6 节方案替代；`radix_adapter!` 和 `RadixDispatch` 不迁移 |
| `tests/radix_safety.rs` | 迁移，改为调用公共接口 |
| `tests/radix_owned.rs`、`tests/radix_views.rs` | 保留用例逻辑，去掉对 `sort_test_tools` 的依赖并重写辅助类型 |
| `tests/tessera_functionality.rs` | 不迁移，由新的性质测试替代 |
| `benches/radix_ab.rs`、`benches/extra_memory.rs` | 不迁移代码，只沿用其方法（交错计时、batch、计时外校验、计数分配器），在新 runner 中重写 |
| `results/tessera/` | 留在研究仓库，新 README 链接到它 |

新仓库从干净的初始提交开始，不用 `git filter-repo` 带历史。第一次提交的说明里写明迁移来源 commit `6c0877f`。

## 4. 仓库结构

```text
tessera-sort/
  LICENSE
  README.md
  Cargo.toml                 # Cargo workspace
  rust/tessera-sort/         # 唯一发布的 crate
    src/lib.rs               # 公共接口与契约
    src/key.rs               # 键编码、RadixKey、Reverse 支持
    src/workspace.rs         # 缓冲容量与调用状态
    src/engine.rs            # Tessera 路由与 tile 排序
    src/block.rs             # 稳定 block 分区与恢复（BSD 通知）
    tests/
  bench/rust/                # benchmark runner，publish = false
  bench/report/              # 报告脚本（Python，只用标准库）
  bench/cases/               # 用例套件定义
  conformance/semantics.md   # 跨语言共享的排序语义
  conformance/vectors/       # 小型测试向量与输入生成器的黄金哈希
  docs/
  .github/workflows/
```

C++ 实现开始时再加 `cpp/` 和 `bench/cpp/`。benchmark 的依赖只出现在 `bench/rust`，crate 用户不会安装到它们。

## 5. 公共接口与语义

### 5.1 Rust 接口

```rust
pub fn stable_sort<K: RadixKey>(values: &mut [K]);
pub fn stable_sort_by_key<T, K: RadixKey, F: FnMut(&T) -> K>(values: &mut [T], key: F);

impl<T> Workspace<T> {   // 另实现 Default
    pub fn new() -> Self;
    pub fn stable_sort(&mut self, values: &mut [T]) where T: RadixKey;
    pub fn stable_sort_by_key<K, F>(&mut self, values: &mut [T], key: F);
    pub fn retained_bytes(&self) -> usize;
    pub fn shrink_to_fit(&mut self);
}
```

- **只有一个 `stable_sort_by_key`，对任何 `T` 每趟存取 u16 桶号**（2026-11-06 定，依据 [`cache-default-20261106`](cache-default-20261106/README.md)）。相对研究版 Copy 快路径的差异在 ±4.5% 以内、方向不一，128 字节记录无差别。key 不一致时仍输出原数据的排列，与 std 排序的保证一致。
- `stable_sort` 的键由库编码、结果确定，scatter 重算键，不存桶号，也不需要首趟写满 scratch。
- 不提供 `_copy`、`_owned`、`_unchecked`。研究版的 `RecordType` specialization 随 Copy 路径一起移除。
- 未直接测量 8 字节 key + 行号记录（`{u32, u32}`）；u32 键数组在 random 16K–1M 上统一缓存慢 3.0–4.4%，P3 runner 完成后补测。
- `RadixKey` 是封闭 trait，首版覆盖 `u8..u128`、`i8..i128`、`f32`、`f64` 和 `Reverse<K>`。
- 不提供任意比较器的 `sort_by`，不支持的键在编译期报错。
- 0.1 不提供 `unstable_sort` 系列（2026-11-06 定）。以后加入是新增函数，不破坏兼容性；语义文档先保留不稳定契约的定义。

### 5.2 语义（写入 `conformance/semantics.md`）

| 项目 | 规则 |
|---|---|
| 稳定性 | 等键记录保持输入顺序 |
| 键提取 | 可能调用多次，不承诺调用次数和顺序；同一记录在一次排序中应当返回相同的键 |
| 键不一致或 panic | 不产生未定义行为；输出仍是原数据的排列（具体顺序不承诺）；panic 原样向上传递，切片中保留全部记录 |
| 降序 | 用 `Reverse` 反转键，不反转升序结果，因此等键仍保持稳定 |
| 浮点 | 与 `total_cmp` 一致：`-0.0 < +0.0`，NaN 按位型排序。需要 NaN 互相等价、两种零相等的场景，由调用方先规范化键 |
| 128 位 | 按无符号高位、低位比较；有符号键翻转符号位 |
| 空间 | 分别说明本次调用所需的工作区、栈上固定表和 `Workspace` 保留的容量 |

### 5.3 Workspace 的复用边界

`Workspace` 只跨调用复用分配的容量。每次调用都重新建立有效记录、来源指针和恢复状态；上次调用留下的字节不能当作有效记录。用 workspace 排序的结果与对应的普通函数完全一致，key 函数 panic 后 workspace 仍可继续使用。

## 6. Stable Rust 移植

研究版代码真正依赖 nightly 的只有一处：`radix_support.rs:59-66` 用 `specialization` 实现 `RecordType::COPY`。研究仓库 `src/lib.rs` 的 feature 列表属于整个研究仓库，与 Tessera 无关；`core.rs` 里的 `unlikely` 是自己实现的。

替代方案（2026-11-06 定）：不再区分 Copy。`stable_sort_by_key` 对所有类型走带桶号缓存的路径，`stable_sort` 走不存桶号的路径，内核用私有 const 泛型参数选择，不在逐记录循环中做运行时判断。研究版 `RecordType` 的 specialization 和 Copy scratch 的已初始化前缀逻辑都不移植。

平台目标：Edition 2024，MSRV 先定为 1.85，由 CI 验证后写入 `rust-version`。开发工具（Miri、`-Zsanitizer`）可以用 nightly。

## 7. Benchmark

benchmark 分成三个用途，各自有固定的工具：

| 用途 | 回答的问题 | 工具 |
|---|---|---|
| 迁移验收（一次性） | 移植后的 crate 是否保持了研究版的性能 | 研究仓库现有的 `radix_ab`，在同一个二进制里对比 |
| 回归 A/B（日常） | 这次改动相对上一版本是变快还是变慢 | 新仓库的 Rust runner，新旧版本在同一进程里交错计时 |
| 对外报告（每次发版） | 与 std、其他 radix 排序相比在哪里快、哪里慢 | 同一个 runner 跑固定的 `core` 矩阵，用报告脚本从 raw 重新生成表格 |

### 7.1 迁移验收：复用研究仓库的 harness

研究阶段的所有数据都来自 `radix_ab`。用同一个 harness 对比，结果才能直接与历史数据比较。

先修正 `radix_ab` 的计时循环：`--batch 1` 把恢复输入的复制计入耗时，`--batch N>1` 不恢复输入（§104 已记录）。`cache-default-20261106/harness-and-arms.patch` 中的循环已改为每次计时前在计时外恢复全部缓冲，迁移验收直接沿用。

1. 在研究仓库新建临时分支，把新 crate 作为 path 依赖加入，在 `radix_ab` 中增加 `tessera_crate` 变体，与原有 `tessera` 变体放进同一个二进制。
2. 采用 `results/tessera/cleanup/` 的配对矩阵和机器设置（CPU 2 绑核、串行执行），同时加入一个 A/A 变体，以及一份逐字相同的研究版引擎副本作为代码放置对照：这台 Xeon Gold 6140 上仅对齐变化就能造成 10–24% 的差异（§104），缓存实验中副本相对原版在 1M 上也偏离到 4%。
3. 验收标准：每个工况的 `tessera_crate / tessera` 中位比值落在 A/A 与放置对照的范围内；超出范围的工况逐个查明原因，再决定是否发布。注意移植版对 Copy 记录改走桶号缓存，预期差异见 5.1，不按回退处理。
4. 研究仓库用 nightly 编译，产品用户用 stable。验收后在新 runner 中用 stable 工具链再跑一次同样的矩阵，作为新仓库的第一份基线。

这个临时分支不合并，结论记录到新仓库的 `docs/benchmarking.md`。

### 7.2 Rust runner 的结构

`bench/rust` 是一个 `publish = false` 的二进制 crate，提供三个子命令：

| 命令 | 作用 |
|---|---|
| `list` | 列出变体及其能力：稳定性、支持的类型、是否支持 workspace |
| `validate --suite S` | 对套件中的每个用例和变体校验有序性、完整排列，稳定变体还校验等键 ID 顺序 |
| `measure --suite S --out DIR` | 计时，写出 `samples.jsonl` 和 `manifest.json` |

用例通过完整的 `case_id` 精确选择（例如 `u64/random/4096/s1`），不用正则匹配，避免 `10K` 同时匹配到 `10M`。

**变体：**

- `tessera`：当前工作区中的 crate。
- `tessera_base`：作为基线的已发布版本或 git tag，通过重命名依赖引入（`tessera_base = { package = "tessera-sort", git = "...", tag = "v0.1.0" }`），与当前版本编译进同一个二进制。实施时先确认 Cargo 接受同名包来自 path 和 git 两个来源；如果不接受，改为构建两个二进制，由外层脚本交替调度。
- `std_stable`（`slice::sort_by_key`）、`std_unstable`（`sort_unstable_by_key`）。
- `radsort`，以及可选的其他 crates.io radix 排序（例如 `rdst`、`voracious_radix_sort`），引入前核对各自是否支持对应任务。

### 7.3 输入生成：写成规范，不提交数据文件

生成器按 `(分布, 长度, seed)` 确定性地产生输入，并在 `conformance/` 中写成规范，以后 C++ 照规范实现：

- 随机数源用 SplitMix64，规范里写明完整算法。
- 只用整数运算。Zipf 等分布用整数累积表加二分查找实现，不调用 `pow`、`ln`，因为这些函数在不同语言和库中的结果可能不一致。
- 浮点输入由整数位型构造，并覆盖 `±0`、`±inf`、多种 NaN payload 和次正规数。
- `conformance/vectors/` 保存每个 `core` 用例输入的 SHA-256 黄金哈希。runner 启动时校验，跨语言实现用同一组哈希证明输入一致。

大数组在内存中生成，不落盘，也不进 Git。

### 7.4 任务与类型

同一张排名表中，所有实现完成的是同一种任务；不支持该任务的实现标为“不支持”，不拿其他任务的耗时代替。

| 任务 | 类型 | 说明 |
|---|---|---|
| `keys` | `u32`、`u64`、`u128`、`i64`、`f64` | 只排序键数组 |
| `rows` | `{u32 key, u32 row}`、`{u64 key, u32 row}`、`{f64 key, u32 row}` | 数据库 key + 行号布局，对应 StarRocks 的 `InlinePermuteItem` |
| `records` | `{u64 key, u64 payload}`（16 B）、`{u64 key, [u8; 120]}`（128 B） | 整条记录随键移动 |
| `owned` | `{u64 key, String}` | 持有堆资源的记录 |

`rows` 和 `records` 中的 payload 填入可校验的原始 ID，用于检查排列和稳定性。不稳定变体只检查有序性和完整排列，不把 ID 加入比较键。

### 7.5 用例套件

| 套件 | 规模 | 用途 |
|---|---|---|
| `smoke` | `u64` 与 `{u64,u32}`；random、sorted、all_equal；长度 0、1、47、48、49、4096、1M | 每次改动和 CI：构建、输入恢复、校验、raw 格式 |
| `core` | 7.4 的全部类型 × 下列分布 × 下列长度 | 发版报告和回归 A/B 的固定矩阵 |
| `extended` | 更多 seed、浮点特殊值、复合键、宽记录、冷缓存模式 | 针对性调查 |

`core` 的分布：random、random_d256、random_d16、zipf11、zipf20、sorted、reverse、random_sorted95、random_sorted99、runs、all_equal。分布含义沿用研究仓库 `radix_ab` 的同名分布，在新规范中重新定义。

`core` 的长度：8、16、32、48、64、256、1K、4K、16K、64K、256K、1M、10M，再按引擎阈值补充：

- `INSERTION_MAX = 48` 附近：47、48、49；
- tile 容量边界：`TILE_BYTES = 64 KiB`，每种记录大小的 tile 记录数取 `min(64 KiB / size, 65535)` 及其前后各一个点。

阈值由 runner 从 crate 暴露的 `#[doc(hidden)]` 常量计算，内部参数调整后边界用例自动跟着变。

### 7.6 计时协议

每个测量单元只用一种流程：

1. 在计时外，把 batch 中每个缓冲恢复为原始输入。
2. 开始计时。
3. 依次排序这一批相互独立的缓冲。
4. 停止计时，记录整批的纳秒数和排序次数。
5. 在计时外校验每个缓冲：输出（键与 ID）的哈希必须等于参考哈希。参考哈希在启动时用 `std_stable` 的结果计算一次。

其他规则：

- batch 大小按目标计时时长（默认每个样本至少 5 ms）和准备内存上限（默认 256 MiB）共同确定，实际 batch 大小和占用字节数写入 raw。
- 每轮按随机顺序轮换变体；一个 block 是所有变体各跑一次，默认 9 个 block。
- 基线变体在每个 block 中出现两次（A1、A2），用来得到 A/A 噪声范围。
- 分配方式是独立的维度：`api` 模式调用普通自由函数，包含内部 scratch 的分配与释放；`workspace` 模式使用计时前准备好的 `Workspace`。不支持 workspace 的对照算法只参与 `api` 组。
- 缓存状态是另一个维度。`core` 只用默认的“计时前刚写入”状态；需要冷缓存时，在 `extended` 中每次排序前遍历一块 2 倍 LLC 大小的缓冲。

### 7.7 统计与结论

- 主统计量是逐 block 配对比值 `变体 / mean(基线 A1, A2)` 的中位数，同时报告 A/A 比值 `A2 / A1` 的范围和中位数。
- “更快”或“更慢”的结论需同时满足两条：中位比值落在 A/A 范围之外；9 个 block 中至少 8 个方向一致。达不到时写“差异在噪声内”。
- 跨构建或跨源码副本的比较还要加入放置对照（同一源码的另一份副本），其偏离范围与 A/A 一起作为噪声下限。
- 报告同时给出 ns/元素，由整批纳秒数和元素数重新计算。

### 7.8 Raw 与 manifest

`samples.jsonl` 每行一个样本：

```json
{"schema":1,"run_id":"2026-11-20-a","case_id":"u64/random/4096/s1","task":"keys",
 "variant":"tessera","kernel":"tessera-stable","alloc_mode":"api","block":2,"slot":3,
 "batch":64,"elements":262144,"elapsed_ns":1523840,"verified":true}
```

`manifest.json` 记录：各变体的版本与 git commit、runner 源码和二进制的哈希、`rustc -vV`、`RUSTFLAGS`、LTO 设置、CPU 型号与绑核、内核版本、用例计划和输入黄金哈希的校验结果。

报告脚本 `bench/report/report.py`（只用标准库）从 raw 和 manifest 重新生成 markdown 表格。遇到缺少样本、重复样本、`verified = false`、manifest 不匹配或输入哈希不符时直接失败。

发版时，正式结果用 `zstd` 压缩后作为 GitHub Release 附件保存，不进 Git；仓库只提交生成的 `docs/benchmark-results-vX.Y.md`。

### 7.9 构建配置

研究阶段的数据使用 `target-cpu=native` 和 thin LTO，而 crates.io 用户默认按通用 x86-64 编译。正式报告分两组：

| 配置 | 说明 |
|---|---|
| `default` | `cargo build --release`，不加 `target-cpu`；这是主报告 |
| `native` | `-C target-cpu=native`，fat LTO；作为附加列 |

如果两组之间差距明显，在 README 中给出建议的编译选项。迁移验收（7.1）阶段先测一次两组的差距，因为它决定 README 怎么写。

### 7.10 内存测量与集成 benchmark

**内存** 单独运行，不和计时一起跑。runner 的 `memory` 子命令安装计数分配器，按用例报告：本次调用的分配次数、请求字节数、峰值额外字节数，以及 `Workspace::retained_bytes()`。栈上固定表的大小由 crate 以常量公开，写在文档里。`workspace` 模式分配次数为零，不代表工作区为零，报告中两项分开写。没有测到的项目写“未测”，不用进程 RSS 代替。

**StarRocks 与 ClickHouse 集成 benchmark** 包含上游 Apache-2.0 代码，不放进本仓库。研究仓库的 `results/tessera/public-projects-20261105/` 已有完整做法；新版本发布后，在研究仓库或单独的 `tessera-integrations` 仓库中改为链接 crate 的发布版本重新跑一次，README 只链接结果。

### 7.11 Harness 自测

runner 自带两项测试，在 CI 中运行：

- 注册一个会修改输入、并且故意排错的假变体，确认每个变体、每一轮、batch 中每个缓冲都收到原始输入，并且校验能发现错误。
- 写出一批样本后读回并用报告脚本重算，确认耗时、单位和排序次数没有丢失。

### 7.12 以后接入 C++

C++ runner 按同一份用例规范和 JSONL 格式实现：

- 照 7.3 的规范实现生成器，并通过同一组黄金哈希；
- 计时循环自己写，不用 Google Benchmark 的汇总结果；
- 对照算法（`std::sort`、`std::stable_sort`、IPS²Ra、IPS⁴o、VQSort、x86-simd-sort、Fluxsort、Dovetail）用 `FetchContent` 按固定 commit 拉取。

C++ runner 可以在 Tessera C++ 实现完成之前先接入标准库和这些对照算法。跨语言对比不使用逐次调用的 FFI；需要同一 ELF 对照时，在被测语言内部完成整批排序和计时。

## 8. 测试与 CI

| 检查 | 内容 |
|---|---|
| 正确性 | 性质测试：随机输入与 `slice::sort_by_key` 结果逐项一致（含 ID）；`conformance/vectors/` 中的边界向量 |
| 恢复 | key panic、不一致的 key、owned 与独占引用记录（来自 `radix_safety.rs`、`radix_owned.rs`、`radix_views.rs`） |
| 内存安全 | Miri（缩小规模）、ASan，单独的 nightly job |
| 平台 | MSRV 与最新 stable；Linux、Windows、macOS；x86-64 与 AArch64 |
| 打包 | `cargo package`、文档示例、`cargo publish --dry-run` |
| benchmark | `validate --suite smoke` 与 harness 自测；共享 CI 不做性能门禁 |

正式性能数据只在固定机器上测：绑核、串行运行，计时期间不同时编译或跑测试。

## 9. 阶段与验收

| 阶段 | 交付 | 验收 |
|---|---|---|
| P0 准备（已完成） | 新建仓库；`LICENSE`、`README`、`semantics.md`、设计与 benchmark 文档、CI、贡献规范；冻结来源 commit | 语义文档覆盖 5.2 的所有项目 |
| P1 Stable 移植 | crate 在 stable 上构建；`stable_sort`、`stable_sort_by_key` 和 `Workspace`；迁移并重写测试 | 全部测试、Miri、ASan 通过；MSRV 在 CI 中验证 |
| P2 迁移验收 | 7.1 的配对测量；`default` 与 `native` 的差距 | 每个工况都在 A/A 范围内，或已查明并记录原因 |
| P3 runner 与 0.1 | 7.2–7.11 全部完成；首份 `core` 报告；发布到 crates.io | 报告可由 raw 重新生成；`validate` 在 CI 中通过 |
| P4 C++ runner | 标准库和对照算法接入；生成器通过黄金哈希 | 与 Rust runner 输出同一格式的 raw |
| P5 C++ 实现 | C++17 header-only，平凡可复制记录；CMake 安装与消费测试 | 通过同一组语义向量；有 Rust 与 C++ 的直接对照 |
| P6 集成 | 用发布版本重跑 StarRocks / ClickHouse | 结果链接进 README |

## 10. 待定事项

已定（2026-11-06）：0.1 不带 `unstable_sort`；`LICENSE` 署名为 `Yikai Liao <lyk-boya@outlook.com>`。

1. **准备内存上限和每个样本的目标时长（7.6）**按测试机器的内存和 10M 用例的实际耗时确定。
2. **8 字节记录补测**（P3 runner 完成后）：`{u32 key, u32 row}` 等 8 字节记录上统一缓存的成本；如回退明显，再评估是否加 `unsafe` 的 `_unchecked` 入口。
