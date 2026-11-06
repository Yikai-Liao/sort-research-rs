
/root/code/sort-research-rs/logs/tessera-cleanup/range-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000057180 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x6d0>:
   57180:	movdqu -0x10(%rdi),%xmm0
   57185:	movdqu (%rdi),%xmm1
   57189:	movdqu -0x10(%r8,%r9,8),%xmm2
   57190:	movdqu (%r8,%r9,8),%xmm3
   57196:	pshufd $0x4e,%xmm3,%xmm3
   5719b:	pshufd $0x4e,%xmm2,%xmm2
   571a0:	movdqu %xmm3,-0x10(%rdi)
   571a5:	movdqu %xmm2,(%rdi)
   571a9:	pshufd $0x4e,%xmm0,%xmm0
   571ae:	pshufd $0x4e,%xmm1,%xmm1
   571b3:	movdqu %xmm0,(%r8,%r9,8)
   571b9:	movdqu %xmm1,-0x10(%r8,%r9,8)
   571c0:	add    $0xfffffffffffffffc,%r9
   571c4:	add    $0x20,%rdi
   571c8:	cmp    %r9,%rsi
   571cb:	jne    57180 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x6d0>
