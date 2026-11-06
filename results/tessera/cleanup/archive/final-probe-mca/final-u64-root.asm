
/root/code/sort-research-rs/logs/tessera-cleanup/final-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056870 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>>:
   56870:	push   %rbp
   56871:	push   %r15
   56873:	push   %r14
   56875:	push   %r13
   56877:	push   %r12
   56879:	push   %rbx
   5687a:	sub    $0x28,%rsp
   5687e:	cmp    $0x2,%rsi
   56882:	jae    56893 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x23>
   56884:	add    $0x28,%rsp
   56888:	pop    %rbx
   56889:	pop    %r12
   5688b:	pop    %r13
   5688d:	pop    %r14
   5688f:	pop    %r15
   56891:	pop    %rbp
   56892:	ret
   56893:	mov    %rdi,%rbx
   56896:	cmp    $0x9,%rdx
   5689a:	mov    $0x8,%eax
   5689f:	cmovae %rdx,%rax
   568a3:	cmp    $0x7fff8,%rax
   568a9:	mov    $0x7fff8,%r15d
   568af:	cmovb  %rax,%r15
   568b3:	mov    %rsi,%rcx
   568b6:	shr    $0xc,%rcx
   568ba:	cmp    $0x1,%rcx
   568be:	adc    $0x0,%rcx
   568c2:	xor    %eax,%eax
   568c4:	mov    %rsi,%rdx
   568c7:	sub    %rcx,%rdx
   568ca:	cmovae %rdx,%rax
   568ce:	mov    %rax,%rdx
   568d1:	or     %rcx,%rdx
   568d4:	shr    $0x20,%rdx
   568d8:	je     568e1 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x71>
   568da:	xor    %edx,%edx
   568dc:	div    %rcx
   568df:	jmp    568e5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x75>
   568e1:	xor    %edx,%edx
   568e3:	div    %ecx
   568e5:	xor    %edi,%edi
   568e7:	test   %rdx,%rdx
   568ea:	setne  %dil
   568ee:	add    %rax,%rdi
   568f1:	je     56a95 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x225>
   568f7:	neg    %rdi
   568fa:	mov    $0x1,%al
   568fc:	mov    $0x1,%r8b
   568ff:	mov    %rcx,%rdx
   56902:	jmp    5691c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xac>
   56904:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   56910:	add    %rcx,%rdx
   56913:	inc    %rdi
   56916:	je     56a8c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x21c>
   5691c:	mov    %r8d,%r9d
   5691f:	mov    %eax,%r10d
   56922:	mov    -0x8(%rbx,%rdx,8),%rax
   56927:	cmp    (%rbx,%rdx,8),%rax
   5692b:	setbe  %al
   5692e:	setae  %r8b
   56932:	and    %r10b,%al
   56935:	and    %r9b,%r8b
   56938:	test   %al,%al
   5693a:	jne    56910 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa0>
   5693c:	test   %r8b,%r8b
   5693f:	jne    56910 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa0>
   56941:	mov    %r15,(%rsp)
   56945:	mov    %r15,%rax
   56948:	shr    $0x3,%rax
   5694c:	cmp    $0x3,%rax
   56950:	mov    $0x2,%r13d
   56956:	cmovae %rax,%r13
   5695a:	cmp    %rsi,%r13
   5695d:	mov    %rsi,%r14
   56960:	mov    %rsi,%rbp
   56963:	cmovb  %r13,%rbp
   56967:	lea    0x0(,%rbp,8),%r12
   5696f:	mov    %r12,%rdi
   56972:	call   *0x83b30(%rip)        # da4a8 <malloc@GLIBC_2.2.5>
   56978:	test   %rax,%rax
   5697b:	je     5747c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc0c>
   56981:	mov    %rax,%r15
   56984:	mov    %rax,%rdi
   56987:	mov    %rbx,%rsi
   5698a:	mov    %r12,%rdx
   5698d:	call   *0x83b35(%rip)        # da4c8 <memcpy@GLIBC_2.14>
   56993:	mov    %rbp,%rax
   56996:	neg    %rax
   56999:	xor    %r12d,%r12d
   5699c:	xor    %ecx,%ecx
   5699e:	lea    0x0(,%rcx,8),%rdx
   569a6:	mov    (%r15,%rcx,8),%rsi
   569aa:	inc    %rcx
   569ad:	neg    %rdx
   569b0:	mov    %rsi,%rdi
   569b3:	mov    (%r15,%rcx,8),%rsi
   569b7:	cmp    %rsi,%rdi
   569ba:	ja     569cd <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x15d>
   569bc:	inc    %rcx
   569bf:	add    $0xfffffffffffffff8,%rdx
   569c3:	cmp    %rcx,%rbp
   569c6:	jne    569b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x140>
   569c8:	jmp    56a5c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1ec>
   569cd:	cmp    %rcx,%rbp
   569d0:	jb     57452 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbe2>
   569d6:	test   %rcx,%rcx
   569d9:	je     5746d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbfd>
   569df:	mov    %rbp,%rsi
   569e2:	sub    %rcx,%rsi
   569e5:	je     5748a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc1a>
   569eb:	mov    -0x8(%r15,%rcx,8),%r8
   569f0:	mov    (%r15,%rcx,8),%rdi
   569f4:	mov    %rdi,-0x8(%r15,%rcx,8)
   569f9:	mov    %r8,(%r15,%rcx,8)
   569fd:	inc    %r12
   56a00:	cmp    %rbp,%r12
   56a03:	ja     56c0a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x39a>
   56a09:	lea    (%r15,%rcx,8),%r8
   56a0d:	add    $0xfffffffffffffff8,%r8
   56a11:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56a20:	test   %rdx,%rdx
   56a23:	je     56a52 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1e2>
   56a25:	mov    -0x8(%r8),%r9
   56a29:	cmp    %rdi,%r9
   56a2c:	jbe    56a52 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1e2>
   56a2e:	mov    %rdi,-0x8(%r8)
   56a32:	mov    %r9,(%r8)
   56a35:	add    $0xfffffffffffffff8,%r8
   56a39:	add    $0x8,%rdx
   56a3d:	lea    (%rax,%r12,1),%r9
   56a41:	inc    %r9
   56a44:	inc    %r12
   56a47:	cmp    $0x1,%r9
   56a4b:	jne    56a20 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1b0>
   56a4d:	jmp    56c0a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x39a>
   56a52:	cmp    $0x2,%rsi
   56a56:	jae    5699e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x12e>
   56a5c:	mov    %r15,%rdi
   56a5f:	call   *0x83a53(%rip)        # da4b8 <free@GLIBC_2.2.5>
   56a65:	test   %r12,%r12
   56a68:	je     56bf5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x385>
   56a6e:	imul   %r14,%r12
   56a72:	mov    %r12,%rax
   56a75:	shr    $0x20,%rax
   56a79:	je     56dd6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x566>
   56a7f:	mov    %r12,%rax
   56a82:	xor    %edx,%edx
   56a84:	div    %rbp
   56a87:	jmp    56ddd <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x56d>
   56a8c:	test   %r8b,%al
   56a8f:	je     56bb0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x340>
   56a95:	cmp    $0x401,%rsi
   56a9c:	mov    $0x401,%r8d
   56aa2:	cmovb  %rsi,%r8
   56aa6:	mov    (%rbx),%rax
   56aa9:	lea    0x0(,%r8,8),%edi
   56ab1:	lea    (%rbx,%rdi,1),%rdx
   56ab5:	lea    0x8(%rbx),%rcx
   56ab9:	lea    -0x10(%rdi),%r11
   56abd:	cmp    $0x18,%r11
   56ac1:	jae    56ace <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x25e>
   56ac3:	mov    %rax,%r9
   56ac6:	mov    %rax,%r10
   56ac9:	jmp    56b50 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x2e0>
   56ace:	shr    $0x3,%r11
   56ad2:	inc    %r11
   56ad5:	mov    %r11,%r14
   56ad8:	and    $0xfffffffffffffffc,%r14
   56adc:	lea    (%rcx,%r14,8),%rcx
   56ae0:	movq   %rax,%xmm2
   56ae5:	shufpd $0x2,-0x4cd2e(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56aee:	movq   %rax,%xmm1
   56af3:	pxor   %xmm0,%xmm0
   56af7:	pcmpeqd %xmm3,%xmm3
   56afb:	xor    %r9d,%r9d
   56afe:	xchg   %ax,%ax
   56b00:	movdqu 0x8(%rbx,%r9,8),%xmm4
   56b07:	movdqu 0x18(%rbx,%r9,8),%xmm5
   56b0e:	por    %xmm4,%xmm1
   56b12:	por    %xmm5,%xmm0
   56b16:	andpd  %xmm4,%xmm2
   56b1a:	pand   %xmm5,%xmm3
   56b1e:	add    $0x4,%r9
   56b22:	cmp    %r9,%r14
   56b25:	jne    56b00 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x290>
   56b27:	pand   %xmm2,%xmm3
   56b2b:	pshufd $0xee,%xmm3,%xmm2
   56b30:	pand   %xmm3,%xmm2
   56b34:	movq   %xmm2,%r9
   56b39:	por    %xmm1,%xmm0
   56b3d:	pshufd $0xee,%xmm0,%xmm1
   56b42:	por    %xmm0,%xmm1
   56b46:	movq   %xmm1,%r10
   56b4b:	cmp    %r14,%r11
   56b4e:	je     56b62 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x2f2>
   56b50:	mov    (%rcx),%r11
   56b53:	or     %r11,%r10
   56b56:	and    %r11,%r9
   56b59:	add    $0x8,%rcx
   56b5d:	cmp    %rdx,%rcx
   56b60:	jne    56b50 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x2e0>
   56b62:	mov    %r9,%rcx
   56b65:	xor    %r10,%rcx
   56b68:	cmp    $0xffffffffffffffff,%rcx
   56b6c:	je     56ba4 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x334>
   56b6e:	lea    0x0(,%rsi,8),%r11
   56b76:	cmp    %r11,%rdi
   56b79:	je     56cd5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x465>
   56b7f:	not    %r8
   56b82:	add    %rsi,%r8
   56b85:	movabs $0x1fffffffffffffff,%r11
   56b8f:	and    %r8,%r11
   56b92:	cmp    $0x3,%r11
   56b96:	jae    56c1f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3af>
   56b9c:	mov    %r10,%rcx
   56b9f:	jmp    56cad <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x43d>
   56ba4:	mov    $0xffffffffffffffff,%rcx
   56bab:	jmp    56cde <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x46e>
   56bb0:	mov    (%rbx),%rdi
   56bb3:	lea    0x0(,%rsi,8),%rax
   56bbb:	mov    %rax,%r8
   56bbe:	neg    %r8
   56bc1:	mov    $0x10,%ecx
   56bc6:	cs nopw 0x0(%rax,%rax,1)
   56bd0:	mov    -0x8(%rbx,%rcx,1),%rdx
   56bd5:	cmp    %rdi,%rdx
   56bd8:	jne    56da0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x530>
   56bde:	lea    (%r8,%rcx,1),%rdx
   56be2:	add    $0x8,%rdx
   56be6:	add    $0x8,%rcx
   56bea:	cmp    $0x8,%rdx
   56bee:	jne    56bd0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x360>
   56bf0:	jmp    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56bf5:	mov    %r14,%rsi
   56bf8:	cmp    %r13,%r14
   56bfb:	mov    (%rsp),%r15
   56bff:	jbe    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56c05:	jmp    56dfb <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x58b>
   56c0a:	mov    %r15,%rdi
   56c0d:	call   *0x838a5(%rip)        # da4b8 <free@GLIBC_2.2.5>
   56c13:	mov    %r14,%rsi
   56c16:	mov    (%rsp),%r15
   56c1a:	jmp    56dfb <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x58b>
   56c1f:	inc    %r11
   56c22:	mov    %r11,%r8
   56c25:	and    $0xfffffffffffffffc,%r8
   56c29:	lea    (%rdx,%r8,8),%rdx
   56c2d:	movq   %r9,%xmm2
   56c32:	shufpd $0x2,-0x4ce7b(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56c3b:	movq   %r10,%xmm1
   56c40:	lea    (%rdi,%rbx,1),%rcx
   56c44:	add    $0x10,%rcx
   56c48:	pxor   %xmm0,%xmm0
   56c4c:	pcmpeqd %xmm3,%xmm3
   56c50:	xor    %edi,%edi
   56c52:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56c60:	movdqu -0x10(%rcx,%rdi,8),%xmm4
   56c66:	movdqu (%rcx,%rdi,8),%xmm5
   56c6b:	por    %xmm4,%xmm1
   56c6f:	por    %xmm5,%xmm0
   56c73:	andpd  %xmm4,%xmm2
   56c77:	pand   %xmm5,%xmm3
   56c7b:	add    $0x4,%rdi
   56c7f:	cmp    %rdi,%r8
   56c82:	jne    56c60 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3f0>
   56c84:	pand   %xmm2,%xmm3
   56c88:	pshufd $0xee,%xmm3,%xmm2
   56c8d:	pand   %xmm3,%xmm2
   56c91:	movq   %xmm2,%r9
   56c96:	por    %xmm1,%xmm0
   56c9a:	pshufd $0xee,%xmm0,%xmm1
   56c9f:	por    %xmm0,%xmm1
   56ca3:	movq   %xmm1,%rcx
   56ca8:	cmp    %r8,%r11
   56cab:	je     56cd2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x462>
   56cad:	lea    (%rbx,%rsi,8),%rdi
   56cb1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56cc0:	mov    (%rdx),%r8
   56cc3:	add    $0x8,%rdx
   56cc7:	or     %r8,%rcx
   56cca:	and    %r8,%r9
   56ccd:	cmp    %rdi,%rdx
   56cd0:	jne    56cc0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x450>
   56cd2:	xor    %r9,%rcx
   56cd5:	test   %rcx,%rcx
   56cd8:	je     56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56cde:	lea    0x0(,%rsi,8),%rdx
   56ce6:	mov    %rdx,%r9
   56ce9:	neg    %r9
   56cec:	mov    $0x10,%edi
   56cf1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56d00:	mov    -0x8(%rbx,%rdi,1),%r8
   56d05:	cmp    %rax,%r8
   56d08:	jne    56d21 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4b1>
   56d0a:	lea    (%r9,%rdi,1),%r8
   56d0e:	add    $0x8,%r8
   56d12:	add    $0x8,%rdi
   56d16:	cmp    $0x8,%r8
   56d1a:	jne    56d00 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x490>
   56d1c:	jmp    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56d21:	mov    %rdi,%r9
   56d24:	sub    %rdx,%r9
   56d27:	cmp    %r8,%rax
   56d2a:	jae    56d5e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4ee>
   56d2c:	test   %r9,%r9
   56d2f:	je     56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56d35:	data16 cs nopw 0x0(%rax,%rax,1)
   56d40:	mov    %r8,%rax
   56d43:	mov    (%rbx,%rdi,1),%r8
   56d47:	cmp    %r8,%rax
   56d4a:	ja     571fe <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x98e>
   56d50:	add    $0x8,%rdi
   56d54:	cmp    %rdi,%rdx
   56d57:	jne    56d40 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4d0>
   56d59:	jmp    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56d5e:	test   %r9,%r9
   56d61:	je     56d89 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x519>
   56d63:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56d70:	mov    %r8,%rax
   56d73:	mov    (%rbx,%rdi,1),%r8
   56d77:	cmp    %r8,%rax
   56d7a:	jb     571fe <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x98e>
   56d80:	add    $0x8,%rdi
   56d84:	cmp    %rdi,%rdx
   56d87:	jne    56d70 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x500>
   56d89:	mov    %rsi,%rax
   56d8c:	shr    $1,%rax
   56d8f:	cmp    $0x8,%rsi
   56d93:	jae    56f45 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6d5>
   56d99:	xor    %ecx,%ecx
   56d9b:	jmp    56fc6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x756>
   56da0:	mov    %rcx,%r8
   56da3:	sub    %rax,%r8
   56da6:	cmp    %rdx,%rdi
   56da9:	jae    56f10 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6a0>
   56daf:	test   %r8,%r8
   56db2:	je     56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56db8:	mov    %rdx,%rdi
   56dbb:	mov    (%rbx,%rcx,1),%rdx
   56dbf:	cmp    %rdx,%rdi
   56dc2:	ja     56941 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xd1>
   56dc8:	add    $0x8,%rcx
   56dcc:	cmp    %rcx,%rax
   56dcf:	jne    56db8 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x548>
   56dd1:	jmp    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56dd6:	mov    %r12d,%eax
   56dd9:	xor    %edx,%edx
   56ddb:	div    %ebp
   56ddd:	mov    (%rsp),%r15
   56de1:	mov    %rbx,%rdi
   56de4:	mov    %r14,%rsi
   56de7:	mov    %rax,%rdx
   56dea:	call   58020 <sort_research_rs::other::rust_tessera_radix::engine::budgeted_insertion_sort::<u64, u64, <u64 as sort_research_rs::other::radix_support::Key>::key>>
   56def:	mov    %r14,%rsi
   56df2:	test   %rax,%rax
   56df5:	jne    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   56dfb:	cmp    $0x401,%rsi
   56e02:	mov    $0x401,%r8d
   56e08:	cmovb  %rsi,%r8
   56e0c:	lea    0x0(,%r8,8),%edi
   56e14:	lea    (%rbx,%rdi,1),%rax
   56e18:	mov    (%rbx),%rdx
   56e1b:	lea    0x8(%rbx),%rcx
   56e1f:	lea    -0x10(%rdi),%r10
   56e23:	cmp    $0x18,%r10
   56e27:	jae    56e31 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x5c1>
   56e29:	mov    %rdx,%r9
   56e2c:	jmp    56eb0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x640>
   56e31:	shr    $0x3,%r10
   56e35:	inc    %r10
   56e38:	mov    %r10,%r11
   56e3b:	and    $0xfffffffffffffffc,%r11
   56e3f:	lea    (%rcx,%r11,8),%rcx
   56e43:	movq   %rdx,%xmm2
   56e48:	shufpd $0x2,-0x4d091(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56e51:	movq   %rdx,%xmm1
   56e56:	pxor   %xmm0,%xmm0
   56e5a:	pcmpeqd %xmm3,%xmm3
   56e5e:	xor    %edx,%edx
   56e60:	movdqu 0x8(%rbx,%rdx,8),%xmm4
   56e66:	movdqu 0x18(%rbx,%rdx,8),%xmm5
   56e6c:	por    %xmm4,%xmm1
   56e70:	por    %xmm5,%xmm0
   56e74:	andpd  %xmm4,%xmm2
   56e78:	pand   %xmm5,%xmm3
   56e7c:	add    $0x4,%rdx
   56e80:	cmp    %rdx,%r11
   56e83:	jne    56e60 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x5f0>
   56e85:	pand   %xmm2,%xmm3
   56e89:	pshufd $0xee,%xmm3,%xmm2
   56e8e:	pand   %xmm3,%xmm2
   56e92:	movq   %xmm2,%rdx
   56e97:	por    %xmm1,%xmm0
   56e9b:	pshufd $0xee,%xmm0,%xmm1
   56ea0:	por    %xmm0,%xmm1
   56ea4:	movq   %xmm1,%r9
   56ea9:	cmp    %r11,%r10
   56eac:	je     56ec2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x652>
   56eae:	xchg   %ax,%ax
   56eb0:	mov    (%rcx),%r10
   56eb3:	or     %r10,%r9
   56eb6:	and    %r10,%rdx
   56eb9:	add    $0x8,%rcx
   56ebd:	cmp    %rax,%rcx
   56ec0:	jne    56eb0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x640>
   56ec2:	mov    %rdx,%rcx
   56ec5:	xor    %r9,%rcx
   56ec8:	cmp    $0xffffffffffffffff,%rcx
   56ecc:	je     56f04 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x694>
   56ece:	lea    0x0(,%rsi,8),%r10
   56ed6:	cmp    %r10,%rdi
   56ed9:	je     571f5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x985>
   56edf:	not    %r8
   56ee2:	add    %rsi,%r8
   56ee5:	movabs $0x1fffffffffffffff,%r10
   56eef:	and    %r8,%r10
   56ef2:	cmp    $0x3,%r10
   56ef6:	jae    5713f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8cf>
   56efc:	mov    %r9,%rcx
   56eff:	jmp    571cd <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x95d>
   56f04:	mov    $0xffffffffffffffff,%rcx
   56f0b:	jmp    571fe <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x98e>
   56f10:	test   %r8,%r8
   56f13:	je     56f2e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6be>
   56f15:	mov    %rdx,%rdi
   56f18:	mov    (%rbx,%rcx,1),%rdx
   56f1c:	cmp    %rdx,%rdi
   56f1f:	jb     56941 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xd1>
   56f25:	add    $0x8,%rcx
   56f29:	cmp    %rcx,%rax
   56f2c:	jne    56f15 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6a5>
   56f2e:	mov    %rsi,%rcx
   56f31:	shr    $1,%rcx
   56f34:	cmp    $0x8,%rsi
   56f38:	jae    5724d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9dd>
   56f3e:	xor    %edx,%edx
   56f40:	jmp    572d6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa66>
   56f45:	movabs $0x7fffffffffffffc,%rcx
   56f4f:	and    %rax,%rcx
   56f52:	mov    %rax,%rdi
   56f55:	and    $0xfffffffffffffffc,%rdi
   56f59:	neg    %rdi
   56f5c:	lea    0x10(%rbx),%r8
   56f60:	lea    (%rdx,%rbx,1),%r9
   56f64:	add    $0xfffffffffffffff0,%r9
   56f68:	xor    %r10d,%r10d
   56f6b:	nopl   0x0(%rax,%rax,1)
   56f70:	movdqu -0x10(%r8),%xmm0
   56f76:	movdqu (%r8),%xmm1
   56f7b:	movdqu -0x10(%r9,%r10,8),%xmm2
   56f82:	movdqu (%r9,%r10,8),%xmm3
   56f88:	pshufd $0x4e,%xmm3,%xmm3
   56f8d:	pshufd $0x4e,%xmm2,%xmm2
   56f92:	movdqu %xmm3,-0x10(%r8)
   56f98:	movdqu %xmm2,(%r8)
   56f9d:	pshufd $0x4e,%xmm0,%xmm0
   56fa2:	pshufd $0x4e,%xmm1,%xmm1
   56fa7:	movdqu %xmm0,(%r9,%r10,8)
   56fad:	movdqu %xmm1,-0x10(%r9,%r10,8)
   56fb4:	add    $0xfffffffffffffffc,%r10
   56fb8:	add    $0x20,%r8
   56fbc:	cmp    %r10,%rdi
   56fbf:	jne    56f70 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x700>
   56fc1:	cmp    %rcx,%rax
   56fc4:	je     56ffa <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x78a>
   56fc6:	neg    %rax
   56fc9:	lea    (%rbx,%rcx,8),%rdi
   56fcd:	neg    %rcx
   56fd0:	add    %rbx,%rdx
   56fd3:	add    $0xfffffffffffffff8,%rdx
   56fd7:	nopw   0x0(%rax,%rax,1)
   56fe0:	mov    (%rdi),%r8
   56fe3:	mov    (%rdx,%rcx,8),%r9
   56fe7:	mov    %r9,(%rdi)
   56fea:	mov    %r8,(%rdx,%rcx,8)
   56fee:	dec    %rcx
   56ff1:	add    $0x8,%rdi
   56ff5:	cmp    %rcx,%rax
   56ff8:	jne    56fe0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x770>
   56ffa:	movabs $0x7ffffffffffffffc,%rcx
   57004:	lea    0x10(%rbx),%rdx
   57008:	lea    -0x10(%rbx),%r8
   5700c:	lea    -0x8(%rbx),%r9
   57010:	xor    %eax,%eax
   57012:	jmp    57029 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b9>
   57014:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57020:	cmp    %rsi,%rax
   57023:	jae    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   57029:	mov    %rax,%rdi
   5702c:	lea    0x1(%rax),%r10
   57030:	cmp    %r10,%rsi
   57033:	mov    %r10,%rax
   57036:	cmova  %rsi,%rax
   5703a:	lea    -0x1(%rax),%r11
   5703e:	cmp    %r11,%rdi
   57041:	je     57063 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7f3>
   57043:	mov    (%rbx,%rdi,8),%r11
   57047:	nopw   0x0(%rax,%rax,1)
   57050:	cmp    %r11,(%rbx,%r10,8)
   57054:	jne    57060 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7f0>
   57056:	inc    %r10
   57059:	cmp    %r10,%rax
   5705c:	jne    57050 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7e0>
   5705e:	jmp    57063 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7f3>
   57060:	mov    %r10,%rax
   57063:	mov    %rax,%r11
   57066:	sub    %rdi,%r11
   57069:	jb     5743f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbcf>
   5706f:	mov    %r11,%r10
   57072:	shr    $1,%r10
   57075:	je     57020 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b0>
   57077:	cmp    $0x8,%r11
   5707b:	jae    57090 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x820>
   5707d:	xor    %r11d,%r11d
   57080:	jmp    5710a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x89a>
   57085:	data16 cs nopw 0x0(%rax,%rax,1)
   57090:	mov    %r10,%r11
   57093:	and    %rcx,%r11
   57096:	mov    %r10,%r14
   57099:	and    $0xfffffffffffffffc,%r14
   5709d:	neg    %r14
   570a0:	lea    (%rdx,%rdi,8),%r15
   570a4:	lea    (%r8,%rax,8),%r12
   570a8:	xor    %r13d,%r13d
   570ab:	nopl   0x0(%rax,%rax,1)
   570b0:	movdqu -0x10(%r15),%xmm0
   570b6:	movdqu (%r15),%xmm1
   570bb:	movdqu -0x10(%r12,%r13,8),%xmm2
   570c2:	movdqu (%r12,%r13,8),%xmm3
   570c8:	pshufd $0x4e,%xmm3,%xmm3
   570cd:	pshufd $0x4e,%xmm2,%xmm2
   570d2:	movdqu %xmm3,-0x10(%r15)
   570d8:	movdqu %xmm2,(%r15)
   570dd:	pshufd $0x4e,%xmm0,%xmm0
   570e2:	pshufd $0x4e,%xmm1,%xmm1
   570e7:	movdqu %xmm0,(%r12,%r13,8)
   570ed:	movdqu %xmm1,-0x10(%r12,%r13,8)
   570f4:	add    $0xfffffffffffffffc,%r13
   570f8:	add    $0x20,%r15
   570fc:	cmp    %r13,%r14
   570ff:	jne    570b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x840>
   57101:	cmp    %r11,%r10
   57104:	je     57020 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b0>
   5710a:	lea    (%rbx,%rdi,8),%rdi
   5710e:	lea    0x0(,%r11,8),%r15
   57116:	lea    (%r9,%rax,8),%r14
   5711a:	sub    %r15,%r14
   5711d:	nopl   (%rax)
   57120:	mov    (%rdi,%r11,8),%r15
   57124:	mov    (%r14),%r12
   57127:	mov    %r12,(%rdi,%r11,8)
   5712b:	mov    %r15,(%r14)
   5712e:	inc    %r11
   57131:	add    $0xfffffffffffffff8,%r14
   57135:	cmp    %r11,%r10
   57138:	jne    57120 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8b0>
   5713a:	jmp    57020 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b0>
   5713f:	inc    %r10
   57142:	mov    %r10,%r8
   57145:	and    $0xfffffffffffffffc,%r8
   57149:	lea    (%rax,%r8,8),%rax
   5714d:	movq   %rdx,%xmm2
   57152:	shufpd $0x2,-0x4d39b(%rip),%xmm2        # 9dc0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   5715b:	movq   %r9,%xmm1
   57160:	lea    (%rdi,%rbx,1),%rcx
   57164:	add    $0x10,%rcx
   57168:	pxor   %xmm0,%xmm0
   5716c:	pcmpeqd %xmm3,%xmm3
   57170:	xor    %edx,%edx
   57172:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57180:	movdqu -0x10(%rcx,%rdx,8),%xmm4
   57186:	movdqu (%rcx,%rdx,8),%xmm5
   5718b:	por    %xmm4,%xmm1
   5718f:	por    %xmm5,%xmm0
   57193:	andpd  %xmm4,%xmm2
   57197:	pand   %xmm5,%xmm3
   5719b:	add    $0x4,%rdx
   5719f:	cmp    %rdx,%r8
   571a2:	jne    57180 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x910>
   571a4:	pand   %xmm2,%xmm3
   571a8:	pshufd $0xee,%xmm3,%xmm2
   571ad:	pand   %xmm3,%xmm2
   571b1:	movq   %xmm2,%rdx
   571b6:	por    %xmm1,%xmm0
   571ba:	pshufd $0xee,%xmm0,%xmm1
   571bf:	por    %xmm0,%xmm1
   571c3:	movq   %xmm1,%rcx
   571c8:	cmp    %r8,%r10
   571cb:	je     571f2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x982>
   571cd:	lea    (%rbx,%rsi,8),%rdi
   571d1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   571e0:	mov    (%rax),%r8
   571e3:	add    $0x8,%rax
   571e7:	or     %r8,%rcx
   571ea:	and    %r8,%rdx
   571ed:	cmp    %rdi,%rax
   571f0:	jne    571e0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x970>
   571f2:	xor    %rdx,%rcx
   571f5:	test   %rcx,%rcx
   571f8:	je     56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   571fe:	movq   $0x0,0x8(%rsp)
   57207:	movq   $0x8,0x10(%rsp)
   57210:	movq   $0x0,0x18(%rsp)
   57219:	mov    %r15,0x20(%rsp)
   5721e:	lea    0x8(%rsp),%rdi
   57223:	mov    %rsi,%rdx
   57226:	mov    %rbx,%rsi
   57229:	xor    %r8d,%r8d
   5722c:	call   65fb0 <<sort_research_rs::other::rust_tessera_radix::engine::Sorter<u64>>::sort_node>
   57231:	cmpq   $0x0,0x8(%rsp)
   57237:	je     56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   5723d:	mov    0x10(%rsp),%rdi
   57242:	call   *0x83270(%rip)        # da4b8 <free@GLIBC_2.2.5>
   57248:	jmp    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   5724d:	movabs $0x7fffffffffffffc,%rdx
   57257:	and    %rcx,%rdx
   5725a:	mov    %rcx,%rdi
   5725d:	and    $0xfffffffffffffffc,%rdi
   57261:	neg    %rdi
   57264:	lea    0x10(%rbx),%r8
   57268:	lea    (%rax,%rbx,1),%r9
   5726c:	add    $0xfffffffffffffff0,%r9
   57270:	xor    %r10d,%r10d
   57273:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57280:	movdqu -0x10(%r8),%xmm0
   57286:	movdqu (%r8),%xmm1
   5728b:	movdqu -0x10(%r9,%r10,8),%xmm2
   57292:	movdqu (%r9,%r10,8),%xmm3
   57298:	pshufd $0x4e,%xmm3,%xmm3
   5729d:	pshufd $0x4e,%xmm2,%xmm2
   572a2:	movdqu %xmm3,-0x10(%r8)
   572a8:	movdqu %xmm2,(%r8)
   572ad:	pshufd $0x4e,%xmm0,%xmm0
   572b2:	pshufd $0x4e,%xmm1,%xmm1
   572b7:	movdqu %xmm0,(%r9,%r10,8)
   572bd:	movdqu %xmm1,-0x10(%r9,%r10,8)
   572c4:	add    $0xfffffffffffffffc,%r10
   572c8:	add    $0x20,%r8
   572cc:	cmp    %r10,%rdi
   572cf:	jne    57280 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa10>
   572d1:	cmp    %rdx,%rcx
   572d4:	je     5730a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa9a>
   572d6:	neg    %rcx
   572d9:	lea    (%rbx,%rdx,8),%rdi
   572dd:	neg    %rdx
   572e0:	add    %rbx,%rax
   572e3:	add    $0xfffffffffffffff8,%rax
   572e7:	nopw   0x0(%rax,%rax,1)
   572f0:	mov    (%rdi),%r8
   572f3:	mov    (%rax,%rdx,8),%r9
   572f7:	mov    %r9,(%rdi)
   572fa:	mov    %r8,(%rax,%rdx,8)
   572fe:	dec    %rdx
   57301:	add    $0x8,%rdi
   57305:	cmp    %rdx,%rcx
   57308:	jne    572f0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa80>
   5730a:	movabs $0x7ffffffffffffffc,%rcx
   57314:	lea    0x10(%rbx),%rdx
   57318:	lea    -0x10(%rbx),%r8
   5731c:	lea    -0x8(%rbx),%r9
   57320:	xor    %eax,%eax
   57322:	jmp    57339 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xac9>
   57324:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57330:	cmp    %rsi,%rax
   57333:	jae    56884 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14>
   57339:	mov    %rax,%rdi
   5733c:	lea    0x1(%rax),%r10
   57340:	cmp    %r10,%rsi
   57343:	mov    %r10,%rax
   57346:	cmova  %rsi,%rax
   5734a:	lea    -0x1(%rax),%r11
   5734e:	cmp    %r11,%rdi
   57351:	je     57373 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb03>
   57353:	mov    (%rbx,%rdi,8),%r11
   57357:	nopw   0x0(%rax,%rax,1)
   57360:	cmp    %r11,(%rbx,%r10,8)
   57364:	jne    57370 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb00>
   57366:	inc    %r10
   57369:	cmp    %r10,%rax
   5736c:	jne    57360 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaf0>
   5736e:	jmp    57373 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb03>
   57370:	mov    %r10,%rax
   57373:	mov    %rax,%r11
   57376:	sub    %rdi,%r11
   57379:	jb     5743f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbcf>
   5737f:	mov    %r11,%r10
   57382:	shr    $1,%r10
   57385:	je     57330 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xac0>
   57387:	cmp    $0x8,%r11
   5738b:	jae    57392 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb22>
   5738d:	xor    %r11d,%r11d
   57390:	jmp    5740a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb9a>
   57392:	mov    %r10,%r11
   57395:	and    %rcx,%r11
   57398:	mov    %r10,%r14
   5739b:	and    $0xfffffffffffffffc,%r14
   5739f:	neg    %r14
   573a2:	lea    (%rdx,%rdi,8),%r15
   573a6:	lea    (%r8,%rax,8),%r12
   573aa:	xor    %r13d,%r13d
   573ad:	nopl   (%rax)
   573b0:	movdqu -0x10(%r15),%xmm0
   573b6:	movdqu (%r15),%xmm1
   573bb:	movdqu -0x10(%r12,%r13,8),%xmm2
   573c2:	movdqu (%r12,%r13,8),%xmm3
   573c8:	pshufd $0x4e,%xmm3,%xmm3
   573cd:	pshufd $0x4e,%xmm2,%xmm2
   573d2:	movdqu %xmm3,-0x10(%r15)
   573d8:	movdqu %xmm2,(%r15)
   573dd:	pshufd $0x4e,%xmm0,%xmm0
   573e2:	pshufd $0x4e,%xmm1,%xmm1
   573e7:	movdqu %xmm0,(%r12,%r13,8)
   573ed:	movdqu %xmm1,-0x10(%r12,%r13,8)
   573f4:	add    $0xfffffffffffffffc,%r13
   573f8:	add    $0x20,%r15
   573fc:	cmp    %r13,%r14
   573ff:	jne    573b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb40>
   57401:	cmp    %r11,%r10
   57404:	je     57330 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xac0>
   5740a:	lea    (%rbx,%rdi,8),%rdi
   5740e:	lea    0x0(,%r11,8),%r15
   57416:	lea    (%r9,%rax,8),%r14
   5741a:	sub    %r15,%r14
   5741d:	nopl   (%rax)
   57420:	mov    (%rdi,%r11,8),%r15
   57424:	mov    (%r14),%r12
   57427:	mov    %r12,(%rdi,%r11,8)
   5742b:	mov    %r15,(%r14)
   5742e:	inc    %r11
   57431:	add    $0xfffffffffffffff8,%r14
   57435:	cmp    %r11,%r10
   57438:	jne    57420 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbb0>
   5743a:	jmp    57330 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xac0>
   5743f:	lea    0x80aea(%rip),%rcx        # d7f30 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7e0>
   57446:	mov    %rsi,%rdx
   57449:	mov    %rax,%rsi
   5744c:	call   *0x830de(%rip)        # da530 <_DYNAMIC+0x280>
   57452:	lea    -0x4a8b8(%rip),%rdi        # cba1 <panic_unwind::imp::CANARY+0x1f2>
   57459:	lea    0x80b30(%rip),%rdx        # d7f90 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x840>
   57460:	mov    $0x13,%esi
   57465:	call   *0x830f5(%rip)        # da560 <_DYNAMIC+0x2b0>
   5746b:	jmp    5749b <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc2b>
   5746d:	lea    0x80b34(%rip),%rdi        # d7fa8 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x858>
   57474:	call   *0x8321e(%rip)        # da698 <_DYNAMIC+0x3e8>
   5747a:	jmp    5749b <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc2b>
   5747c:	mov    $0x8,%edi
   57481:	mov    %r12,%rsi
   57484:	call   *0x83066(%rip)        # da4f0 <_DYNAMIC+0x240>
   5748a:	lea    0x80b2f(%rip),%rdx        # d7fc0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x870>
   57491:	xor    %edi,%edi
   57493:	xor    %esi,%esi
   57495:	call   *0x83065(%rip)        # da500 <_DYNAMIC+0x250>
   5749b:	ud2
   5749d:	mov    %rax,%rbx
   574a0:	cmpq   $0x0,0x8(%rsp)
   574a6:	je     574bb <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc4b>
   574a8:	mov    0x10(%rsp),%rdi
   574ad:	jmp    574b5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc45>
   574af:	mov    %rax,%rbx
   574b2:	mov    %r15,%rdi
   574b5:	call   *0x82ffd(%rip)        # da4b8 <free@GLIBC_2.2.5>
   574bb:	mov    %rbx,%rdi
   574be:	call   d5b60 <_Unwind_Resume@plt>
