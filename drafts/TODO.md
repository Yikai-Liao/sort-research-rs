# block LSD radix sort — PR 交付清单

分支：sort-research-rs feat/block-lsd（fork: Yikai-Liao/sort-research-rs），关联 issue #35

## 已完成

- [x] vendored 实现 src/other/rust_block_lsd/core.rs + adapter、radix_support.rs 键契约
- [x] sort_test_tools::Sort 增加 supports_type / supports_sort_by 声明，block LSD 走完整 233 条标准套件
- [x] 额外测试：block 边界、记录宽度、整数宽度、拒绝路径、非 Copy 记录、drop 计数、panic 恢复（8 + 233 + 2 + 6）
- [x] release / ASan / Miri 全绿（Miri: 8 + 2 + 6，缩规模记录在测试注释）
- [x] benches/extra_memory.rs：峰值额外堆
- [x] benches/modules/radsort_keyed.rs：keyed radsort 对照
- [x] 图工具为新 sort 注册元信息（util.py）
- [x] 热循环优化：去 bounds check、提出 shift、group-of-4 scatter
- [x] panic 恢复按 needs_drop::<T>() 特化：Copy 记录省 3%，非 Copy 仍精确恢复（drop 计数验证）
- [x] 与 C 参考对齐：同策略下指令数 gcc/clang/rust 同量级；C++ 模板版对比 0.96–1.06
- [x] 多用例对比：pair / u64 / u128 / 32B / 128B 记录，Rust vs C++ 模板版（交替测量 9 轮中位数）
- [x] 仓库标准 benchmark：critcmp baseline block_lsd，hot + cold，多种类型
- [x] 仓库标准图：analysis_block_lsd/（scaling / single_size / direct_versus，HTML）
- [x] issue 回复草稿 issue-reply.md
- [x] PR body 草稿 pr-body.md

## 待办

- [x] cold 数据并入 JSON（80 项）后重出图，填两份草稿的表格
- [x] 生成 PNG 图（figures/scaling-cold.png、scaling-hot.png、relative.png）
- [x] 提交（429f6f7 needs_drop 特化 + util.py 元信息）
- [x] push feat/block-lsd 到 fork（429f6f7）
- [ ] 草稿交用户审阅（issue-reply.md / pr-body.md）

## 路径

- 主工作区 /root/code/sort-research-rs，target /root/code/sort-research-rs-target（ASan: -target-asan）
- 结果 /root/code/sort-research-rs-pr/results/block_lsd.json，图 analysis_block_lsd/
- C/C++ 对比 /root/code/sort-research-rs-c-compare（c-compare/ 原 C，cxx/ 模板版，rust-compare/）
