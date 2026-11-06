
logs/tessera-cleanup/unchecked-probe-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056970 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab>:
   56970:	push   %rbp
   56971:	push   %r15
   56973:	push   %r14
   56975:	push   %r13
   56977:	push   %r12
   56979:	push   %rbx
   5697a:	sub    $0x28,%rsp
   5697e:	cmp    $0x2,%rsi
   56982:	jae    56993 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x23>
   56984:	add    $0x28,%rsp
   56988:	pop    %rbx
   56989:	pop    %r12
   5698b:	pop    %r13
   5698d:	pop    %r14
   5698f:	pop    %r15
   56991:	pop    %rbp
   56992:	ret
   56993:	mov    %rsi,%rbx
   56996:	mov    %rdi,%r14
   56999:	cmp    $0x9,%rdx
   5699d:	mov    $0x8,%eax
   569a2:	cmovae %rdx,%rax
   569a6:	cmp    $0x7fff8,%rax
   569ac:	mov    $0x7fff8,%r12d
   569b2:	cmovb  %rax,%r12
   569b6:	mov    %rsi,%rcx
   569b9:	shr    $0xc,%rcx
   569bd:	cmp    $0x1,%rcx
   569c1:	adc    $0x0,%rcx
   569c5:	xor    %eax,%eax
   569c7:	mov    %rsi,%rdx
   569ca:	sub    %rcx,%rdx
   569cd:	cmovae %rdx,%rax
   569d1:	mov    %rax,%rdx
   569d4:	or     %rcx,%rdx
   569d7:	shr    $0x20,%rdx
   569db:	je     569e4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x74>
   569dd:	xor    %edx,%edx
   569df:	div    %rcx
   569e2:	jmp    569e8 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x78>
   569e4:	xor    %edx,%edx
   569e6:	div    %ecx
   569e8:	xor    %esi,%esi
   569ea:	test   %rdx,%rdx
   569ed:	setne  %sil
   569f1:	add    %rax,%rsi
   569f4:	je     56b95 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x225>
   569fa:	neg    %rsi
   569fd:	mov    $0x1,%al
   569ff:	mov    $0x1,%dil
   56a02:	mov    %rcx,%rdx
   56a05:	jmp    56a1c <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xac>
   56a07:	nopw   0x0(%rax,%rax,1)
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
   56a41:	mov    %r12,(%rsp)
   56a45:	mov    %r12,%rax
   56a48:	shr    $0x3,%rax
   56a4c:	cmp    $0x3,%rax
   56a50:	mov    $0x2,%r13d
   56a56:	cmovae %rax,%r13
   56a5a:	cmp    %rbx,%r13
   56a5d:	mov    %rbx,%rbp
   56a60:	cmovb  %r13,%rbp
   56a64:	lea    0x0(,%rbp,8),%r12
   56a6c:	mov    %r12,%rdi
   56a6f:	call   *0x83e13(%rip)        # da888 <malloc@GLIBC_2.2.5>
   56a75:	test   %rax,%rax
   56a78:	je     57529 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbb9>
   56a7e:	mov    %rax,%r15
   56a81:	mov    %rax,%rdi
   56a84:	mov    %r14,%rsi
   56a87:	mov    %r12,%rdx
   56a8a:	call   *0x83e18(%rip)        # da8a8 <memcpy@GLIBC_2.14>
   56a90:	mov    %rbp,%rax
   56a93:	neg    %rax
   56a96:	xor    %r12d,%r12d
   56a99:	xor    %ecx,%ecx
   56a9b:	lea    0x0(,%rcx,8),%rdx
   56aa3:	mov    (%r15,%rcx,8),%rsi
   56aa7:	inc    %rcx
   56aaa:	neg    %rdx
   56aad:	nopl   (%rax)
   56ab0:	mov    %rsi,%rdi
   56ab3:	mov    (%r15,%rcx,8),%rsi
   56ab7:	cmp    %rsi,%rdi
   56aba:	ja     56acd <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x15d>
   56abc:	inc    %rcx
   56abf:	add    $0xfffffffffffffff8,%rdx
   56ac3:	cmp    %rcx,%rbp
   56ac6:	jne    56ab0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x140>
   56ac8:	jmp    56b5c <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x1ec>
   56acd:	cmp    %rcx,%rbp
   56ad0:	jb     574ff <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb8f>
   56ad6:	test   %rcx,%rcx
   56ad9:	je     5751a <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbaa>
   56adf:	mov    %rbp,%rsi
   56ae2:	sub    %rcx,%rsi
   56ae5:	je     57537 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbc7>
   56aeb:	mov    -0x8(%r15,%rcx,8),%r8
   56af0:	mov    (%r15,%rcx,8),%rdi
   56af4:	mov    %rdi,-0x8(%r15,%rcx,8)
   56af9:	mov    %r8,(%r15,%rcx,8)
   56afd:	inc    %r12
   56b00:	cmp    %rbp,%r12
   56b03:	ja     56cfe <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x38e>
   56b09:	lea    (%r15,%rcx,8),%r8
   56b0d:	add    $0xfffffffffffffff8,%r8
   56b11:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56b20:	test   %rdx,%rdx
   56b23:	je     56b52 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x1e2>
   56b25:	mov    -0x8(%r8),%r9
   56b29:	cmp    %rdi,%r9
   56b2c:	jbe    56b52 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x1e2>
   56b2e:	mov    %rdi,-0x8(%r8)
   56b32:	mov    %r9,(%r8)
   56b35:	add    $0xfffffffffffffff8,%r8
   56b39:	add    $0x8,%rdx
   56b3d:	lea    (%rax,%r12,1),%r9
   56b41:	inc    %r9
   56b44:	inc    %r12
   56b47:	cmp    $0x1,%r9
   56b4b:	jne    56b20 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x1b0>
   56b4d:	jmp    56cfe <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x38e>
   56b52:	cmp    $0x2,%rsi
   56b56:	jae    56a9b <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x12b>
   56b5c:	mov    %r15,%rdi
   56b5f:	call   *0x83d33(%rip)        # da898 <free@GLIBC_2.2.5>
   56b65:	test   %r12,%r12
   56b68:	je     56cec <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x37c>
   56b6e:	imul   %rbx,%r12
   56b72:	mov    %r12,%rax
   56b75:	shr    $0x20,%rax
   56b79:	je     56eab <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x53b>
   56b7f:	mov    %r12,%rax
   56b82:	xor    %edx,%edx
   56b84:	div    %rbp
   56b87:	jmp    56eb2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x542>
   56b8c:	test   %dil,%al
   56b8f:	je     56cac <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x33c>
   56b95:	cmp    $0x401,%rbx
   56b9c:	mov    $0x401,%r8d
   56ba2:	cmovb  %rbx,%r8
   56ba6:	mov    (%r14),%rax
   56ba9:	lea    0x0(,%r8,8),%edi
   56bb1:	lea    (%r14,%rdi,1),%rsi
   56bb5:	lea    0x8(%r14),%rdx
   56bb9:	lea    -0x10(%rdi),%r11
   56bbd:	mov    %rdx,%rcx
   56bc0:	mov    %rax,%r9
   56bc3:	mov    %rax,%r10
   56bc6:	cmp    $0x18,%r11
   56bca:	jb     56c50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2e0>
   56bd0:	shr    $0x3,%r11
   56bd4:	inc    %r11
   56bd7:	mov    %r11,%r15
   56bda:	and    $0xfffffffffffffffc,%r15
   56bde:	lea    (%rdx,%r15,8),%rcx
   56be2:	movq   %rax,%xmm2
   56be7:	shufpd $0x2,-0x4ce30(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56bf0:	movq   %rax,%xmm1
   56bf5:	pxor   %xmm0,%xmm0
   56bf9:	pcmpeqd %xmm3,%xmm3
   56bfd:	xor    %r9d,%r9d
   56c00:	movdqu 0x8(%r14,%r9,8),%xmm4
   56c07:	movdqu 0x18(%r14,%r9,8),%xmm5
   56c0e:	por    %xmm4,%xmm1
   56c12:	por    %xmm5,%xmm0
   56c16:	andpd  %xmm4,%xmm2
   56c1a:	pand   %xmm5,%xmm3
   56c1e:	add    $0x4,%r9
   56c22:	cmp    %r9,%r15
   56c25:	jne    56c00 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x290>
   56c27:	pand   %xmm2,%xmm3
   56c2b:	pshufd $0xee,%xmm3,%xmm2
   56c30:	pand   %xmm3,%xmm2
   56c34:	movq   %xmm2,%r9
   56c39:	por    %xmm1,%xmm0
   56c3d:	pshufd $0xee,%xmm0,%xmm1
   56c42:	por    %xmm0,%xmm1
   56c46:	movq   %xmm1,%r10
   56c4b:	cmp    %r15,%r11
   56c4e:	je     56c62 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2f2>
   56c50:	mov    (%rcx),%r11
   56c53:	or     %r11,%r10
   56c56:	and    %r11,%r9
   56c59:	add    $0x8,%rcx
   56c5d:	cmp    %rsi,%rcx
   56c60:	jne    56c50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x2e0>
   56c62:	mov    %r9,%rcx
   56c65:	xor    %r10,%rcx
   56c68:	cmp    $0xffffffffffffffff,%rcx
   56c6c:	je     56ca0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x330>
   56c6e:	lea    0x0(,%rbx,8),%r11
   56c76:	cmp    %r11,%rdi
   56c79:	je     56dc5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x455>
   56c7f:	not    %r8
   56c82:	add    %rbx,%r8
   56c85:	movabs $0x1fffffffffffffff,%r11
   56c8f:	and    %r8,%r11
   56c92:	cmp    $0x3,%r11
   56c96:	jae    56d10 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3a0>
   56c98:	mov    %r10,%rcx
   56c9b:	jmp    56d9d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x42d>
   56ca0:	mov    $0xffffffffffffffff,%rcx
   56ca7:	jmp    56dce <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x45e>
   56cac:	mov    (%r14),%rdx
   56caf:	lea    0x0(,%rbx,8),%r15
   56cb7:	lea    (%r14,%rbx,8),%rax
   56cbb:	lea    0x10(%r14),%rcx
   56cbf:	lea    -0x8(,%rbx,8),%rsi
   56cc7:	nopw   0x0(%rax,%rax,1)
   56cd0:	mov    -0x8(%rcx),%rdi
   56cd4:	cmp    %rdx,%rdi
   56cd7:	jne    56e76 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x506>
   56cdd:	add    $0x8,%rcx
   56ce1:	add    $0xfffffffffffffff8,%rsi
   56ce5:	jne    56cd0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x360>
   56ce7:	jmp    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56cec:	cmp    %r13,%rbx
   56cef:	mov    (%rsp),%r12
   56cf3:	jbe    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56cf9:	jmp    56ecd <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x55d>
   56cfe:	mov    %r15,%rdi
   56d01:	call   *0x83b91(%rip)        # da898 <free@GLIBC_2.2.5>
   56d07:	mov    (%rsp),%r12
   56d0b:	jmp    56ecd <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x55d>
   56d10:	inc    %r11
   56d13:	mov    %r11,%r8
   56d16:	and    $0xfffffffffffffffc,%r8
   56d1a:	lea    (%rsi,%r8,8),%rsi
   56d1e:	movq   %r9,%xmm2
   56d23:	shufpd $0x2,-0x4cf6c(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56d2c:	movq   %r10,%xmm1
   56d31:	lea    (%rdi,%r14,1),%rcx
   56d35:	add    $0x10,%rcx
   56d39:	pxor   %xmm0,%xmm0
   56d3d:	pcmpeqd %xmm3,%xmm3
   56d41:	xor    %edi,%edi
   56d43:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56d50:	movdqu -0x10(%rcx,%rdi,8),%xmm4
   56d56:	movdqu (%rcx,%rdi,8),%xmm5
   56d5b:	por    %xmm4,%xmm1
   56d5f:	por    %xmm5,%xmm0
   56d63:	andpd  %xmm4,%xmm2
   56d67:	pand   %xmm5,%xmm3
   56d6b:	add    $0x4,%rdi
   56d6f:	cmp    %rdi,%r8
   56d72:	jne    56d50 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x3e0>
   56d74:	pand   %xmm2,%xmm3
   56d78:	pshufd $0xee,%xmm3,%xmm2
   56d7d:	pand   %xmm3,%xmm2
   56d81:	movq   %xmm2,%r9
   56d86:	por    %xmm1,%xmm0
   56d8a:	pshufd $0xee,%xmm0,%xmm1
   56d8f:	por    %xmm0,%xmm1
   56d93:	movq   %xmm1,%rcx
   56d98:	cmp    %r8,%r11
   56d9b:	je     56dc2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x452>
   56d9d:	lea    (%r14,%rbx,8),%rdi
   56da1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56db0:	mov    (%rsi),%r8
   56db3:	add    $0x8,%rsi
   56db7:	or     %r8,%rcx
   56dba:	and    %r8,%r9
   56dbd:	cmp    %rdi,%rsi
   56dc0:	jne    56db0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x440>
   56dc2:	xor    %r9,%rcx
   56dc5:	test   %rcx,%rcx
   56dc8:	je     56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56dce:	lea    0x0(,%rbx,8),%rsi
   56dd6:	lea    -0x8(,%rbx,8),%r9
   56dde:	xor    %edi,%edi
   56de0:	mov    (%rdx,%rdi,1),%r8
   56de4:	cmp    %rax,%r8
   56de7:	jne    56df7 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x487>
   56de9:	add    $0x8,%rdi
   56ded:	cmp    %rdi,%r9
   56df0:	jne    56de0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x470>
   56df2:	jmp    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56df7:	lea    -0x10(%rsi),%rdx
   56dfb:	cmp    %r8,%rax
   56dfe:	jae    56e34 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4c4>
   56e00:	cmp    %rdi,%rdx
   56e03:	je     56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56e09:	add    $0xffffffffffffffe8,%rsi
   56e0d:	nopl   (%rax)
   56e10:	mov    %r8,%rax
   56e13:	mov    0x10(%r14,%rdi,1),%r8
   56e18:	cmp    %r8,%rax
   56e1b:	ja     56e26 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4b6>
   56e1d:	cmp    %rdi,%rsi
   56e20:	lea    0x8(%rdi),%rdi
   56e24:	jne    56e10 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4a0>
   56e26:	cmp    %r8,%rax
   56e29:	jbe    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56e2f:	jmp    572be <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x94e>
   56e34:	cmp    %rdi,%rdx
   56e37:	je     56e5f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4ef>
   56e39:	lea    -0x18(%rsi),%rax
   56e3d:	nopl   (%rax)
   56e40:	mov    %r8,%rdx
   56e43:	mov    0x10(%r14,%rdi,1),%r8
   56e48:	cmp    %r8,%rdx
   56e4b:	jb     56e56 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4e6>
   56e4d:	cmp    %rdi,%rax
   56e50:	lea    0x8(%rdi),%rdi
   56e54:	jne    56e40 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x4d0>
   56e56:	cmp    %r8,%rdx
   56e59:	jb     572be <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x94e>
   56e5f:	mov    %rbx,%rax
   56e62:	shr    $1,%rax
   56e65:	cmp    $0x8,%rbx
   56e69:	jae    57004 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x694>
   56e6f:	xor    %ecx,%ecx
   56e71:	jmp    57082 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x712>
   56e76:	movq   $0x1,0x8(%rsp)
   56e7f:	mov    %rdi,0x10(%rsp)
   56e84:	mov    %rcx,0x18(%rsp)
   56e89:	mov    %rax,0x20(%rsp)
   56e8e:	lea    0x8(%rsp),%rdi
   56e93:	jbe    56fe0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x670>
   56e99:	call   89570 <_RINvYINtNtNtNtCsdLJKRdCaWyW_4core4iter8adapters5chain5ChainINtNtNtBa_7sources4once4OnceyEINtNtB8_3map3MapINtNtNtBc_5slice4iter4IteryENvYyNtNtNtCs7jsxoleoEt5_16sort_research_rs5other13radix_support3Key3keyEENtNtNtBa_6traits8iterator8Iterator12is_sorted_byNCNvYB3_B3i_9is_sorted0ECs59F9KgNujep_8radix_ab>
   56e9e:	test   %al,%al
   56ea0:	jne    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56ea6:	jmp    56a41 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xd1>
   56eab:	mov    %r12d,%eax
   56eae:	xor    %edx,%edx
   56eb0:	div    %ebp
   56eb2:	mov    (%rsp),%r12
   56eb6:	mov    %r14,%rdi
   56eb9:	mov    %rbx,%rsi
   56ebc:	mov    %rax,%rdx
   56ebf:	call   580c0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine23budgeted_insertion_sortyyNvYyNtNtB6_13radix_support3Key3keyECs59F9KgNujep_8radix_ab>
   56ec4:	test   %rax,%rax
   56ec7:	jne    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   56ecd:	cmp    $0x401,%rbx
   56ed4:	mov    $0x401,%edi
   56ed9:	cmovb  %rbx,%rdi
   56edd:	lea    0x0(,%rdi,8),%esi
   56ee4:	lea    (%r14,%rsi,1),%rax
   56ee8:	mov    (%r14),%rdx
   56eeb:	lea    0x8(%r14),%rcx
   56eef:	lea    -0x10(%rsi),%r9
   56ef3:	cmp    $0x18,%r9
   56ef7:	jae    56f01 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x591>
   56ef9:	mov    %rdx,%r8
   56efc:	jmp    56f80 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x610>
   56f01:	shr    $0x3,%r9
   56f05:	inc    %r9
   56f08:	mov    %r9,%r10
   56f0b:	and    $0xfffffffffffffffc,%r10
   56f0f:	lea    (%rcx,%r10,8),%rcx
   56f13:	movq   %rdx,%xmm2
   56f18:	shufpd $0x2,-0x4d161(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56f21:	movq   %rdx,%xmm1
   56f26:	pxor   %xmm0,%xmm0
   56f2a:	pcmpeqd %xmm3,%xmm3
   56f2e:	xor    %edx,%edx
   56f30:	movdqu 0x8(%r14,%rdx,8),%xmm4
   56f37:	movdqu 0x18(%r14,%rdx,8),%xmm5
   56f3e:	por    %xmm4,%xmm1
   56f42:	por    %xmm5,%xmm0
   56f46:	andpd  %xmm4,%xmm2
   56f4a:	pand   %xmm5,%xmm3
   56f4e:	add    $0x4,%rdx
   56f52:	cmp    %rdx,%r10
   56f55:	jne    56f30 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x5c0>
   56f57:	pand   %xmm2,%xmm3
   56f5b:	pshufd $0xee,%xmm3,%xmm2
   56f60:	pand   %xmm3,%xmm2
   56f64:	movq   %xmm2,%rdx
   56f69:	por    %xmm1,%xmm0
   56f6d:	pshufd $0xee,%xmm0,%xmm1
   56f72:	por    %xmm0,%xmm1
   56f76:	movq   %xmm1,%r8
   56f7b:	cmp    %r10,%r9
   56f7e:	je     56f92 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x622>
   56f80:	mov    (%rcx),%r9
   56f83:	or     %r9,%r8
   56f86:	and    %r9,%rdx
   56f89:	add    $0x8,%rcx
   56f8d:	cmp    %rax,%rcx
   56f90:	jne    56f80 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x610>
   56f92:	mov    %rdx,%rcx
   56f95:	xor    %r8,%rcx
   56f98:	cmp    $0xffffffffffffffff,%rcx
   56f9c:	je     56fd4 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x664>
   56f9e:	lea    0x0(,%rbx,8),%r9
   56fa6:	cmp    %r9,%rsi
   56fa9:	je     572b5 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x945>
   56faf:	not    %rdi
   56fb2:	add    %rbx,%rdi
   56fb5:	movabs $0x1fffffffffffffff,%r9
   56fbf:	and    %rdi,%r9
   56fc2:	cmp    $0x3,%r9
   56fc6:	jae    571ff <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x88f>
   56fcc:	mov    %r8,%rcx
   56fcf:	jmp    5728d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x91d>
   56fd4:	mov    $0xffffffffffffffff,%rcx
   56fdb:	jmp    572be <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x94e>
   56fe0:	call   89500 <_RINvYINtNtNtNtCsdLJKRdCaWyW_4core4iter8adapters5chain5ChainINtNtNtBa_7sources4once4OnceyEINtNtB8_3map3MapINtNtNtBc_5slice4iter4IteryENvYyNtNtNtCs7jsxoleoEt5_16sort_research_rs5other13radix_support3Key3keyEENtNtNtBa_6traits8iterator8Iterator12is_sorted_byNCINvNtNtB2f_18rust_tessera_radix6engine13confirm_orderyEs_0ECs59F9KgNujep_8radix_ab>
   56fe5:	test   %al,%al
   56fe7:	je     56a41 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xd1>
   56fed:	mov    %rbx,%rax
   56ff0:	shr    $1,%rax
   56ff3:	cmp    $0x8,%rbx
   56ff7:	jae    5730d <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x99d>
   56ffd:	xor    %ecx,%ecx
   56fff:	jmp    57385 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa15>
   57004:	movabs $0x7fffffffffffffc,%rcx
   5700e:	and    %rax,%rcx
   57011:	mov    %rax,%rdx
   57014:	and    $0xfffffffffffffffc,%rdx
   57018:	neg    %rdx
   5701b:	lea    0x10(%r14),%rdi
   5701f:	lea    (%rsi,%r14,1),%r8
   57023:	add    $0xfffffffffffffff0,%r8
   57027:	xor    %r9d,%r9d
   5702a:	nopw   0x0(%rax,%rax,1)
   57030:	movdqu -0x10(%rdi),%xmm0
   57035:	movdqu (%rdi),%xmm1
   57039:	movdqu -0x10(%r8,%r9,8),%xmm2
   57040:	movdqu (%r8,%r9,8),%xmm3
   57046:	pshufd $0x4e,%xmm3,%xmm3
   5704b:	pshufd $0x4e,%xmm2,%xmm2
   57050:	movdqu %xmm3,-0x10(%rdi)
   57055:	movdqu %xmm2,(%rdi)
   57059:	pshufd $0x4e,%xmm0,%xmm0
   5705e:	pshufd $0x4e,%xmm1,%xmm1
   57063:	movdqu %xmm0,(%r8,%r9,8)
   57069:	movdqu %xmm1,-0x10(%r8,%r9,8)
   57070:	add    $0xfffffffffffffffc,%r9
   57074:	add    $0x20,%rdi
   57078:	cmp    %r9,%rdx
   5707b:	jne    57030 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x6c0>
   5707d:	cmp    %rcx,%rax
   57080:	je     570ba <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x74a>
   57082:	neg    %rax
   57085:	lea    (%r14,%rcx,8),%rdx
   57089:	neg    %rcx
   5708c:	add    %r14,%rsi
   5708f:	add    $0xfffffffffffffff8,%rsi
   57093:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   570a0:	mov    (%rdx),%rdi
   570a3:	mov    (%rsi,%rcx,8),%r8
   570a7:	mov    %r8,(%rdx)
   570aa:	mov    %rdi,(%rsi,%rcx,8)
   570ae:	dec    %rcx
   570b1:	add    $0x8,%rdx
   570b5:	cmp    %rcx,%rax
   570b8:	jne    570a0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x730>
   570ba:	movabs $0x7ffffffffffffffc,%rax
   570c4:	lea    0x10(%r14),%rcx
   570c8:	lea    -0x10(%r14),%rdx
   570cc:	lea    -0x8(%r14),%r8
   570d0:	xor    %esi,%esi
   570d2:	jmp    570e9 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x779>
   570d4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   570e0:	cmp    %rbx,%rsi
   570e3:	jae    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   570e9:	mov    %rsi,%rdi
   570ec:	lea    0x1(%rsi),%r9
   570f0:	cmp    %r9,%rbx
   570f3:	mov    %r9,%rsi
   570f6:	cmova  %rbx,%rsi
   570fa:	lea    -0x1(%rsi),%r10
   570fe:	cmp    %r10,%rdi
   57101:	je     57123 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7b3>
   57103:	mov    (%r14,%rdi,8),%r10
   57107:	nopw   0x0(%rax,%rax,1)
   57110:	cmp    %r10,(%r14,%r9,8)
   57114:	jne    57120 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7b0>
   57116:	inc    %r9
   57119:	cmp    %r9,%rsi
   5711c:	jne    57110 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7a0>
   5711e:	jmp    57123 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7b3>
   57120:	mov    %r9,%rsi
   57123:	mov    %rsi,%r10
   57126:	sub    %rdi,%r10
   57129:	jb     574ef <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb7f>
   5712f:	mov    %r10,%r9
   57132:	shr    $1,%r9
   57135:	je     570e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x770>
   57137:	cmp    $0x8,%r10
   5713b:	jae    57150 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x7e0>
   5713d:	xor    %r10d,%r10d
   57140:	jmp    571ca <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x85a>
   57145:	data16 cs nopw 0x0(%rax,%rax,1)
   57150:	mov    %r9,%r10
   57153:	and    %rax,%r10
   57156:	mov    %r9,%r11
   57159:	and    $0xfffffffffffffffc,%r11
   5715d:	neg    %r11
   57160:	lea    (%rcx,%rdi,8),%r15
   57164:	lea    (%rdx,%rsi,8),%r12
   57168:	xor    %r13d,%r13d
   5716b:	nopl   0x0(%rax,%rax,1)
   57170:	movdqu -0x10(%r15),%xmm0
   57176:	movdqu (%r15),%xmm1
   5717b:	movdqu -0x10(%r12,%r13,8),%xmm2
   57182:	movdqu (%r12,%r13,8),%xmm3
   57188:	pshufd $0x4e,%xmm3,%xmm3
   5718d:	pshufd $0x4e,%xmm2,%xmm2
   57192:	movdqu %xmm3,-0x10(%r15)
   57198:	movdqu %xmm2,(%r15)
   5719d:	pshufd $0x4e,%xmm0,%xmm0
   571a2:	pshufd $0x4e,%xmm1,%xmm1
   571a7:	movdqu %xmm0,(%r12,%r13,8)
   571ad:	movdqu %xmm1,-0x10(%r12,%r13,8)
   571b4:	add    $0xfffffffffffffffc,%r13
   571b8:	add    $0x20,%r15
   571bc:	cmp    %r13,%r11
   571bf:	jne    57170 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x800>
   571c1:	cmp    %r10,%r9
   571c4:	je     570e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x770>
   571ca:	lea    (%r14,%rdi,8),%rdi
   571ce:	lea    0x0(,%r10,8),%r15
   571d6:	lea    (%r8,%rsi,8),%r11
   571da:	sub    %r15,%r11
   571dd:	nopl   (%rax)
   571e0:	mov    (%rdi,%r10,8),%r15
   571e4:	mov    (%r11),%r12
   571e7:	mov    %r12,(%rdi,%r10,8)
   571eb:	mov    %r15,(%r11)
   571ee:	inc    %r10
   571f1:	add    $0xfffffffffffffff8,%r11
   571f5:	cmp    %r10,%r9
   571f8:	jne    571e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x870>
   571fa:	jmp    570e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x770>
   571ff:	inc    %r9
   57202:	mov    %r9,%rdi
   57205:	and    $0xfffffffffffffffc,%rdi
   57209:	lea    (%rax,%rdi,8),%rax
   5720d:	movq   %rdx,%xmm2
   57212:	shufpd $0x2,-0x4d45b(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   5721b:	movq   %r8,%xmm1
   57220:	lea    (%rsi,%r14,1),%rcx
   57224:	add    $0x10,%rcx
   57228:	pxor   %xmm0,%xmm0
   5722c:	pcmpeqd %xmm3,%xmm3
   57230:	xor    %edx,%edx
   57232:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57240:	movdqu -0x10(%rcx,%rdx,8),%xmm4
   57246:	movdqu (%rcx,%rdx,8),%xmm5
   5724b:	por    %xmm4,%xmm1
   5724f:	por    %xmm5,%xmm0
   57253:	andpd  %xmm4,%xmm2
   57257:	pand   %xmm5,%xmm3
   5725b:	add    $0x4,%rdx
   5725f:	cmp    %rdx,%rdi
   57262:	jne    57240 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x8d0>
   57264:	pand   %xmm2,%xmm3
   57268:	pshufd $0xee,%xmm3,%xmm2
   5726d:	pand   %xmm3,%xmm2
   57271:	movq   %xmm2,%rdx
   57276:	por    %xmm1,%xmm0
   5727a:	pshufd $0xee,%xmm0,%xmm1
   5727f:	por    %xmm0,%xmm1
   57283:	movq   %xmm1,%rcx
   57288:	cmp    %rdi,%r9
   5728b:	je     572b2 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x942>
   5728d:	lea    (%r14,%rbx,8),%rsi
   57291:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   572a0:	mov    (%rax),%rdi
   572a3:	add    $0x8,%rax
   572a7:	or     %rdi,%rcx
   572aa:	and    %rdi,%rdx
   572ad:	cmp    %rsi,%rax
   572b0:	jne    572a0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x930>
   572b2:	xor    %rdx,%rcx
   572b5:	test   %rcx,%rcx
   572b8:	je     56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   572be:	movq   $0x0,0x8(%rsp)
   572c7:	movq   $0x8,0x10(%rsp)
   572d0:	movq   $0x0,0x18(%rsp)
   572d9:	mov    %r12,0x20(%rsp)
   572de:	lea    0x8(%rsp),%rdi
   572e3:	mov    %r14,%rsi
   572e6:	mov    %rbx,%rdx
   572e9:	xor    %r8d,%r8d
   572ec:	call   66050 <_RNvMs0_NtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engineINtB5_6SorteryE9sort_nodeCs59F9KgNujep_8radix_ab>
   572f1:	cmpq   $0x0,0x8(%rsp)
   572f7:	je     56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   572fd:	mov    0x10(%rsp),%rdi
   57302:	call   *0x83590(%rip)        # da898 <free@GLIBC_2.2.5>
   57308:	jmp    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   5730d:	movabs $0x7fffffffffffffc,%rcx
   57317:	and    %rax,%rcx
   5731a:	mov    %rax,%rdx
   5731d:	and    $0xfffffffffffffffc,%rdx
   57321:	neg    %rdx
   57324:	lea    0x10(%r14),%rsi
   57328:	lea    (%r15,%r14,1),%rdi
   5732c:	add    $0xfffffffffffffff0,%rdi
   57330:	xor    %r8d,%r8d
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
   57380:	cmp    %rcx,%rax
   57383:	je     573ba <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa4a>
   57385:	neg    %rax
   57388:	lea    (%r14,%rcx,8),%rdx
   5738c:	neg    %rcx
   5738f:	lea    (%r15,%r14,1),%rsi
   57393:	add    $0xfffffffffffffff8,%rsi
   57397:	nopw   0x0(%rax,%rax,1)
   573a0:	mov    (%rdx),%rdi
   573a3:	mov    (%rsi,%rcx,8),%r8
   573a7:	mov    %r8,(%rdx)
   573aa:	mov    %rdi,(%rsi,%rcx,8)
   573ae:	dec    %rcx
   573b1:	add    $0x8,%rdx
   573b5:	cmp    %rcx,%rax
   573b8:	jne    573a0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa30>
   573ba:	movabs $0x7ffffffffffffffc,%rax
   573c4:	lea    0x10(%r14),%rcx
   573c8:	lea    -0x10(%r14),%rdx
   573cc:	lea    -0x8(%r14),%r8
   573d0:	xor    %esi,%esi
   573d2:	jmp    573e9 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa79>
   573d4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   573e0:	cmp    %rbx,%rsi
   573e3:	jae    56984 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0x14>
   573e9:	mov    %rsi,%rdi
   573ec:	lea    0x1(%rsi),%r9
   573f0:	cmp    %r9,%rbx
   573f3:	mov    %r9,%rsi
   573f6:	cmova  %rbx,%rsi
   573fa:	lea    -0x1(%rsi),%r10
   573fe:	cmp    %r10,%rdi
   57401:	je     57423 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab3>
   57403:	mov    (%r14,%rdi,8),%r10
   57407:	nopw   0x0(%rax,%rax,1)
   57410:	cmp    %r10,(%r14,%r9,8)
   57414:	jne    57420 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab0>
   57416:	inc    %r9
   57419:	cmp    %r9,%rsi
   5741c:	jne    57410 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xaa0>
   5741e:	jmp    57423 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xab3>
   57420:	mov    %r9,%rsi
   57423:	mov    %rsi,%r10
   57426:	sub    %rdi,%r10
   57429:	jb     574ef <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb7f>
   5742f:	mov    %r10,%r9
   57432:	shr    $1,%r9
   57435:	je     573e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa70>
   57437:	cmp    $0x8,%r10
   5743b:	jae    57442 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xad2>
   5743d:	xor    %r10d,%r10d
   57440:	jmp    574ba <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb4a>
   57442:	mov    %r9,%r10
   57445:	and    %rax,%r10
   57448:	mov    %r9,%r11
   5744b:	and    $0xfffffffffffffffc,%r11
   5744f:	neg    %r11
   57452:	lea    (%rcx,%rdi,8),%r15
   57456:	lea    (%rdx,%rsi,8),%r12
   5745a:	xor    %r13d,%r13d
   5745d:	nopl   (%rax)
   57460:	movdqu -0x10(%r15),%xmm0
   57466:	movdqu (%r15),%xmm1
   5746b:	movdqu -0x10(%r12,%r13,8),%xmm2
   57472:	movdqu (%r12,%r13,8),%xmm3
   57478:	pshufd $0x4e,%xmm3,%xmm3
   5747d:	pshufd $0x4e,%xmm2,%xmm2
   57482:	movdqu %xmm3,-0x10(%r15)
   57488:	movdqu %xmm2,(%r15)
   5748d:	pshufd $0x4e,%xmm0,%xmm0
   57492:	pshufd $0x4e,%xmm1,%xmm1
   57497:	movdqu %xmm0,(%r12,%r13,8)
   5749d:	movdqu %xmm1,-0x10(%r12,%r13,8)
   574a4:	add    $0xfffffffffffffffc,%r13
   574a8:	add    $0x20,%r15
   574ac:	cmp    %r13,%r11
   574af:	jne    57460 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xaf0>
   574b1:	cmp    %r10,%r9
   574b4:	je     573e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa70>
   574ba:	lea    (%r14,%rdi,8),%rdi
   574be:	lea    0x0(,%r10,8),%r15
   574c6:	lea    (%r8,%rsi,8),%r11
   574ca:	sub    %r15,%r11
   574cd:	nopl   (%rax)
   574d0:	mov    (%rdi,%r10,8),%r15
   574d4:	mov    (%r11),%r12
   574d7:	mov    %r12,(%rdi,%r10,8)
   574db:	mov    %r15,(%r11)
   574de:	inc    %r10
   574e1:	add    $0xfffffffffffffff8,%r11
   574e5:	cmp    %r10,%r9
   574e8:	jne    574d0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xb60>
   574ea:	jmp    573e0 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xa70>
   574ef:	lea    0x80e1a(%rip),%rcx        # d8310 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7e0>
   574f6:	mov    %rbx,%rdx
   574f9:	call   *0x83411(%rip)        # da910 <_DYNAMIC+0x280>
   574ff:	lea    -0x4a965(%rip),%rdi        # cba1 <_RNvNtCs5PduvXNAoUW_12panic_unwind3imp6CANARY.llvm.6315980824996689386+0x1f2>
   57506:	lea    0x80e63(%rip),%rdx        # d8370 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x840>
   5750d:	mov    $0x13,%esi
   57512:	call   *0x83428(%rip)        # da940 <_DYNAMIC+0x2b0>
   57518:	jmp    57548 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbd8>
   5751a:	lea    0x80e67(%rip),%rdi        # d8388 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x858>
   57521:	call   *0x83551(%rip)        # daa78 <_DYNAMIC+0x3e8>
   57527:	jmp    57548 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbd8>
   57529:	mov    $0x8,%edi
   5752e:	mov    %r12,%rsi
   57531:	call   *0x83399(%rip)        # da8d0 <_DYNAMIC+0x240>
   57537:	lea    0x80e62(%rip),%rdx        # d83a0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x870>
   5753e:	xor    %edi,%edi
   57540:	xor    %esi,%esi
   57542:	call   *0x83398(%rip)        # da8e0 <_DYNAMIC+0x250>
   57548:	ud2
   5754a:	mov    %rax,%rbx
   5754d:	cmpq   $0x0,0x8(%rsp)
   57553:	je     57568 <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbf8>
   57555:	mov    0x10(%rsp),%r15
   5755a:	jmp    5755f <_RINvNtNtNtCs7jsxoleoEt5_16sort_research_rs5other18rust_tessera_radix6engine20sort_with_tile_bytesyECs59F9KgNujep_8radix_ab+0xbef>
   5755c:	mov    %rax,%rbx
   5755f:	mov    %r15,%rdi
   57562:	call   *0x83330(%rip)        # da898 <free@GLIBC_2.2.5>
   57568:	mov    %rbx,%rdi
   5756b:	call   d5f40 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
