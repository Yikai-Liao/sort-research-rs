
logs/tessera-cleanup/range-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056ab0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab>:
   56ab0:	push   %rbp
   56ab1:	push   %r15
   56ab3:	push   %r14
   56ab5:	push   %r13
   56ab7:	push   %r12
   56ab9:	push   %rbx
   56aba:	sub    $0x28,%rsp
   56abe:	cmp    $0x2,%rsi
   56ac2:	jae    56ad3 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23>
   56ac4:	add    $0x28,%rsp
   56ac8:	pop    %rbx
   56ac9:	pop    %r12
   56acb:	pop    %r13
   56acd:	pop    %r14
   56acf:	pop    %r15
   56ad1:	pop    %rbp
   56ad2:	ret
   56ad3:	mov    %rsi,%rbx
   56ad6:	mov    %rdi,%r14
   56ad9:	cmp    $0x9,%rdx
   56add:	mov    $0x8,%eax
   56ae2:	cmovae %rdx,%rax
   56ae6:	cmp    $0x7fff8,%rax
   56aec:	mov    $0x7fff8,%r13d
   56af2:	cmovb  %rax,%r13
   56af6:	mov    %rsi,%rcx
   56af9:	shr    $0xc,%rcx
   56afd:	cmp    $0x1,%rcx
   56b01:	adc    $0x0,%rcx
   56b05:	xor    %eax,%eax
   56b07:	mov    %rsi,%rdx
   56b0a:	sub    %rcx,%rdx
   56b0d:	cmovae %rdx,%rax
   56b11:	mov    %rax,%rdx
   56b14:	or     %rcx,%rdx
   56b17:	shr    $0x20,%rdx
   56b1b:	je     56b24 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x74>
   56b1d:	xor    %edx,%edx
   56b1f:	div    %rcx
   56b22:	jmp    56b28 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x78>
   56b24:	xor    %edx,%edx
   56b26:	div    %ecx
   56b28:	xor    %esi,%esi
   56b2a:	test   %rdx,%rdx
   56b2d:	setne  %sil
   56b31:	add    %rax,%rsi
   56b34:	je     56bc7 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x117>
   56b3a:	neg    %rsi
   56b3d:	mov    $0x1,%dl
   56b3f:	mov    $0x1,%r8b
   56b42:	mov    %rcx,%rdi
   56b45:	data16 cs nopw 0x0(%rax,%rax,1)
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
   56b8e:	and    %r8b,%dl
   56b91:	mov    (%r14),%rax
   56b94:	lea    0x8(%r14),%rsi
   56b98:	test   %dl,%dl
   56b9a:	jne    56bd4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x124>
   56b9c:	lea    0x0(,%rbx,8),%rcx
   56ba4:	lea    -0x8(,%rbx,8),%r8
   56bac:	xor    %edx,%edx
   56bae:	xchg   %ax,%ax
   56bb0:	mov    (%rsi,%rdx,1),%rdi
   56bb4:	cmp    %rax,%rdi
   56bb7:	jne    56c04 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x154>
   56bb9:	add    $0x8,%rdx
   56bbd:	cmp    %rdx,%r8
   56bc0:	jne    56bb0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x100>
   56bc2:	jmp    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56bc7:	mov    $0x1,%dl
   56bc9:	mov    (%r14),%rax
   56bcc:	lea    0x8(%r14),%rsi
   56bd0:	test   %dl,%dl
   56bd2:	je     56b9c <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xec>
   56bd4:	cmp    $0x401,%rbx
   56bdb:	mov    $0x401,%edi
   56be0:	cmovb  %rbx,%rdi
   56be4:	lea    0x0(,%rdi,8),%edx
   56beb:	lea    (%r14,%rdx,1),%rcx
   56bef:	lea    -0x10(%rdx),%r10
   56bf3:	cmp    $0x18,%r10
   56bf7:	jae    56c44 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x194>
   56bf9:	mov    %rax,%r8
   56bfc:	mov    %rax,%r9
   56bff:	jmp    56cd0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x220>
   56c04:	lea    -0x10(%rcx),%rsi
   56c08:	cmp    %rdi,%rax
   56c0b:	jae    56d30 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x280>
   56c11:	cmp    %rdx,%rsi
   56c14:	je     56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56c1a:	add    $0xffffffffffffffe8,%rcx
   56c1e:	xchg   %ax,%ax
   56c20:	mov    %rdi,%rax
   56c23:	mov    0x10(%r14,%rdx,1),%rdi
   56c28:	cmp    %rdi,%rax
   56c2b:	ja     56c36 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x186>
   56c2d:	cmp    %rdx,%rcx
   56c30:	lea    0x8(%rdx),%rdx
   56c34:	jne    56c20 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x170>
   56c36:	cmp    %rdi,%rax
   56c39:	jbe    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56c3f:	jmp    56d5f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2af>
   56c44:	shr    $0x3,%r10
   56c48:	inc    %r10
   56c4b:	mov    %r10,%r11
   56c4e:	and    $0xfffffffffffffffc,%r11
   56c52:	lea    (%rsi,%r11,8),%rsi
   56c56:	movq   %rax,%xmm2
   56c5b:	shufpd $0x2,-0x4ce64(%rip),%xmm2        # 9e00 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56c64:	movq   %rax,%xmm1
   56c69:	pxor   %xmm0,%xmm0
   56c6d:	pcmpeqd %xmm3,%xmm3
   56c71:	xor    %r8d,%r8d
   56c74:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   56c80:	movdqu 0x8(%r14,%r8,8),%xmm4
   56c87:	movdqu 0x18(%r14,%r8,8),%xmm5
   56c8e:	por    %xmm4,%xmm1
   56c92:	por    %xmm5,%xmm0
   56c96:	andpd  %xmm4,%xmm2
   56c9a:	pand   %xmm5,%xmm3
   56c9e:	add    $0x4,%r8
   56ca2:	cmp    %r8,%r11
   56ca5:	jne    56c80 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x1d0>
   56ca7:	pand   %xmm2,%xmm3
   56cab:	pshufd $0xee,%xmm3,%xmm2
   56cb0:	pand   %xmm3,%xmm2
   56cb4:	movq   %xmm2,%r8
   56cb9:	por    %xmm1,%xmm0
   56cbd:	pshufd $0xee,%xmm0,%xmm1
   56cc2:	por    %xmm0,%xmm1
   56cc6:	movq   %xmm1,%r9
   56ccb:	cmp    %r11,%r10
   56cce:	je     56ce2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x232>
   56cd0:	mov    (%rsi),%r10
   56cd3:	or     %r10,%r9
   56cd6:	and    %r10,%r8
   56cd9:	add    $0x8,%rsi
   56cdd:	cmp    %rcx,%rsi
   56ce0:	jne    56cd0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x220>
   56ce2:	mov    %r8,%r15
   56ce5:	xor    %r9,%r15
   56ce8:	cmp    $0xffffffffffffffff,%r15
   56cec:	je     56d24 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x274>
   56cee:	lea    0x0(,%rbx,8),%rsi
   56cf6:	cmp    %rsi,%rdx
   56cf9:	je     570b5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x605>
   56cff:	not    %rdi
   56d02:	add    %rbx,%rdi
   56d05:	movabs $0x1fffffffffffffff,%rsi
   56d0f:	and    %rdi,%rsi
   56d12:	cmp    $0x3,%rsi
   56d16:	jae    57000 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x550>
   56d1c:	mov    %r9,%r15
   56d1f:	jmp    5708f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x5df>
   56d24:	mov    $0xffffffffffffffff,%r15
   56d2b:	jmp    570be <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x60e>
   56d30:	cmp    %rdx,%rsi
   56d33:	je     56eac <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3fc>
   56d39:	lea    -0x18(%rcx),%rax
   56d3d:	nopl   (%rax)
   56d40:	mov    %rdi,%rsi
   56d43:	mov    0x10(%r14,%rdx,1),%rdi
   56d48:	cmp    %rdi,%rsi
   56d4b:	jb     56d56 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2a6>
   56d4d:	cmp    %rdx,%rax
   56d50:	lea    0x8(%rdx),%rdx
   56d54:	jne    56d40 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x290>
   56d56:	cmp    %rdi,%rsi
   56d59:	jae    56eac <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3fc>
   56d5f:	mov    %r13,(%rsp)
   56d63:	mov    %r13,%rax
   56d66:	shr    $0x3,%rax
   56d6a:	cmp    $0x3,%rax
   56d6e:	mov    $0x2,%r13d
   56d74:	cmovae %rax,%r13
   56d78:	cmp    %rbx,%r13
   56d7b:	mov    %rbx,%rbp
   56d7e:	cmovb  %r13,%rbp
   56d82:	lea    0x0(,%rbp,8),%r12
   56d8a:	mov    %r12,%rdi
   56d8d:	call   *0x83cb5(%rip)        # daa48 <malloc@GLIBC_2.2.5>
   56d93:	test   %rax,%rax
   56d96:	je     576ac <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbfc>
   56d9c:	mov    %rax,%r15
   56d9f:	mov    %rax,%rdi
   56da2:	mov    %r14,%rsi
   56da5:	mov    %r12,%rdx
   56da8:	call   *0x83cba(%rip)        # daa68 <memcpy@GLIBC_2.14>
   56dae:	mov    %rbp,%rax
   56db1:	neg    %rax
   56db4:	xor    %r12d,%r12d
   56db7:	xor    %ecx,%ecx
   56db9:	lea    0x0(,%rcx,8),%rdx
   56dc1:	mov    (%r15,%rcx,8),%rsi
   56dc5:	inc    %rcx
   56dc8:	neg    %rdx
   56dcb:	nopl   0x0(%rax,%rax,1)
   56dd0:	mov    %rsi,%rdi
   56dd3:	mov    (%r15,%rcx,8),%rsi
   56dd7:	cmp    %rsi,%rdi
   56dda:	ja     56dea <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x33a>
   56ddc:	inc    %rcx
   56ddf:	add    $0xfffffffffffffff8,%rdx
   56de3:	cmp    %rcx,%rbp
   56de6:	jne    56dd0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x320>
   56de8:	jmp    56e69 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3b9>
   56dea:	cmp    %rcx,%rbp
   56ded:	jb     5765f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbaf>
   56df3:	test   %rcx,%rcx
   56df6:	je     5767a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbca>
   56dfc:	mov    %rbp,%rsi
   56dff:	sub    %rcx,%rsi
   56e02:	je     576ba <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xc0a>
   56e08:	mov    -0x8(%r15,%rcx,8),%r8
   56e0d:	mov    (%r15,%rcx,8),%rdi
   56e11:	mov    %rdi,-0x8(%r15,%rcx,8)
   56e16:	mov    %r8,(%r15,%rcx,8)
   56e1a:	inc    %r12
   56e1d:	cmp    %rbp,%r12
   56e20:	ja     56e9d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3ed>
   56e22:	lea    (%r15,%rcx,8),%r8
   56e26:	add    $0xfffffffffffffff8,%r8
   56e2a:	nopw   0x0(%rax,%rax,1)
   56e30:	test   %rdx,%rdx
   56e33:	je     56e5f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3af>
   56e35:	mov    -0x8(%r8),%r9
   56e39:	cmp    %rdi,%r9
   56e3c:	jbe    56e5f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3af>
   56e3e:	mov    %rdi,-0x8(%r8)
   56e42:	mov    %r9,(%r8)
   56e45:	add    $0xfffffffffffffff8,%r8
   56e49:	add    $0x8,%rdx
   56e4d:	lea    (%rax,%r12,1),%r9
   56e51:	inc    %r9
   56e54:	inc    %r12
   56e57:	cmp    $0x1,%r9
   56e5b:	jne    56e30 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x380>
   56e5d:	jmp    56e9d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3ed>
   56e5f:	cmp    $0x2,%rsi
   56e63:	jae    56db9 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x309>
   56e69:	mov    %r15,%rdi
   56e6c:	call   *0x83be6(%rip)        # daa58 <free@GLIBC_2.2.5>
   56e72:	test   %r12,%r12
   56e75:	je     56e8e <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3de>
   56e77:	imul   %rbx,%r12
   56e7b:	mov    %r12,%rax
   56e7e:	shr    $0x20,%rax
   56e82:	je     56ec3 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x413>
   56e84:	mov    %r12,%rax
   56e87:	xor    %edx,%edx
   56e89:	div    %rbp
   56e8c:	jmp    56eca <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x41a>
   56e8e:	cmp    %r13,%rbx
   56e91:	mov    (%rsp),%r13
   56e95:	jbe    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56e9b:	jmp    56ee5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x435>
   56e9d:	mov    %r15,%rdi
   56ea0:	call   *0x83bb2(%rip)        # daa58 <free@GLIBC_2.2.5>
   56ea6:	mov    (%rsp),%r13
   56eaa:	jmp    56ee5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x435>
   56eac:	mov    %rbx,%rax
   56eaf:	shr    $1,%rax
   56eb2:	cmp    $0x8,%rbx
   56eb6:	jae    5714d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x69d>
   56ebc:	xor    %edx,%edx
   56ebe:	jmp    571d2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x722>
   56ec3:	mov    %r12d,%eax
   56ec6:	xor    %edx,%edx
   56ec8:	div    %ebp
   56eca:	mov    (%rsp),%r13
   56ece:	mov    %r14,%rdi
   56ed1:	mov    %rbx,%rsi
   56ed4:	mov    %rax,%rdx
   56ed7:	call   58250 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine23budgeted_insertion_sortyyNvYyNtNtB6_13radix_support3Key3keyECs59F9KgNujep_8radix_ab>
   56edc:	test   %rax,%rax
   56edf:	jne    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56ee5:	cmp    $0x401,%rbx
   56eec:	mov    $0x401,%esi
   56ef1:	cmovb  %rbx,%rsi
   56ef5:	lea    0x0(,%rsi,8),%edx
   56efc:	lea    (%r14,%rdx,1),%rax
   56f00:	mov    (%r14),%rcx
   56f03:	lea    0x8(%r14),%rdi
   56f07:	lea    -0x10(%rdx),%r9
   56f0b:	cmp    $0x18,%r9
   56f0f:	jae    56f19 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x469>
   56f11:	mov    %rcx,%r8
   56f14:	jmp    56fa0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4f0>
   56f19:	shr    $0x3,%r9
   56f1d:	inc    %r9
   56f20:	mov    %r9,%r10
   56f23:	and    $0xfffffffffffffffc,%r10
   56f27:	lea    (%rdi,%r10,8),%rdi
   56f2b:	movq   %rcx,%xmm2
   56f30:	shufpd $0x2,-0x4d139(%rip),%xmm2        # 9e00 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56f39:	movq   %rcx,%xmm1
   56f3e:	pxor   %xmm0,%xmm0
   56f42:	pcmpeqd %xmm3,%xmm3
   56f46:	xor    %ecx,%ecx
   56f48:	nopl   0x0(%rax,%rax,1)
   56f50:	movdqu 0x8(%r14,%rcx,8),%xmm4
   56f57:	movdqu 0x18(%r14,%rcx,8),%xmm5
   56f5e:	por    %xmm4,%xmm1
   56f62:	por    %xmm5,%xmm0
   56f66:	andpd  %xmm4,%xmm2
   56f6a:	pand   %xmm5,%xmm3
   56f6e:	add    $0x4,%rcx
   56f72:	cmp    %rcx,%r10
   56f75:	jne    56f50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4a0>
   56f77:	pand   %xmm2,%xmm3
   56f7b:	pshufd $0xee,%xmm3,%xmm2
   56f80:	pand   %xmm3,%xmm2
   56f84:	movq   %xmm2,%rcx
   56f89:	por    %xmm1,%xmm0
   56f8d:	pshufd $0xee,%xmm0,%xmm1
   56f92:	por    %xmm0,%xmm1
   56f96:	movq   %xmm1,%r8
   56f9b:	cmp    %r10,%r9
   56f9e:	je     56fb2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x502>
   56fa0:	mov    (%rdi),%r9
   56fa3:	or     %r9,%r8
   56fa6:	and    %r9,%rcx
   56fa9:	add    $0x8,%rdi
   56fad:	cmp    %rax,%rdi
   56fb0:	jne    56fa0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4f0>
   56fb2:	mov    %rcx,%r15
   56fb5:	xor    %r8,%r15
   56fb8:	cmp    $0xffffffffffffffff,%r15
   56fbc:	je     56ff4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x544>
   56fbe:	lea    0x0(,%rbx,8),%rdi
   56fc6:	cmp    %rdi,%rdx
   56fc9:	je     57405 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x955>
   56fcf:	not    %rsi
   56fd2:	add    %rbx,%rsi
   56fd5:	movabs $0x1fffffffffffffff,%rdi
   56fdf:	and    %rsi,%rdi
   56fe2:	cmp    $0x3,%rdi
   56fe6:	jae    5734f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x89f>
   56fec:	mov    %r8,%r15
   56fef:	jmp    573dd <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x92d>
   56ff4:	mov    $0xffffffffffffffff,%r15
   56ffb:	jmp    5740e <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x95e>
   57000:	inc    %rsi
   57003:	mov    %rsi,%rdi
   57006:	and    $0xfffffffffffffffc,%rdi
   5700a:	lea    (%rcx,%rdi,8),%rcx
   5700e:	movq   %r8,%xmm2
   57013:	shufpd $0x2,-0x4d21c(%rip),%xmm2        # 9e00 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   5701c:	movq   %r9,%xmm1
   57021:	add    %r14,%rdx
   57024:	add    $0x10,%rdx
   57028:	pxor   %xmm0,%xmm0
   5702c:	pcmpeqd %xmm3,%xmm3
   57030:	xor    %r8d,%r8d
   57033:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57040:	movdqu -0x10(%rdx,%r8,8),%xmm4
   57047:	movdqu (%rdx,%r8,8),%xmm5
   5704d:	por    %xmm4,%xmm1
   57051:	por    %xmm5,%xmm0
   57055:	andpd  %xmm4,%xmm2
   57059:	pand   %xmm5,%xmm3
   5705d:	add    $0x4,%r8
   57061:	cmp    %r8,%rdi
   57064:	jne    57040 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x590>
   57066:	pand   %xmm2,%xmm3
   5706a:	pshufd $0xee,%xmm3,%xmm2
   5706f:	pand   %xmm3,%xmm2
   57073:	movq   %xmm2,%r8
   57078:	por    %xmm1,%xmm0
   5707c:	pshufd $0xee,%xmm0,%xmm1
   57081:	por    %xmm0,%xmm1
   57085:	movq   %xmm1,%r15
   5708a:	cmp    %rdi,%rsi
   5708d:	je     570b2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x602>
   5708f:	lea    (%r14,%rbx,8),%rdx
   57093:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   570a0:	mov    (%rcx),%rsi
   570a3:	add    $0x8,%rcx
   570a7:	or     %rsi,%r15
   570aa:	and    %rsi,%r8
   570ad:	cmp    %rdx,%rcx
   570b0:	jne    570a0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x5f0>
   570b2:	xor    %r8,%r15
   570b5:	test   %r15,%r15
   570b8:	je     56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   570be:	lea    0x0(,%rbx,8),%r12
   570c6:	lea    (%r14,%rbx,8),%rcx
   570ca:	lea    0x10(%r14),%rdx
   570ce:	lea    -0x8(,%rbx,8),%rsi
   570d6:	cs nopw 0x0(%rax,%rax,1)
   570e0:	mov    -0x8(%rdx),%rdi
   570e4:	cmp    %rax,%rdi
   570e7:	jne    570f8 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x648>
   570e9:	add    $0x8,%rdx
   570ed:	add    $0xfffffffffffffff8,%rsi
   570f1:	jne    570e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x630>
   570f3:	jmp    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   570f8:	movq   $0x1,0x8(%rsp)
   57101:	mov    %rdi,0x10(%rsp)
   57106:	mov    %rdx,0x18(%rsp)
   5710b:	mov    %rcx,0x20(%rsp)
   57110:	lea    0x8(%rsp),%rdi
   57115:	jbe    57129 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x679>
   57117:	call   89700 <_RINvYINtNtNtNtCsdLJKRdCaWyW_4core4iter8adapters5chain5ChainINtNtNtBa_7sources4once4OnceyEINtNtB8_3map3MapINtNtNtBc_5slice4iter4IteryENvYyNtNtNtCs7jsxoleoEt5_16sort_research_rs5other13radix_support3Key3keyEENtNtNtBa_6traits8iterator8Iterator12is_sorted_byNCNvYB3_B3i_9is_sorted0ECs59F9KgNujep_8radix_ab>
   5711c:	test   %al,%al
   5711e:	jne    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   57124:	jmp    5740e <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x95e>
   57129:	call   89690 <_RINvYINtNtNtNtCsdLJKRdCaWyW_4core4iter8adapters5chain5ChainINtNtNtBa_7sources4once4OnceyEINtNtB8_3map3MapINtNtNtBc_5slice4iter4IteryENvYyNtNtNtCs7jsxoleoEt5_16sort_research_rs5other13radix_support3Key3keyEENtNtNtBa_6traits8iterator8Iterator12is_sorted_byNCINvNtNtB2f_18rust_tessera_radix6engine13confirm_orderyEs_0ECs59F9KgNujep_8radix_ab>
   5712e:	test   %al,%al
   57130:	je     5740e <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x95e>
   57136:	mov    %rbx,%rax
   57139:	shr    $1,%rax
   5713c:	cmp    $0x8,%rbx
   57140:	jae    57460 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x9b0>
   57146:	xor    %ecx,%ecx
   57148:	jmp    574e2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa32>
   5714d:	movabs $0x7fffffffffffffc,%rdx
   57157:	and    %rax,%rdx
   5715a:	mov    %rax,%rsi
   5715d:	and    $0xfffffffffffffffc,%rsi
   57161:	neg    %rsi
   57164:	lea    0x10(%r14),%rdi
   57168:	lea    (%rcx,%r14,1),%r8
   5716c:	add    $0xfffffffffffffff0,%r8
   57170:	xor    %r9d,%r9d
   57173:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
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
   571cd:	cmp    %rdx,%rax
   571d0:	je     5720a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x75a>
   571d2:	neg    %rax
   571d5:	lea    (%r14,%rdx,8),%rsi
   571d9:	neg    %rdx
   571dc:	add    %r14,%rcx
   571df:	add    $0xfffffffffffffff8,%rcx
   571e3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   571f0:	mov    (%rsi),%rdi
   571f3:	mov    (%rcx,%rdx,8),%r8
   571f7:	mov    %r8,(%rsi)
   571fa:	mov    %rdi,(%rcx,%rdx,8)
   571fe:	dec    %rdx
   57201:	add    $0x8,%rsi
   57205:	cmp    %rdx,%rax
   57208:	jne    571f0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x740>
   5720a:	movabs $0x7ffffffffffffffc,%rax
   57214:	lea    0x10(%r14),%rcx
   57218:	lea    -0x10(%r14),%rdx
   5721c:	lea    -0x8(%r14),%r8
   57220:	xor    %esi,%esi
   57222:	jmp    57239 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x789>
   57224:	data16 data16 cs nopw 0x0(%rax,%rax,1)
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
   57287:	cmp    $0x8,%r10
   5728b:	jae    572a0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7f0>
   5728d:	xor    %r10d,%r10d
   57290:	jmp    5731a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x86a>
   57295:	data16 cs nopw 0x0(%rax,%rax,1)
   572a0:	mov    %r9,%r10
   572a3:	and    %rax,%r10
   572a6:	mov    %r9,%r11
   572a9:	and    $0xfffffffffffffffc,%r11
   572ad:	neg    %r11
   572b0:	lea    (%rcx,%rdi,8),%r15
   572b4:	lea    (%rdx,%rsi,8),%r12
   572b8:	xor    %r13d,%r13d
   572bb:	nopl   0x0(%rax,%rax,1)
   572c0:	movdqu -0x10(%r15),%xmm0
   572c6:	movdqu (%r15),%xmm1
   572cb:	movdqu -0x10(%r12,%r13,8),%xmm2
   572d2:	movdqu (%r12,%r13,8),%xmm3
   572d8:	pshufd $0x4e,%xmm3,%xmm3
   572dd:	pshufd $0x4e,%xmm2,%xmm2
   572e2:	movdqu %xmm3,-0x10(%r15)
   572e8:	movdqu %xmm2,(%r15)
   572ed:	pshufd $0x4e,%xmm0,%xmm0
   572f2:	pshufd $0x4e,%xmm1,%xmm1
   572f7:	movdqu %xmm0,(%r12,%r13,8)
   572fd:	movdqu %xmm1,-0x10(%r12,%r13,8)
   57304:	add    $0xfffffffffffffffc,%r13
   57308:	add    $0x20,%r15
   5730c:	cmp    %r13,%r11
   5730f:	jne    572c0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x810>
   57311:	cmp    %r10,%r9
   57314:	je     57230 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x780>
   5731a:	lea    (%r14,%rdi,8),%rdi
   5731e:	lea    0x0(,%r10,8),%r15
   57326:	lea    (%r8,%rsi,8),%r11
   5732a:	sub    %r15,%r11
   5732d:	nopl   (%rax)
   57330:	mov    (%rdi,%r10,8),%r15
   57334:	mov    (%r11),%r12
   57337:	mov    %r12,(%rdi,%r10,8)
   5733b:	mov    %r15,(%r11)
   5733e:	inc    %r10
   57341:	add    $0xfffffffffffffff8,%r11
   57345:	cmp    %r10,%r9
   57348:	jne    57330 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x880>
   5734a:	jmp    57230 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x780>
   5734f:	inc    %rdi
   57352:	mov    %rdi,%rsi
   57355:	and    $0xfffffffffffffffc,%rsi
   57359:	lea    (%rax,%rsi,8),%rax
   5735d:	movq   %rcx,%xmm2
   57362:	shufpd $0x2,-0x4d56b(%rip),%xmm2        # 9e00 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   5736b:	movq   %r8,%xmm1
   57370:	lea    (%rdx,%r14,1),%rcx
   57374:	add    $0x10,%rcx
   57378:	pxor   %xmm0,%xmm0
   5737c:	pcmpeqd %xmm3,%xmm3
   57380:	xor    %edx,%edx
   57382:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57390:	movdqu -0x10(%rcx,%rdx,8),%xmm4
   57396:	movdqu (%rcx,%rdx,8),%xmm5
   5739b:	por    %xmm4,%xmm1
   5739f:	por    %xmm5,%xmm0
   573a3:	andpd  %xmm4,%xmm2
   573a7:	pand   %xmm5,%xmm3
   573ab:	add    $0x4,%rdx
   573af:	cmp    %rdx,%rsi
   573b2:	jne    57390 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x8e0>
   573b4:	pand   %xmm2,%xmm3
   573b8:	pshufd $0xee,%xmm3,%xmm2
   573bd:	pand   %xmm3,%xmm2
   573c1:	movq   %xmm2,%rcx
   573c6:	por    %xmm1,%xmm0
   573ca:	pshufd $0xee,%xmm0,%xmm1
   573cf:	por    %xmm0,%xmm1
   573d3:	movq   %xmm1,%r15
   573d8:	cmp    %rsi,%rdi
   573db:	je     57402 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x952>
   573dd:	lea    (%r14,%rbx,8),%rdx
   573e1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   573f0:	mov    (%rax),%rsi
   573f3:	add    $0x8,%rax
   573f7:	or     %rsi,%r15
   573fa:	and    %rsi,%rcx
   573fd:	cmp    %rdx,%rax
   57400:	jne    573f0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x940>
   57402:	xor    %rcx,%r15
   57405:	test   %r15,%r15
   57408:	je     56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   5740e:	movq   $0x0,0x8(%rsp)
   57417:	movq   $0x8,0x10(%rsp)
   57420:	movq   $0x0,0x18(%rsp)
   57429:	mov    %r13,0x20(%rsp)
   5742e:	lea    0x8(%rsp),%rdi
   57433:	mov    %r14,%rsi
   57436:	mov    %rbx,%rdx
   57439:	mov    %r15,%rcx
   5743c:	xor    %r8d,%r8d
   5743f:	call   661e0 <_RNvMs0_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engineINtB5_6SorteryE9sort_nodeCs59F9KgNujep_8radix_ab>
   57444:	cmpq   $0x0,0x8(%rsp)
   5744a:	je     56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   57450:	mov    0x10(%rsp),%rdi
   57455:	call   *0x835fd(%rip)        # daa58 <free@GLIBC_2.2.5>
   5745b:	jmp    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   57460:	movabs $0x7fffffffffffffc,%rcx
   5746a:	and    %rax,%rcx
   5746d:	mov    %rax,%rdx
   57470:	and    $0xfffffffffffffffc,%rdx
   57474:	neg    %rdx
   57477:	lea    0x10(%r14),%rsi
   5747b:	lea    (%r12,%r14,1),%rdi
   5747f:	add    $0xfffffffffffffff0,%rdi
   57483:	xor    %r8d,%r8d
   57486:	cs nopw 0x0(%rax,%rax,1)
   57490:	movdqu -0x10(%rsi),%xmm0
   57495:	movdqu (%rsi),%xmm1
   57499:	movdqu -0x10(%rdi,%r8,8),%xmm2
   574a0:	movdqu (%rdi,%r8,8),%xmm3
   574a6:	pshufd $0x4e,%xmm3,%xmm3
   574ab:	pshufd $0x4e,%xmm2,%xmm2
   574b0:	movdqu %xmm3,-0x10(%rsi)
   574b5:	movdqu %xmm2,(%rsi)
   574b9:	pshufd $0x4e,%xmm0,%xmm0
   574be:	pshufd $0x4e,%xmm1,%xmm1
   574c3:	movdqu %xmm0,(%rdi,%r8,8)
   574c9:	movdqu %xmm1,-0x10(%rdi,%r8,8)
   574d0:	add    $0xfffffffffffffffc,%r8
   574d4:	add    $0x20,%rsi
   574d8:	cmp    %r8,%rdx
   574db:	jne    57490 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x9e0>
   574dd:	cmp    %rcx,%rax
   574e0:	je     5751a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa6a>
   574e2:	neg    %rax
   574e5:	lea    (%r14,%rcx,8),%rdx
   574e9:	neg    %rcx
   574ec:	lea    (%r12,%r14,1),%rsi
   574f0:	add    $0xfffffffffffffff8,%rsi
   574f4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57500:	mov    (%rdx),%rdi
   57503:	mov    (%rsi,%rcx,8),%r8
   57507:	mov    %r8,(%rdx)
   5750a:	mov    %rdi,(%rsi,%rcx,8)
   5750e:	dec    %rcx
   57511:	add    $0x8,%rdx
   57515:	cmp    %rcx,%rax
   57518:	jne    57500 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa50>
   5751a:	movabs $0x7ffffffffffffffc,%rax
   57524:	lea    0x10(%r14),%rcx
   57528:	lea    -0x10(%r14),%rdx
   5752c:	lea    -0x8(%r14),%r8
   57530:	xor    %esi,%esi
   57532:	jmp    57549 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa99>
   57534:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57540:	cmp    %rbx,%rsi
   57543:	jae    56ac4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   57549:	mov    %rsi,%rdi
   5754c:	lea    0x1(%rsi),%r9
   57550:	cmp    %r9,%rbx
   57553:	mov    %r9,%rsi
   57556:	cmova  %rbx,%rsi
   5755a:	lea    -0x1(%rsi),%r10
   5755e:	cmp    %r10,%rdi
   57561:	je     57583 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xad3>
   57563:	mov    (%r14,%rdi,8),%r10
   57567:	nopw   0x0(%rax,%rax,1)
   57570:	cmp    %r10,(%r14,%r9,8)
   57574:	jne    57580 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xad0>
   57576:	inc    %r9
   57579:	cmp    %r9,%rsi
   5757c:	jne    57570 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xac0>
   5757e:	jmp    57583 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xad3>
   57580:	mov    %r9,%rsi
   57583:	mov    %rsi,%r10
   57586:	sub    %rdi,%r10
   57589:	jb     5764f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb9f>
   5758f:	mov    %r10,%r9
   57592:	shr    $1,%r9
   57595:	je     57540 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa90>
   57597:	cmp    $0x8,%r10
   5759b:	jae    575a2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xaf2>
   5759d:	xor    %r10d,%r10d
   575a0:	jmp    5761a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb6a>
   575a2:	mov    %r9,%r10
   575a5:	and    %rax,%r10
   575a8:	mov    %r9,%r11
   575ab:	and    $0xfffffffffffffffc,%r11
   575af:	neg    %r11
   575b2:	lea    (%rcx,%rdi,8),%r15
   575b6:	lea    (%rdx,%rsi,8),%r12
   575ba:	xor    %r13d,%r13d
   575bd:	nopl   (%rax)
   575c0:	movdqu -0x10(%r15),%xmm0
   575c6:	movdqu (%r15),%xmm1
   575cb:	movdqu -0x10(%r12,%r13,8),%xmm2
   575d2:	movdqu (%r12,%r13,8),%xmm3
   575d8:	pshufd $0x4e,%xmm3,%xmm3
   575dd:	pshufd $0x4e,%xmm2,%xmm2
   575e2:	movdqu %xmm3,-0x10(%r15)
   575e8:	movdqu %xmm2,(%r15)
   575ed:	pshufd $0x4e,%xmm0,%xmm0
   575f2:	pshufd $0x4e,%xmm1,%xmm1
   575f7:	movdqu %xmm0,(%r12,%r13,8)
   575fd:	movdqu %xmm1,-0x10(%r12,%r13,8)
   57604:	add    $0xfffffffffffffffc,%r13
   57608:	add    $0x20,%r15
   5760c:	cmp    %r13,%r11
   5760f:	jne    575c0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb10>
   57611:	cmp    %r10,%r9
   57614:	je     57540 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa90>
   5761a:	lea    (%r14,%rdi,8),%rdi
   5761e:	lea    0x0(,%r10,8),%r15
   57626:	lea    (%r8,%rsi,8),%r11
   5762a:	sub    %r15,%r11
   5762d:	nopl   (%rax)
   57630:	mov    (%rdi,%r10,8),%r15
   57634:	mov    (%r11),%r12
   57637:	mov    %r12,(%rdi,%r10,8)
   5763b:	mov    %r15,(%r11)
   5763e:	inc    %r10
   57641:	add    $0xfffffffffffffff8,%r11
   57645:	cmp    %r10,%r9
   57648:	jne    57630 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb80>
   5764a:	jmp    57540 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa90>
   5764f:	lea    0x80e7a(%rip),%rcx        # d84d0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x810>
   57656:	mov    %rbx,%rdx
   57659:	call   *0x83471(%rip)        # daad0 <_DYNAMIC+0x280>
   5765f:	lea    -0x4aa85(%rip),%rdi        # cbe1 <_RNvNtCs5PduvXNAoUW_12panic_unwind3imp6CANARY.llvm.6315980824996689386+0x1f2>
   57666:	lea    0x80ec3(%rip),%rdx        # d8530 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x870>
   5766d:	mov    $0x13,%esi
   57672:	call   *0x83488(%rip)        # dab00 <_DYNAMIC+0x2b0>
   57678:	jmp    576cb <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xc1b>
   5767a:	lea    0x80ec7(%rip),%rdi        # d8548 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x888>
   57681:	call   *0x835b1(%rip)        # dac38 <_DYNAMIC+0x3e8>
   57687:	jmp    576cb <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xc1b>
   57689:	lea    0x80df8(%rip),%rdx        # d8488 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7c8>
   57690:	mov    %rbx,%rsi
   57693:	call   *0x83407(%rip)        # daaa0 <_DYNAMIC+0x250>
   57699:	lea    0x80dd0(%rip),%rdx        # d8470 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7b0>
   576a0:	mov    %rax,%rdi
   576a3:	mov    %rbx,%rsi
   576a6:	call   *0x833f4(%rip)        # daaa0 <_DYNAMIC+0x250>
   576ac:	mov    $0x8,%edi
   576b1:	mov    %r12,%rsi
   576b4:	call   *0x833d6(%rip)        # daa90 <_DYNAMIC+0x240>
   576ba:	lea    0x80e9f(%rip),%rdx        # d8560 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x8a0>
   576c1:	xor    %edi,%edi
   576c3:	xor    %esi,%esi
   576c5:	call   *0x833d5(%rip)        # daaa0 <_DYNAMIC+0x250>
   576cb:	ud2
   576cd:	mov    %rax,%rbx
   576d0:	cmpq   $0x0,0x8(%rsp)
   576d6:	je     576eb <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xc3b>
   576d8:	mov    0x10(%rsp),%rdi
   576dd:	jmp    576e5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xc35>
   576df:	mov    %rax,%rbx
   576e2:	mov    %r15,%rdi
   576e5:	call   *0x8336d(%rip)        # daa58 <free@GLIBC_2.2.5>
   576eb:	mov    %rbx,%rdi
   576ee:	call   d60d0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
