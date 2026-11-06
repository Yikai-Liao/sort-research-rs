
/root/code/sort-research-rs/logs/tessera-cleanup/api-compat-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056be0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>:
   56be0:	dec    %rdx
   56be3:	sub    %rax,%rdx
   56be6:	jb     56d6d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23d>
   56bec:	cmp    $0x1,%rdx
   56bf0:	jbe    56d6d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23d>
   56bf6:	mov    -0x8(%r10),%r11
   56bfa:	cmp    (%r10),%r11
   56bfd:	lea    (%r10,%rcx,1),%r11
   56c01:	setbe  %r10b
   56c05:	setae  %bl
   56c08:	and    %r10b,%r9b
   56c0b:	and    %bl,%r8b
   56c0e:	mov    %r11,%r10
   56c11:	test   %r9b,%r9b
   56c14:	jne    56be0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>
   56c16:	mov    %r11,%r10
   56c19:	test   %r8b,%r8b
   56c1c:	jne    56be0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>
