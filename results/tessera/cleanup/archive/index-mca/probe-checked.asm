
/root/code/sort-research-rs/logs/tessera-cleanup/range-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056b50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>:
   56b50:	lea    -0x1(%rdi),%rax
   56b54:	cmp    %rbx,%rax
   56b57:	jae    57699 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbe9>
   56b5d:	cmp    %rbx,%rdi
   56b60:	jae    57689 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbd9>
   56b66:	mov    (%r14,%rdi,8),%rax
   56b6a:	cmp    %rax,-0x8(%r14,%rdi,8)
   56b6f:	setbe  %al
   56b72:	setae  %r9b
   56b76:	and    %al,%dl
   56b78:	and    %r9b,%r8b
   56b7b:	mov    %edx,%eax
   56b7d:	or     %r8b,%al
   56b80:	je     56d5f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2af>
   56b86:	add    %rcx,%rdi
   56b89:	inc    %rsi
   56b8c:	jne    56b50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa0>
