
/root/code/sort-research-rs/logs/tessera-cleanup/range-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000057230 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x780>:
   57230:	cmp    %rbx,%rsi
   57233:	jae    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   57239:	mov    %rsi,%rdi
   5723c:	lea    0x1(%rsi),%r9
   57240:	cmp    %r9,%rbx
   57243:	mov    %r9,%rsi
   57246:	cmova  %rbx,%rsi
   5724a:	lea    -0x1(%rsi),%r10
   5724e:	cmp    %r10,%rdi
   57251:	je     57273 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7c3>
   57253:	mov    (%r14,%rdi,8),%r10
   57257:	nopw   0x0(%rax,%rax,1)
   57260:	cmp    %r10,(%r14,%r9,8)
   57264:	jne    57270 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7c0>
   57266:	inc    %r9
   57269:	cmp    %r9,%rsi
   5726c:	jne    57260 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7b0>
   5726e:	jmp    57273 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7c3>
   57270:	mov    %r9,%rsi
   57273:	mov    %rsi,%r10
   57276:	sub    %rdi,%r10
   57279:	jb     5764f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb9f>
   5727f:	mov    %r10,%r9
   57282:	shr    $1,%r9
   57285:	je     57230 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x780>
