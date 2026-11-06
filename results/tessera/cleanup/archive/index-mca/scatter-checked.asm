
/root/code/sort-research-rs/logs/tessera-cleanup/final-bench:     file format elf64-x86-64


Disassembly of section .text:

000000000003f0d0 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x1f0>:
   3f0d0:	mov    -0x18(%rbx),%rax
   3f0d4:	mov    %rax,%rbp
   3f0d7:	and    %r14,%rbp
   3f0da:	mov    0xf0(%rsp),%rcx
   3f0e2:	shr    %cl,%rbp
   3f0e5:	mov    %rbp,%r12
   3f0e8:	shl    $0x4,%r12
   3f0ec:	mov    0x0(%r13,%r12,1),%rcx
   3f0f1:	lea    0x8(%rcx),%rdx
   3f0f5:	cmp    0x8(%r13,%r12,1),%rdx
   3f0fa:	mov    %rax,(%rcx)
   3f0fd:	mov    %rdx,0x0(%r13,%r12,1)
   3f102:	je     3f1c4 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x2e4>
   3f108:	mov    -0x10(%rbx),%rax
   3f10c:	mov    %rax,%rbp
   3f10f:	and    %r14,%rbp
   3f112:	mov    0xf0(%rsp),%rcx
   3f11a:	shr    %cl,%rbp
   3f11d:	mov    %rbp,%r12
   3f120:	shl    $0x4,%r12
   3f124:	mov    0x0(%r13,%r12,1),%rcx
   3f129:	lea    0x8(%rcx),%rdx
   3f12d:	cmp    0x8(%r13,%r12,1),%rdx
   3f132:	mov    %rax,(%rcx)
   3f135:	mov    %rdx,0x0(%r13,%r12,1)
   3f13a:	je     3f289 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x3a9>
   3f140:	mov    -0x8(%rbx),%rax
   3f144:	mov    %rax,%rbp
   3f147:	and    %r14,%rbp
   3f14a:	mov    0xf0(%rsp),%rcx
   3f152:	shr    %cl,%rbp
   3f155:	mov    %rbp,%r12
   3f158:	shl    $0x4,%r12
   3f15c:	mov    0x0(%r13,%r12,1),%rcx
   3f161:	lea    0x8(%rcx),%rdx
   3f165:	cmp    0x8(%r13,%r12,1),%rdx
   3f16a:	mov    %rax,(%rcx)
   3f16d:	mov    %rdx,0x0(%r13,%r12,1)
   3f172:	je     3f34e <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x46e>
   3f178:	mov    (%rbx),%rax
   3f17b:	mov    %rax,%rbp
   3f17e:	and    %r14,%rbp
   3f181:	mov    0xf0(%rsp),%rcx
   3f189:	shr    %cl,%rbp
   3f18c:	mov    %rbp,%r12
   3f18f:	shl    $0x4,%r12
   3f193:	mov    0x0(%r13,%r12,1),%rcx
   3f198:	lea    0x8(%rcx),%rdx
   3f19c:	cmp    0x8(%r13,%r12,1),%rdx
   3f1a1:	mov    %rax,(%rcx)
   3f1a4:	mov    %rdx,0x0(%r13,%r12,1)
   3f1a9:	je     3f413 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x533>
   3f1af:	dec    %r15
   3f1b2:	add    $0x20,%rbx
   3f1b6:	test   %r15,%r15
   3f1b9:	jne    3f0d0 <_RINvMs_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other14rust_block_lsd4coreINtB5_6SorteryE7scatterNCINvB5_21partition_digit_inneryE0ECs59F9KgNujep_8radix_ab+0x1f0>
