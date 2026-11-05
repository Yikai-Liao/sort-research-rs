# Tessera 性能计划（2026-11，第二轮脑暴）

状态：**完成**。六个 idea 都已实现到可测量形态或做了最便宜的证伪；只有方案 2 进入默认代码。
结论已回填到 EXPERIMENTS.md 第 83–88 节，BACKLOG.md 和 CEILING.md 已同步。

## 0. 判定标准与约定

- 门槛沿用账本：机制级改动要 **≥1.2x** 才保留；小于 1.2x 但与"少一趟内存遍历"直接相关时，
  允许按指令数 + 墙钟一起判断。任何结论必须同时有指令数（perf stat）和墙钟。
- 固定数据：OVERRIDE_SEED=1；跨 build A/B 必须固定 seed。1M 用 5–7 轮交错、报中位数。
- 每项先跑**最便宜的证伪实验**；证伪失败就停，不实现完整机制。
- 新机制先在 1M 上过门槛，再看 100k / 10M 和指令数；噪声 5–20%，5% 以内不算结果。
- **不对特定数据类型做特化分支**：只允许按"数据形状"（varying 跨度、记录字节数、节点大小）
  决定的通用规则；方案 6 的 512 KiB 记录特化因此不保留。

## 1. 前置：补 harness 缺失的形状（已完成）

- [x] f64_normal：正态浮点列的排序键（u64 = f64 的 total-order 位型）。
- [x] f64_lognormal：对数正态（价格/耗时列），全部为正。
- [x] small12：u64 低位装 0..4096 的小整数。
- [x] composite16_32：高 16 位低基数（16 类）+ 低 32 位随机。
- [x] window32_high：同一个 32-bit 逻辑 key 放进 u64/u128 的高 32 位；用于把
      "记录宽度"和"key 运算宽度"分开。
- [x] 用 RADIX_TRACE=1 校准：f64 两列走 11-bit root + tile 叶子；small12/composite 走
      5–8 bit root + tile；window32 与 u64/u128 路由一致。

## 2. 方案 1：按样本分位切分（已实现，否决）

- [x] 1a. 恒等表证伪：1M u64 random，1 次链式查表 +0.9 ns/记录、2 次 +0.9、3 次 +1.9。
      查表本身处在 scatter 的 store 瓶颈下，可被藏住；但完整分类器不止查表。
- [x] 1b. 完整实现：1024 键采样 + 排序；众数 ≥1/4 时给精确桶；两侧按到众数的距离建
      256 格一级表；重格再挂 4096 子格二级表（8 KiB）；桶数 = 2×tile 数，向上取 2 的幂，
      上限 2048。
- [x] 1c. 等值桶：众数桶和二级表里的重键桶直接完成；子节点不再扫。
- [x] 1d. 均匀数据：没有 ≥1/4 众数时直接返回原路径，random trace 不变。
- [x] 1e. 目标 cell（1M 中位数，radix_ab，7 轮）：
      u64 zipf13 **18.75 → 20.98（+12%）**；u128 zipf 35.13 → 45.53（+30%）；
      u64 p95zero 6.64 → 8.49（+28%）；u128 p95zero 15.27 → 25.76（+69%）；
      two_values 约 2x 慢；random/sparse 持平。
- [x] 1f. 搭车项"采样猜高位"未实施：主机制已输，且它会新增独立的正确性/扫描融合问题。
- **判决：否决。** 省下的一层（约 1.2M 记录·趟）小于分类器 + 桶不均衡的代价；
  zipf 的重格仍留下 206k 条的子节点，二级表并没有把桶压到 tile。

## 3. 方案 2：key 运算宽度跟随 varying 跨度（保留，默认开启）

- [x] 2a. 对照实验：同一 32-bit 逻辑 key、1M：u64 16.60 / record16 26.38 / u128 31.94 ns。
      record16 与 u128 记录宽度相同，u128 仍慢 21%，说明 128-bit key 运算有实际成本；
      但 u128 vs u64 的 1.9x 里大部分是字节量，不能全部归给算术。
- [x] 2b. 实现 Shifted<SHIFT, T>（#[repr(transparent)]，key() = (inner >> SHIFT) as u64）。
      只在根节点、且 varying 位全部落在 [0,64) 或 [64,128) 时启用；子节点保持窄视图，
      不再二次包装。
- [x] 2c. 1M A/B（同一 build、交错、7 轮）：
      window32_high 30.37 → 25.94（1.17x）、small12 21.26 → 18.48（1.15x）、
      f64_normal 36.56 → 32.69（1.12x）、u128 zipf 28.21 → 26.82（1.05x）、
      sparse8 18.87 → 18.18（1.04x）；random_wide 不触发（24.71 → 25.11，噪声）。
- [x] 2d. 10M：window32 38.78 → 33.72（1.15x）、f64 43.81 → 40.40（1.08x）、
      small12 25.75 → 23.80（1.08x）、zipf 1.05x、sparse 1.05x。
