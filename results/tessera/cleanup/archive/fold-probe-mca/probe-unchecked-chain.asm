
/root/code/sort-research-rs/logs/tessera-cleanup/unchecked-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056a10 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>:
   56a10:	add    %rcx,%rdx
   56a13:	inc    %rsi
   56a16:	je     56b8c <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x21c>
   56a1c:	mov    %edi,%r8d
   56a1f:	mov    %eax,%r9d
   56a22:	mov    -0x8(%r14,%rdx,8),%rax
   56a27:	cmp    (%r14,%rdx,8),%rax
   56a2b:	setbe  %al
   56a2e:	setae  %dil
   56a32:	and    %r9b,%al
   56a35:	and    %r8b,%dil
   56a38:	test   %al,%al
   56a3a:	jne    56a10 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>
   56a3c:	test   %dil,%dil
   56a3f:	jne    56a10 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>
