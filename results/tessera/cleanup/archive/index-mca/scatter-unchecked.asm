
/root/code/sort-research-rs/logs/tessera-cleanup/scatter-unchecked-bench:     file format elf64-x86-64


Disassembly of section .text:

000000000003c500 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x1e0>:
   3c500:	mov    -0x18(%r12),%rax
   3c505:	mov    %rax,%rbx
   3c508:	and    %r14,%rbx
   3c50b:	mov    %r9d,%ecx
   3c50e:	shr    %cl,%rbx
   3c511:	mov    %rbx,%r15
   3c514:	shl    $0x4,%r15
   3c518:	mov    0x0(%r13,%r15,1),%rcx
   3c51d:	lea    0x8(%rcx),%rdx
   3c521:	cmp    0x8(%r13,%r15,1),%rdx
   3c526:	mov    %rax,(%rcx)
   3c529:	mov    %rdx,0x0(%r13,%r15,1)
   3c52e:	je     3c5e4 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x2c4>
   3c534:	mov    -0x10(%r12),%rax
   3c539:	mov    %rax,%r15
   3c53c:	and    %r14,%r15
   3c53f:	mov    %r9d,%ecx
   3c542:	shr    %cl,%r15
   3c545:	mov    %r15,%rbx
   3c548:	shl    $0x4,%rbx
   3c54c:	mov    0x0(%r13,%rbx,1),%rcx
   3c551:	lea    0x8(%rcx),%rdx
   3c555:	cmp    0x8(%r13,%rbx,1),%rdx
   3c55a:	mov    %rax,(%rcx)
   3c55d:	mov    %rdx,0x0(%r13,%rbx,1)
   3c562:	je     3c67d <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x35d>
   3c568:	mov    -0x8(%r12),%rax
   3c56d:	mov    %rax,%r15
   3c570:	and    %r14,%r15
   3c573:	mov    %r9d,%ecx
   3c576:	shr    %cl,%r15
   3c579:	mov    %r15,%rbx
   3c57c:	shl    $0x4,%rbx
   3c580:	mov    0x0(%r13,%rbx,1),%rcx
   3c585:	lea    0x8(%rcx),%rdx
   3c589:	cmp    0x8(%r13,%rbx,1),%rdx
   3c58e:	mov    %rax,(%rcx)
   3c591:	mov    %rdx,0x0(%r13,%rbx,1)
   3c596:	je     3c717 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x3f7>
   3c59c:	mov    (%r12),%rax
   3c5a0:	mov    %rax,%r15
   3c5a3:	and    %r14,%r15
   3c5a6:	mov    %r9d,%ecx
   3c5a9:	shr    %cl,%r15
   3c5ac:	mov    %r15,%rbx
   3c5af:	shl    $0x4,%rbx
   3c5b3:	mov    0x0(%r13,%rbx,1),%rcx
   3c5b8:	lea    0x8(%rcx),%rdx
   3c5bc:	cmp    0x8(%r13,%rbx,1),%rdx
   3c5c1:	mov    %rax,(%rcx)
   3c5c4:	mov    %rdx,0x0(%r13,%rbx,1)
   3c5c9:	je     3c7b1 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x491>
   3c5cf:	dec    %rbp
   3c5d2:	add    $0x20,%r12
   3c5d6:	test   %rbp,%rbp
   3c5d9:	jne    3c500 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x1e0>
