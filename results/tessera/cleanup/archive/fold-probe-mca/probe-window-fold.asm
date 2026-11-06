
/root/code/sort-research-rs/logs/tessera-cleanup/fold-window-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056ba0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>:
   56ba0:	dec    %rdx
   56ba3:	sub    %rax,%rdx
   56ba6:	jb     56d2d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23d>
   56bac:	cmp    $0x1,%rdx
   56bb0:	jbe    56d2d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23d>
   56bb6:	mov    -0x8(%r10),%r11
   56bba:	cmp    (%r10),%r11
   56bbd:	lea    (%r10,%rcx,1),%r11
   56bc1:	setbe  %r10b
   56bc5:	setae  %bl
   56bc8:	and    %r10b,%r9b
   56bcb:	and    %bl,%r8b
   56bce:	mov    %r11,%r10
   56bd1:	test   %r9b,%r9b
   56bd4:	jne    56ba0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>
   56bd6:	mov    %r11,%r10
   56bd9:	test   %r8b,%r8b
   56bdc:	jne    56ba0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb0>
