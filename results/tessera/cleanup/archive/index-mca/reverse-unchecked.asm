
/root/code/sort-research-rs/logs/tessera-cleanup/unchecked-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000057333 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x9c3>:
   57333:	movdqu -0x10(%rsi),%xmm0
   57338:	movdqu (%rsi),%xmm1
   5733c:	movdqu -0x10(%rdi,%r8,8),%xmm2
   57343:	movdqu (%rdi,%r8,8),%xmm3
   57349:	pshufd $0x4e,%xmm3,%xmm3
   5734e:	pshufd $0x4e,%xmm2,%xmm2
   57353:	movdqu %xmm3,-0x10(%rsi)
   57358:	movdqu %xmm2,(%rsi)
   5735c:	pshufd $0x4e,%xmm0,%xmm0
   57361:	pshufd $0x4e,%xmm1,%xmm1
   57366:	movdqu %xmm0,(%rdi,%r8,8)
   5736c:	movdqu %xmm1,-0x10(%rdi,%r8,8)
   57373:	add    $0xfffffffffffffffc,%r8
   57377:	add    $0x20,%rsi
   5737b:	cmp    %r8,%rdx
   5737e:	jne    57333 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x9c3>