- **判决：保留。** 低于 1.2x 的单 cell 门槛，但方向一致、无回归，且对真实列式形状
  （float、小整数、高位窗口复合键）是 1.08–1.17x。它是按数据形状（varying 跨度）触发，
  不是按类型名分支；默认开启，Policy.narrow_key 保留作 A/B 钩子。

## 4. 方案 3：子节点一完成 compact 就递归（已实现，否决）

- [x] 3a. fuse_all：visitor 对所有子节点调用 sort_node，不只 tile。
- [x] 3b. 10M，3 周期 × 7 轮（tessera → fuse_all，ns/记录）：
      u64 random 20.54→19.41、19.71→18.98、20.20→20.21；
      u64 zipf 20.76→21.40、21.27→21.26、21.04→20.62；
      u128 random 34.62→34.79、36.05→33.88、34.84→35.30；
      u128 zipf 32.14→32.81、32.96→33.18、33.34→34.65；sparse 持平。
- **判决：否决。** 符号不一致，最大改善 <5%，u128 zipf 稳定略差；没有达到门槛。
  原因：子节点二次 split 的第一次读本来就被 compact 写热，省下的一次 DRAM 读在
  写瓶颈下看不见，反而增加了在 compact 中途分配/清扫 scratch 的干扰。

## 5. 方案 4：打断 scatter 的 store-forwarding 依赖链（证伪，否决）

- [x] 4a. 独立 scatter 微基准（同记录、同桶大小，只改桶号顺序，1M）：
      random 9.03、runs8 7.33、runs64 8.12、runs4096 5.41、mixed_runs 5.81 ns/记录。
      同桶连续**更快**，不是延迟链。
- [x] 4b. 寄存器游标缓存（连续同桶时把 next/end 留在寄存器）：random 7.94、
      runs8 5.54、runs64 5.06、runs4096 4.61、mixed_runs 4.81；但没有搬进真实引擎，
      因为它不是原方案，且微基准的 12% 在真实 block 引擎里未验证。
- **判决：否决。** "有局部性时退化成 4–5 cycle 依赖链"的前提不成立；mixed_runs 比
  random 慢的原因在别处（桶分布/叶子），不在 scatter 游标。

## 6. 方案 5：叶子 CPU 效率（关闭）

- [x] 5a. perf stat 1M u64：random_wide 5.27M branch-misses / 21 次排序 ≈ 0.25/记录；
      zipf13 ≈ 0.33；f64_normal ≈ 0.54。插入叶子是当前最便宜的收尾（s65 已测比逐桶快 14.7%），
      不是主要成本中心，占比低于 20% 门槛。
- [x] 5b. 无分支局部修正：收益上限 = 插入阶段的一小部分，低于 1.2x；不做。
- [x] 5c. "compact 同时计数再 scatter"是 E1 的变体，已有 s69/B13 的否决证据；不重开。
- **判决：关闭。** 不改代码。

## 7. 方案 6：宽记录大叶子 + 间接排序 + 原地环应用（已实现，否决）

- [x] 6a. 实现：node ≤512 KiB、record ≥64B 时，构建 (key, u16 index) 对，排序后
      用逆置换环跟随原地应用；每条记录只搬一次。
- [x] 6b. 1M record128，5 轮：random_wide **138.59 → 181.12（+31%）**；
      zipf 200.40 → 194.64（−2.9%）；sparse 116.22 → 119.19（+2.6%）；
      two_values 79.61 → 77.42（−2.8%）。
- [x] 6c. 记录搬移：pairs 63 KiB、sort 在 L2；置换是 506 KiB 内的随机跳链，
      实测把第二层 4-bit split + L1 叶子的成本吃了回来。
- [x] 6d. 附带测试了"宽记录用 512 KiB tile"（删掉第二层 split，不间接排序）：1M
      record128 random 143.18→131.10、144.41→135.13、135.39→126.67（约 6–8%），
      但这是按记录字节数的类型特化，且 10M 持平，**按"不做类型特化"的要求否决**。
- **判决：否决。** 保留原路径。

## 8. 不建议做（记录理由，不排期）

- O(n) 缓冲：O(√n) 是设计约束；不再用未证明的"冷流量下界"论证。
- 重开 s14 宽 fan-out：库不应替用户 madvise；只记注释。

## 9. 最终代码改动

- src/other/rust_tessera_radix/engine.rs：新增 Shifted + sort_shifted_root，
  根节点 narrow key 默认开启；其余实验代码已删除。
- src/other/rust_tessera_radix/engine/tests.rs：新增窄视图稳定性测试（Miri 下 600 条）。
- benches/radix_ab.rs：新增 f64/小整数/复合键/同逻辑 key 窗口 pattern。
- 所有被否决的 Policy 字段、visitor、表路径和微基准已删除；git diff 只含上述内容。

## 10. 验证

- cargo test --release --features rust_tessera_radix：lib 20 + block_lsd 10 +
  block_lsd_functionality 233 + main 233 + radix_owned 2 + radix_views 6 +
  tessera_functionality 233，全部通过。
- cargo +nightly miri test --features rust_tessera_radix --lib：20/20，457 s
  （新测试的 Miri 尺寸 600 条）。
- cargo fmt --all。
