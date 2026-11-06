
/root/code/sort-research-rs/logs/tessera-cleanup/fold-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056910 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>:
   56910:	add    %rcx,%rdx
   56913:	inc    %rdi
   56916:	je     56a8c <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x21c>
   5691c:	mov    %r8d,%r9d
   5691f:	mov    %eax,%r10d
   56922:	mov    -0x8(%rbx,%rdx,8),%rax
   56927:	cmp    (%rbx,%rdx,8),%rax
   5692b:	setbe  %al
   5692e:	setae  %r8b
   56932:	and    %r10b,%al
   56935:	and    %r9b,%r8b
   56938:	test   %al,%al
   5693a:	jne    56910 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>
   5693c:	test   %r8b,%r8b
   5693f:	jne    56910 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>
