
/root/code/sort-research-rs/logs/tessera-cleanup/unchecked-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

00000000000573e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa70>:
   573e0:	cmp    %rbx,%rsi
   573e3:	jae    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   573e9:	mov    %rsi,%rdi
   573ec:	lea    0x1(%rsi),%r9
   573f0:	cmp    %r9,%rbx
   573f3:	mov    %r9,%rsi
   573f6:	cmova  %rbx,%rsi
   573fa:	lea    -0x1(%rsi),%r10
   573fe:	cmp    %r10,%rdi
   57401:	je     57423 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab3>
   57403:	mov    (%r14,%rdi,8),%r10
   57407:	nopw   0x0(%rax,%rax,1)
   57410:	cmp    %r10,(%r14,%r9,8)
   57414:	jne    57420 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab0>
   57416:	inc    %r9
   57419:	cmp    %r9,%rsi
   5741c:	jne    57410 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xaa0>
   5741e:	jmp    57423 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab3>
   57420:	mov    %r9,%rsi
   57423:	mov    %rsi,%r10
   57426:	sub    %rdi,%r10
   57429:	jb     574ef <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb7f>
   5742f:	mov    %r10,%r9
   57432:	shr    $1,%r9
   57435:	je     573e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa70>
