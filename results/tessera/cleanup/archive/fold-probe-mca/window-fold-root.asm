
/root/code/sort-research-rs/logs/tessera-cleanup/fold-window-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056af0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>>:
   56af0:	push   %rbp
   56af1:	push   %r15
   56af3:	push   %r14
   56af5:	push   %r13
   56af7:	push   %r12
   56af9:	push   %rbx
   56afa:	sub    $0x28,%rsp
   56afe:	cmp    $0x2,%rsi
   56b02:	jb     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56b08:	cmp    $0x9,%rdx
   56b0c:	mov    $0x8,%eax
   56b11:	cmovae %rdx,%rax
   56b15:	cmp    $0x7fff8,%rax
   56b1b:	mov    $0x7fff8,%r13d
   56b21:	cmovb  %rax,%r13
   56b25:	mov    %rsi,%rcx
   56b28:	shr    $0xc,%rcx
   56b2c:	xor    %eax,%eax
   56b2e:	mov    %rcx,%rdx
   56b31:	sub    $0x1,%rdx
   56b35:	cmovae %rdx,%rax
   56b39:	mov    %rsi,%rdx
   56b3c:	sub    %rax,%rdx
   56b3f:	jb     57705 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc15>
   56b45:	cmp    $0x2,%rdx
   56b49:	jb     56d3a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x24a>
   56b4f:	mov    (%rdi,%rax,8),%r8
   56b53:	cmp    0x8(%rdi,%rax,8),%r8
   56b58:	setae  %r8b
   56b5c:	setbe  %r9b
   56b60:	mov    %rcx,%r10
   56b63:	shl    $0x4,%r10
   56b67:	cmp    $0x1,%rcx
   56b6b:	mov    $0x1,%r11d
   56b71:	cmovb  %rcx,%r11
   56b75:	mov    %r11d,%ebx
   56b78:	shl    $0x4,%ebx
   56b7b:	sub    %rbx,%r10
   56b7e:	add    %rdi,%r10
   56b81:	add    $0x10,%r10
   56b85:	shl    $0x3,%rcx
   56b89:	shl    $0x3,%r11d
   56b8d:	sub    %r11,%rcx
   56b90:	add    $0x8,%rcx
   56b94:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   56ba0:	dec    %rdx
   56ba3:	sub    %rax,%rdx
   56ba6:	jb     56d2d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x23d>
   56bac:	cmp    $0x1,%rdx
   56bb0:	jbe    56d2d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x23d>
   56bb6:	mov    -0x8(%r10),%r11
   56bba:	cmp    (%r10),%r11
   56bbd:	lea    (%r10,%rcx,1),%r11
   56bc1:	setbe  %r10b
   56bc5:	setae  %bl
   56bc8:	and    %r10b,%r9b
   56bcb:	and    %bl,%r8b
   56bce:	mov    %r11,%r10
   56bd1:	test   %r9b,%r9b
   56bd4:	jne    56ba0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb0>
   56bd6:	mov    %r11,%r10
   56bd9:	test   %r8b,%r8b
   56bdc:	jne    56ba0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb0>
   56bde:	mov    %rdi,%rbx
   56be1:	mov    %r13,%rax
   56be4:	shr    $0x3,%rax
   56be8:	cmp    $0x3,%rax
   56bec:	mov    $0x2,%r14d
   56bf2:	cmovae %rax,%r14
   56bf6:	cmp    %rsi,%r14
   56bf9:	mov    %rsi,(%rsp)
   56bfd:	mov    %rsi,%rbp
   56c00:	cmovb  %r14,%rbp
   56c04:	lea    0x0(,%rbp,8),%r12
   56c0c:	mov    %r12,%rdi
   56c0f:	call   *0x83d9b(%rip)        # da9b0 <malloc@GLIBC_2.2.5>
   56c15:	test   %rax,%rax
   56c18:	je     57742 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc52>
   56c1e:	mov    %rax,%r15
   56c21:	mov    %rax,%rdi
   56c24:	mov    %rbx,%rsi
   56c27:	mov    %r12,%rdx
   56c2a:	call   *0x83da0(%rip)        # da9d0 <memcpy@GLIBC_2.14>
   56c30:	mov    %rbp,%rax
   56c33:	neg    %rax
   56c36:	xor    %r12d,%r12d
   56c39:	xor    %ecx,%ecx
   56c3b:	lea    0x0(,%rcx,8),%rdx
   56c43:	mov    (%r15,%rcx,8),%rsi
   56c47:	inc    %rcx
   56c4a:	neg    %rdx
   56c4d:	nopl   (%rax)
   56c50:	mov    %rsi,%rdi
   56c53:	mov    (%r15,%rcx,8),%rsi
   56c57:	cmp    %rsi,%rdi
   56c5a:	ja     56c6d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x17d>
   56c5c:	inc    %rcx
   56c5f:	add    $0xfffffffffffffff8,%rdx
   56c63:	cmp    %rcx,%rbp
   56c66:	jne    56c50 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x160>
   56c68:	jmp    56cfc <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x20c>
   56c6d:	cmp    %rcx,%rbp
   56c70:	jb     57718 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc28>
   56c76:	test   %rcx,%rcx
   56c79:	je     57733 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc43>
   56c7f:	mov    %rbp,%rsi
   56c82:	sub    %rcx,%rsi
   56c85:	je     57750 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc60>
   56c8b:	mov    -0x8(%r15,%rcx,8),%r8
   56c90:	mov    (%r15,%rcx,8),%rdi
   56c94:	mov    %rdi,-0x8(%r15,%rcx,8)
   56c99:	mov    %r8,(%r15,%rcx,8)
   56c9d:	inc    %r12
   56ca0:	cmp    %rbp,%r12
   56ca3:	ja     57073 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x583>
   56ca9:	lea    (%r15,%rcx,8),%r8
   56cad:	add    $0xfffffffffffffff8,%r8
   56cb1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56cc0:	test   %rdx,%rdx
   56cc3:	je     56cf2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x202>
   56cc5:	mov    -0x8(%r8),%r9
   56cc9:	cmp    %rdi,%r9
   56ccc:	jbe    56cf2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x202>
   56cce:	mov    %rdi,-0x8(%r8)
   56cd2:	mov    %r9,(%r8)
   56cd5:	add    $0xfffffffffffffff8,%r8
   56cd9:	add    $0x8,%rdx
   56cdd:	lea    (%rax,%r12,1),%r9
   56ce1:	inc    %r9
   56ce4:	inc    %r12
   56ce7:	cmp    $0x1,%r9
   56ceb:	jne    56cc0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1d0>
   56ced:	jmp    57073 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x583>
   56cf2:	cmp    $0x2,%rsi
   56cf6:	jae    56c3b <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14b>
   56cfc:	mov    %r15,%rdi
   56cff:	call   *0x83cbb(%rip)        # da9c0 <free@GLIBC_2.2.5>
   56d05:	test   %r12,%r12
   56d08:	je     5705e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x56e>
   56d0e:	imul   (%rsp),%r12
   56d13:	mov    %r12,%rax
   56d16:	shr    $0x20,%rax
   56d1a:	je     572c0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7d0>
   56d20:	mov    %r12,%rax
   56d23:	xor    %edx,%edx
   56d25:	div    %rbp
   56d28:	jmp    572c7 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7d7>
   56d2d:	and    %r8b,%r9b
   56d30:	test   $0x1,%r9b
   56d34:	je     56e5c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x36c>
   56d3a:	cmp    $0x401,%rsi
   56d41:	mov    $0x401,%r9d
   56d47:	cmovb  %rsi,%r9
   56d4b:	mov    (%rdi),%rax
   56d4e:	lea    0x0(,%r9,8),%r8d
   56d56:	lea    (%rdi,%r8,1),%rdx
   56d5a:	lea    0x8(%rdi),%rcx
   56d5e:	lea    -0x10(%r8),%rbx
   56d62:	cmp    $0x18,%rbx
   56d66:	jae    56d73 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x283>
   56d68:	mov    %rax,%r10
   56d6b:	mov    %rax,%r11
   56d6e:	jmp    56e00 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x310>
   56d73:	shr    $0x3,%rbx
   56d77:	inc    %rbx
   56d7a:	mov    %rbx,%r14
   56d7d:	and    $0xfffffffffffffffc,%r14
   56d81:	lea    (%rcx,%r14,8),%rcx
   56d85:	movq   %rax,%xmm2
   56d8a:	shufpd $0x2,-0x4cfb3(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56d93:	movq   %rax,%xmm1
   56d98:	pxor   %xmm0,%xmm0
   56d9c:	pcmpeqd %xmm3,%xmm3
   56da0:	xor    %r10d,%r10d
   56da3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56db0:	movdqu 0x8(%rdi,%r10,8),%xmm4
   56db7:	movdqu 0x18(%rdi,%r10,8),%xmm5
   56dbe:	por    %xmm4,%xmm1
   56dc2:	por    %xmm5,%xmm0
   56dc6:	andpd  %xmm4,%xmm2
   56dca:	pand   %xmm5,%xmm3
   56dce:	add    $0x4,%r10
   56dd2:	cmp    %r10,%r14
   56dd5:	jne    56db0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x2c0>
   56dd7:	pand   %xmm2,%xmm3
   56ddb:	pshufd $0xee,%xmm3,%xmm2
   56de0:	pand   %xmm3,%xmm2
   56de4:	movq   %xmm2,%r10
   56de9:	por    %xmm1,%xmm0
   56ded:	pshufd $0xee,%xmm0,%xmm1
   56df2:	por    %xmm0,%xmm1
   56df6:	movq   %xmm1,%r11
   56dfb:	cmp    %r14,%rbx
   56dfe:	je     56e12 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x322>
   56e00:	mov    (%rcx),%rbx
   56e03:	or     %rbx,%r11
   56e06:	and    %rbx,%r10
   56e09:	add    $0x8,%rcx
   56e0d:	cmp    %rdx,%rcx
   56e10:	jne    56e00 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x310>
   56e12:	mov    %r10,%rcx
   56e15:	xor    %r11,%rcx
   56e18:	cmp    $0xffffffffffffffff,%rcx
   56e1c:	je     56e50 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x360>
   56e1e:	lea    0x0(,%rsi,8),%rbx
   56e26:	cmp    %rbx,%r8
   56e29:	je     56f55 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x465>
   56e2f:	not    %r9
   56e32:	add    %rsi,%r9
   56e35:	movabs $0x1fffffffffffffff,%rbx
   56e3f:	and    %r9,%rbx
   56e42:	cmp    $0x3,%rbx
   56e46:	jae    56ea5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3b5>
   56e48:	mov    %r11,%rcx
   56e4b:	jmp    56f2f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x43f>
   56e50:	mov    $0xffffffffffffffff,%rcx
   56e57:	jmp    56f5e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x46e>
   56e5c:	mov    (%rdi),%r8
   56e5f:	lea    0x0(,%rsi,8),%rax
   56e67:	mov    %rax,%r9
   56e6a:	neg    %r9
   56e6d:	mov    $0x10,%ecx
   56e72:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56e80:	mov    -0x8(%rdi,%rcx,1),%rdx
   56e85:	cmp    %r8,%rdx
   56e88:	jne    5701c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x52c>
   56e8e:	lea    (%r9,%rcx,1),%rdx
   56e92:	add    $0x8,%rdx
   56e96:	add    $0x8,%rcx
   56e9a:	cmp    $0x8,%rdx
   56e9e:	jne    56e80 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x390>
   56ea0:	jmp    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56ea5:	inc    %rbx
   56ea8:	mov    %rbx,%r9
   56eab:	and    $0xfffffffffffffffc,%r9
   56eaf:	lea    (%rdx,%r9,8),%rdx
   56eb3:	movq   %r10,%xmm2
   56eb8:	shufpd $0x2,-0x4d0e1(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56ec1:	movq   %r11,%xmm1
   56ec6:	lea    (%r8,%rdi,1),%rcx
   56eca:	add    $0x10,%rcx
   56ece:	pxor   %xmm0,%xmm0
   56ed2:	pcmpeqd %xmm3,%xmm3
   56ed6:	xor    %r8d,%r8d
   56ed9:	nopl   0x0(%rax)
   56ee0:	movdqu -0x10(%rcx,%r8,8),%xmm4
   56ee7:	movdqu (%rcx,%r8,8),%xmm5
   56eed:	por    %xmm4,%xmm1
   56ef1:	por    %xmm5,%xmm0
   56ef5:	andpd  %xmm4,%xmm2
   56ef9:	pand   %xmm5,%xmm3
   56efd:	add    $0x4,%r8
   56f01:	cmp    %r8,%r9
   56f04:	jne    56ee0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3f0>
   56f06:	pand   %xmm2,%xmm3
   56f0a:	pshufd $0xee,%xmm3,%xmm2
   56f0f:	pand   %xmm3,%xmm2
   56f13:	movq   %xmm2,%r10
   56f18:	por    %xmm1,%xmm0
   56f1c:	pshufd $0xee,%xmm0,%xmm1
   56f21:	por    %xmm0,%xmm1
   56f25:	movq   %xmm1,%rcx
   56f2a:	cmp    %r9,%rbx
   56f2d:	je     56f52 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x462>
   56f2f:	lea    (%rdi,%rsi,8),%r8
   56f33:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56f40:	mov    (%rdx),%r9
   56f43:	add    $0x8,%rdx
   56f47:	or     %r9,%rcx
   56f4a:	and    %r9,%r10
   56f4d:	cmp    %r8,%rdx
   56f50:	jne    56f40 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x450>
   56f52:	xor    %r10,%rcx
   56f55:	test   %rcx,%rcx
   56f58:	je     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56f5e:	lea    0x0(,%rsi,8),%rdx
   56f66:	mov    %rdx,%r10
   56f69:	neg    %r10
   56f6c:	mov    $0x10,%r8d
   56f72:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56f80:	mov    -0x8(%rdi,%r8,1),%r9
   56f85:	cmp    %rax,%r9
   56f88:	jne    56fa1 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4b1>
   56f8a:	lea    (%r10,%r8,1),%r9
   56f8e:	add    $0x8,%r9
   56f92:	add    $0x8,%r8
   56f96:	cmp    $0x8,%r9
   56f9a:	jne    56f80 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x490>
   56f9c:	jmp    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56fa1:	mov    %r8,%r10
   56fa4:	sub    %rdx,%r10
   56fa7:	cmp    %r9,%rax
   56faa:	jae    56fde <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4ee>
   56fac:	test   %r10,%r10
   56faf:	je     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56fb5:	data16 cs nopw 0x0(%rax,%rax,1)
   56fc0:	mov    %r9,%rax
   56fc3:	mov    (%rdi,%r8,1),%r9
   56fc7:	cmp    %r9,%rax
   56fca:	ja     574aa <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9ba>
   56fd0:	add    $0x8,%r8
   56fd4:	cmp    %r8,%rdx
   56fd7:	jne    56fc0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4d0>
   56fd9:	jmp    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   56fde:	test   %r10,%r10
   56fe1:	je     57009 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x519>
   56fe3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56ff0:	mov    %r9,%rax
   56ff3:	mov    (%rdi,%r8,1),%r9
   56ff7:	cmp    %r9,%rax
   56ffa:	jb     574aa <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9ba>
   57000:	add    $0x8,%r8
   57004:	cmp    %r8,%rdx
   57007:	jne    56ff0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x500>
   57009:	mov    %rsi,%rax
   5700c:	shr    $1,%rax
   5700f:	cmp    $0x8,%rsi
   57013:	jae    57088 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x598>
   57015:	xor    %ecx,%ecx
   57017:	jmp    57106 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x616>
   5701c:	mov    %rcx,%r9
   5701f:	sub    %rax,%r9
   57022:	cmp    %rdx,%r8
   57025:	jae    5727f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x78f>
   5702b:	test   %r9,%r9
   5702e:	je     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   57034:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57040:	mov    %rdx,%r8
   57043:	mov    (%rdi,%rcx,1),%rdx
   57047:	cmp    %rdx,%r8
   5704a:	ja     56bde <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xee>
   57050:	add    $0x8,%rcx
   57054:	cmp    %rcx,%rax
   57057:	jne    57040 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x550>
   57059:	jmp    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   5705e:	mov    (%rsp),%rsi
   57062:	cmp    %r14,%rsi
   57065:	mov    %rbx,%rdi
   57068:	ja     572e6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7f6>
   5706e:	jmp    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   57073:	mov    %r15,%rdi
   57076:	call   *0x83944(%rip)        # da9c0 <free@GLIBC_2.2.5>
   5707c:	mov    (%rsp),%rsi
   57080:	mov    %rbx,%rdi
   57083:	jmp    572e6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7f6>
   57088:	movabs $0x7fffffffffffffc,%rcx
   57092:	and    %rax,%rcx
   57095:	mov    %rax,%r8
   57098:	and    $0xfffffffffffffffc,%r8
   5709c:	neg    %r8
   5709f:	lea    0x10(%rdi),%r9
   570a3:	lea    (%rdx,%rdi,1),%r10
   570a7:	add    $0xfffffffffffffff0,%r10
   570ab:	xor    %r11d,%r11d
   570ae:	xchg   %ax,%ax
   570b0:	movdqu -0x10(%r9),%xmm0
   570b6:	movdqu (%r9),%xmm1
   570bb:	movdqu -0x10(%r10,%r11,8),%xmm2
   570c2:	movdqu (%r10,%r11,8),%xmm3
   570c8:	pshufd $0x4e,%xmm3,%xmm3
   570cd:	pshufd $0x4e,%xmm2,%xmm2
   570d2:	movdqu %xmm3,-0x10(%r9)
   570d8:	movdqu %xmm2,(%r9)
   570dd:	pshufd $0x4e,%xmm0,%xmm0
   570e2:	pshufd $0x4e,%xmm1,%xmm1
   570e7:	movdqu %xmm0,(%r10,%r11,8)
   570ed:	movdqu %xmm1,-0x10(%r10,%r11,8)
   570f4:	add    $0xfffffffffffffffc,%r11
   570f8:	add    $0x20,%r9
   570fc:	cmp    %r11,%r8
   570ff:	jne    570b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x5c0>
   57101:	cmp    %rcx,%rax
   57104:	je     5713a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x64a>
   57106:	neg    %rax
   57109:	lea    (%rdi,%rcx,8),%r8
   5710d:	neg    %rcx
   57110:	add    %rdi,%rdx
   57113:	add    $0xfffffffffffffff8,%rdx
   57117:	nopw   0x0(%rax,%rax,1)
   57120:	mov    (%r8),%r9
   57123:	mov    (%rdx,%rcx,8),%r10
   57127:	mov    %r10,(%r8)
   5712a:	mov    %r9,(%rdx,%rcx,8)
   5712e:	dec    %rcx
   57131:	add    $0x8,%r8
   57135:	cmp    %rcx,%rax
   57138:	jne    57120 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x630>
   5713a:	movabs $0x7ffffffffffffffc,%rcx
   57144:	lea    0x10(%rdi),%r8
   57148:	lea    -0x10(%rdi),%r9
   5714c:	lea    -0x8(%rdi),%r10
   57150:	xor    %eax,%eax
   57152:	jmp    57169 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x679>
   57154:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57160:	cmp    %rsi,%rax
   57163:	jae    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   57169:	mov    %rax,%rdx
   5716c:	lea    0x1(%rax),%r11
   57170:	cmp    %r11,%rsi
   57173:	mov    %r11,%rax
   57176:	cmova  %rsi,%rax
   5717a:	lea    -0x1(%rax),%rbx
   5717e:	cmp    %rbx,%rdx
   57181:	je     571a3 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6b3>
   57183:	mov    (%rdi,%rdx,8),%rbx
   57187:	nopw   0x0(%rax,%rax,1)
   57190:	cmp    %rbx,(%rdi,%r11,8)
   57194:	jne    571a0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6b0>
   57196:	inc    %r11
   57199:	cmp    %r11,%rax
   5719c:	jne    57190 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6a0>
   5719e:	jmp    571a3 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6b3>
   571a0:	mov    %r11,%rax
   571a3:	mov    %rax,%rbx
   571a6:	sub    %rdx,%rbx
   571a9:	jb     576ef <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbff>
   571af:	mov    %rbx,%r11
   571b2:	shr    $1,%r11
   571b5:	je     57160 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x670>
   571b7:	cmp    $0x8,%rbx
   571bb:	jae    571d0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6e0>
   571bd:	xor    %ebx,%ebx
   571bf:	jmp    5724a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x75a>
   571c4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   571d0:	mov    %r11,%rbx
   571d3:	and    %rcx,%rbx
   571d6:	mov    %r11,%r14
   571d9:	and    $0xfffffffffffffffc,%r14
   571dd:	neg    %r14
   571e0:	lea    (%r8,%rdx,8),%r15
   571e4:	lea    (%r9,%rax,8),%r12
   571e8:	xor    %r13d,%r13d
   571eb:	nopl   0x0(%rax,%rax,1)
   571f0:	movdqu -0x10(%r15),%xmm0
   571f6:	movdqu (%r15),%xmm1
   571fb:	movdqu -0x10(%r12,%r13,8),%xmm2
   57202:	movdqu (%r12,%r13,8),%xmm3
   57208:	pshufd $0x4e,%xmm3,%xmm3
   5720d:	pshufd $0x4e,%xmm2,%xmm2
   57212:	movdqu %xmm3,-0x10(%r15)
   57218:	movdqu %xmm2,(%r15)
   5721d:	pshufd $0x4e,%xmm0,%xmm0
   57222:	pshufd $0x4e,%xmm1,%xmm1
   57227:	movdqu %xmm0,(%r12,%r13,8)
   5722d:	movdqu %xmm1,-0x10(%r12,%r13,8)
   57234:	add    $0xfffffffffffffffc,%r13
   57238:	add    $0x20,%r15
   5723c:	cmp    %r13,%r14
   5723f:	jne    571f0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x700>
   57241:	cmp    %rbx,%r11
   57244:	je     57160 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x670>
   5724a:	lea    (%rdi,%rdx,8),%rdx
   5724e:	lea    0x0(,%rbx,8),%r15
   57256:	lea    (%r10,%rax,8),%r14
   5725a:	sub    %r15,%r14
   5725d:	nopl   (%rax)
   57260:	mov    (%rdx,%rbx,8),%r15
   57264:	mov    (%r14),%r12
   57267:	mov    %r12,(%rdx,%rbx,8)
   5726b:	mov    %r15,(%r14)
   5726e:	inc    %rbx
   57271:	add    $0xfffffffffffffff8,%r14
   57275:	cmp    %rbx,%r11
   57278:	jne    57260 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x770>
   5727a:	jmp    57160 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x670>
   5727f:	test   %r9,%r9
   57282:	je     572a9 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b9>
   57284:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57290:	mov    %rdx,%r8
   57293:	mov    (%rdi,%rcx,1),%rdx
   57297:	cmp    %rdx,%r8
   5729a:	jb     56bde <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xee>
   572a0:	add    $0x8,%rcx
   572a4:	cmp    %rcx,%rax
   572a7:	jne    57290 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7a0>
   572a9:	mov    %rsi,%rcx
   572ac:	shr    $1,%rcx
   572af:	cmp    $0x8,%rsi
   572b3:	jae    57505 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa15>
   572b9:	xor    %edx,%edx
   572bb:	jmp    57586 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa96>
   572c0:	mov    %r12d,%eax
   572c3:	xor    %edx,%edx
   572c5:	div    %ebp
   572c7:	mov    %rbx,%rdi
   572ca:	mov    (%rsp),%rsi
   572ce:	mov    %rax,%rdx
   572d1:	call   58160 <sort_research_rs::other::rust_tessera_radix::engine::budgeted_insertion_sort::<u64, u64, <u64 as sort_research_rs::other::radix_support::Key>::key>>
   572d6:	mov    %rbx,%rdi
   572d9:	mov    (%rsp),%rsi
   572dd:	test   %rax,%rax
   572e0:	jne    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   572e6:	cmp    $0x401,%rsi
   572ed:	mov    $0x401,%r9d
   572f3:	cmovb  %rsi,%r9
   572f7:	lea    0x0(,%r9,8),%r8d
   572ff:	lea    (%rdi,%r8,1),%rax
   57303:	mov    (%rdi),%rdx
   57306:	lea    0x8(%rdi),%rcx
   5730a:	lea    -0x10(%r8),%r11
   5730e:	cmp    $0x18,%r11
   57312:	jae    5731c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x82c>
   57314:	mov    %rdx,%r10
   57317:	jmp    573a0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8b0>
   5731c:	shr    $0x3,%r11
   57320:	inc    %r11
   57323:	mov    %r11,%rbx
   57326:	and    $0xfffffffffffffffc,%rbx
   5732a:	lea    (%rcx,%rbx,8),%rcx
   5732e:	movq   %rdx,%xmm2
   57333:	shufpd $0x2,-0x4d55c(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   5733c:	movq   %rdx,%xmm1
   57341:	pxor   %xmm0,%xmm0
   57345:	pcmpeqd %xmm3,%xmm3
   57349:	xor    %edx,%edx
   5734b:	nopl   0x0(%rax,%rax,1)
   57350:	movdqu 0x8(%rdi,%rdx,8),%xmm4
   57356:	movdqu 0x18(%rdi,%rdx,8),%xmm5
   5735c:	por    %xmm4,%xmm1
   57360:	por    %xmm5,%xmm0
   57364:	andpd  %xmm4,%xmm2
   57368:	pand   %xmm5,%xmm3
   5736c:	add    $0x4,%rdx
   57370:	cmp    %rdx,%rbx
   57373:	jne    57350 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x860>
   57375:	pand   %xmm2,%xmm3
   57379:	pshufd $0xee,%xmm3,%xmm2
   5737e:	pand   %xmm3,%xmm2
   57382:	movq   %xmm2,%rdx
   57387:	por    %xmm1,%xmm0
   5738b:	pshufd $0xee,%xmm0,%xmm1
   57390:	por    %xmm0,%xmm1
   57394:	movq   %xmm1,%r10
   57399:	cmp    %rbx,%r11
   5739c:	je     573b2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8c2>
   5739e:	xchg   %ax,%ax
   573a0:	mov    (%rcx),%r11
   573a3:	or     %r11,%r10
   573a6:	and    %r11,%rdx
   573a9:	add    $0x8,%rcx
   573ad:	cmp    %rax,%rcx
   573b0:	jne    573a0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8b0>
   573b2:	mov    %rdx,%rcx
   573b5:	xor    %r10,%rcx
   573b8:	cmp    $0xffffffffffffffff,%rcx
   573bc:	je     573f0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x900>
   573be:	lea    0x0(,%rsi,8),%r11
   573c6:	cmp    %r11,%r8
   573c9:	je     574a5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9b5>
   573cf:	not    %r9
   573d2:	add    %rsi,%r9
   573d5:	movabs $0x1fffffffffffffff,%r11
   573df:	and    %r9,%r11
   573e2:	cmp    $0x3,%r11
   573e6:	jae    573fc <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x90c>
   573e8:	mov    %r10,%rcx
   573eb:	jmp    5747d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x98d>
   573f0:	mov    $0xffffffffffffffff,%rcx
   573f7:	jmp    574aa <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9ba>
   573fc:	inc    %r11
   573ff:	mov    %r11,%r9
   57402:	and    $0xfffffffffffffffc,%r9
   57406:	lea    (%rax,%r9,8),%rax
   5740a:	movq   %rdx,%xmm2
   5740f:	shufpd $0x2,-0x4d638(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   57418:	movq   %r10,%xmm1
   5741d:	lea    (%r8,%rdi,1),%rcx
   57421:	add    $0x10,%rcx
   57425:	pxor   %xmm0,%xmm0
   57429:	pcmpeqd %xmm3,%xmm3
   5742d:	xor    %edx,%edx
   5742f:	nop
   57430:	movdqu -0x10(%rcx,%rdx,8),%xmm4
   57436:	movdqu (%rcx,%rdx,8),%xmm5
   5743b:	por    %xmm4,%xmm1
   5743f:	por    %xmm5,%xmm0
   57443:	andpd  %xmm4,%xmm2
   57447:	pand   %xmm5,%xmm3
   5744b:	add    $0x4,%rdx
   5744f:	cmp    %rdx,%r9
   57452:	jne    57430 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x940>
   57454:	pand   %xmm2,%xmm3
   57458:	pshufd $0xee,%xmm3,%xmm2
   5745d:	pand   %xmm3,%xmm2
   57461:	movq   %xmm2,%rdx
   57466:	por    %xmm1,%xmm0
   5746a:	pshufd $0xee,%xmm0,%xmm1
   5746f:	por    %xmm0,%xmm1
   57473:	movq   %xmm1,%rcx
   57478:	cmp    %r9,%r11
   5747b:	je     574a2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9b2>
   5747d:	lea    (%rdi,%rsi,8),%r8
   57481:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57490:	mov    (%rax),%r9
   57493:	add    $0x8,%rax
   57497:	or     %r9,%rcx
   5749a:	and    %r9,%rdx
   5749d:	cmp    %r8,%rax
   574a0:	jne    57490 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9a0>
   574a2:	xor    %rdx,%rcx
   574a5:	test   %rcx,%rcx
   574a8:	je     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   574aa:	movq   $0x0,0x8(%rsp)
   574b3:	movq   $0x8,0x10(%rsp)
   574bc:	movq   $0x0,0x18(%rsp)
   574c5:	mov    %r13,0x20(%rsp)
   574ca:	lea    0x8(%rsp),%rax
   574cf:	mov    %rdi,%r8
   574d2:	mov    %rax,%rdi
   574d5:	mov    %rsi,%rdx
   574d8:	mov    %r8,%rsi
   574db:	xor    %r8d,%r8d
   574de:	call   664c0 <<sort_research_rs::other::rust_tessera_radix::engine::Sorter<u64>>::sort_node>
   574e3:	cmpq   $0x0,0x8(%rsp)
   574e9:	je     574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   574eb:	mov    0x10(%rsp),%rdi
   574f0:	call   *0x834ca(%rip)        # da9c0 <free@GLIBC_2.2.5>
   574f6:	add    $0x28,%rsp
   574fa:	pop    %rbx
   574fb:	pop    %r12
   574fd:	pop    %r13
   574ff:	pop    %r14
   57501:	pop    %r15
   57503:	pop    %rbp
   57504:	ret
   57505:	movabs $0x7fffffffffffffc,%rdx
   5750f:	and    %rcx,%rdx
   57512:	mov    %rcx,%r8
   57515:	and    $0xfffffffffffffffc,%r8
   57519:	neg    %r8
   5751c:	lea    0x10(%rdi),%r9
   57520:	lea    (%rax,%rdi,1),%r10
   57524:	add    $0xfffffffffffffff0,%r10
   57528:	xor    %r11d,%r11d
   5752b:	nopl   0x0(%rax,%rax,1)
   57530:	movdqu -0x10(%r9),%xmm0
   57536:	movdqu (%r9),%xmm1
   5753b:	movdqu -0x10(%r10,%r11,8),%xmm2
   57542:	movdqu (%r10,%r11,8),%xmm3
   57548:	pshufd $0x4e,%xmm3,%xmm3
   5754d:	pshufd $0x4e,%xmm2,%xmm2
   57552:	movdqu %xmm3,-0x10(%r9)
   57558:	movdqu %xmm2,(%r9)
   5755d:	pshufd $0x4e,%xmm0,%xmm0
   57562:	pshufd $0x4e,%xmm1,%xmm1
   57567:	movdqu %xmm0,(%r10,%r11,8)
   5756d:	movdqu %xmm1,-0x10(%r10,%r11,8)
   57574:	add    $0xfffffffffffffffc,%r11
   57578:	add    $0x20,%r9
   5757c:	cmp    %r11,%r8
   5757f:	jne    57530 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa40>
   57581:	cmp    %rdx,%rcx
   57584:	je     575ba <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaca>
   57586:	neg    %rcx
   57589:	lea    (%rdi,%rdx,8),%r8
   5758d:	neg    %rdx
   57590:	add    %rdi,%rax
   57593:	add    $0xfffffffffffffff8,%rax
   57597:	nopw   0x0(%rax,%rax,1)
   575a0:	mov    (%r8),%r9
   575a3:	mov    (%rax,%rdx,8),%r10
   575a7:	mov    %r10,(%r8)
   575aa:	mov    %r9,(%rax,%rdx,8)
   575ae:	dec    %rdx
   575b1:	add    $0x8,%r8
   575b5:	cmp    %rdx,%rcx
   575b8:	jne    575a0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xab0>
   575ba:	movabs $0x7ffffffffffffffc,%rcx
   575c4:	lea    0x10(%rdi),%r8
   575c8:	lea    -0x10(%rdi),%r9
   575cc:	lea    -0x8(%rdi),%r10
   575d0:	xor    %eax,%eax
   575d2:	jmp    575e9 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaf9>
   575d4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   575e0:	cmp    %rsi,%rax
   575e3:	jae    574f6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa06>
   575e9:	mov    %rax,%rdx
   575ec:	lea    0x1(%rax),%r11
   575f0:	cmp    %r11,%rsi
   575f3:	mov    %r11,%rax
   575f6:	cmova  %rsi,%rax
   575fa:	lea    -0x1(%rax),%rbx
   575fe:	cmp    %rbx,%rdx
   57601:	je     57623 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb33>
   57603:	mov    (%rdi,%rdx,8),%rbx
   57607:	nopw   0x0(%rax,%rax,1)
   57610:	cmp    %rbx,(%rdi,%r11,8)
   57614:	jne    57620 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb30>
   57616:	inc    %r11
   57619:	cmp    %r11,%rax
   5761c:	jne    57610 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb20>
   5761e:	jmp    57623 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb33>
   57620:	mov    %r11,%rax
   57623:	mov    %rax,%rbx
   57626:	sub    %rdx,%rbx
   57629:	jb     576ef <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbff>
   5762f:	mov    %rbx,%r11
   57632:	shr    $1,%r11
   57635:	je     575e0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaf0>
   57637:	cmp    $0x8,%rbx
   5763b:	jae    57641 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb51>
   5763d:	xor    %ebx,%ebx
   5763f:	jmp    576ba <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbca>
   57641:	mov    %r11,%rbx
   57644:	and    %rcx,%rbx
   57647:	mov    %r11,%r14
   5764a:	and    $0xfffffffffffffffc,%r14
   5764e:	neg    %r14
   57651:	lea    (%r8,%rdx,8),%r15
   57655:	lea    (%r9,%rax,8),%r12
   57659:	xor    %r13d,%r13d
   5765c:	nopl   0x0(%rax)
   57660:	movdqu -0x10(%r15),%xmm0
   57666:	movdqu (%r15),%xmm1
   5766b:	movdqu -0x10(%r12,%r13,8),%xmm2
   57672:	movdqu (%r12,%r13,8),%xmm3
   57678:	pshufd $0x4e,%xmm3,%xmm3
   5767d:	pshufd $0x4e,%xmm2,%xmm2
   57682:	movdqu %xmm3,-0x10(%r15)
   57688:	movdqu %xmm2,(%r15)
   5768d:	pshufd $0x4e,%xmm0,%xmm0
   57692:	pshufd $0x4e,%xmm1,%xmm1
   57697:	movdqu %xmm0,(%r12,%r13,8)
   5769d:	movdqu %xmm1,-0x10(%r12,%r13,8)
   576a4:	add    $0xfffffffffffffffc,%r13
   576a8:	add    $0x20,%r15
   576ac:	cmp    %r13,%r14
   576af:	jne    57660 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb70>
   576b1:	cmp    %rbx,%r11
   576b4:	je     575e0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaf0>
   576ba:	lea    (%rdi,%rdx,8),%rdx
   576be:	lea    0x0(,%rbx,8),%r15
   576c6:	lea    (%r10,%rax,8),%r14
   576ca:	sub    %r15,%r14
   576cd:	nopl   (%rax)
   576d0:	mov    (%rdx,%rbx,8),%r15
   576d4:	mov    (%r14),%r12
   576d7:	mov    %r12,(%rdx,%rbx,8)
   576db:	mov    %r15,(%r14)
   576de:	inc    %rbx
   576e1:	add    $0xfffffffffffffff8,%r14
   576e5:	cmp    %rbx,%r11
   576e8:	jne    576d0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbe0>
   576ea:	jmp    575e0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaf0>
   576ef:	lea    0x80d42(%rip),%rcx        # d8438 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7f8>
   576f6:	mov    %rdx,%rdi
   576f9:	mov    %rsi,%rdx
   576fc:	mov    %rax,%rsi
   576ff:	call   *0x83333(%rip)        # daa38 <_DYNAMIC+0x280>
   57705:	lea    0x80ce4(%rip),%rcx        # d83f0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7b0>
   5770c:	mov    %rax,%rdi
   5770f:	mov    %rsi,%rdx
   57712:	call   *0x83320(%rip)        # daa38 <_DYNAMIC+0x280>
   57718:	lea    -0x4ab5e(%rip),%rdi        # cbc1 <panic_unwind::imp::CANARY+0x1f2>
   5771f:	lea    0x80d72(%rip),%rdx        # d8498 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x858>
   57726:	mov    $0x13,%esi
   5772b:	call   *0x83337(%rip)        # daa68 <_DYNAMIC+0x2b0>
   57731:	jmp    57761 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc71>
   57733:	lea    0x80d76(%rip),%rdi        # d84b0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x870>
   5773a:	call   *0x83460(%rip)        # daba0 <_DYNAMIC+0x3e8>
   57740:	jmp    57761 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc71>
   57742:	mov    $0x8,%edi
   57747:	mov    %r12,%rsi
   5774a:	call   *0x832a8(%rip)        # da9f8 <_DYNAMIC+0x240>
   57750:	lea    0x80d71(%rip),%rdx        # d84c8 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x888>
   57757:	xor    %edi,%edi
   57759:	xor    %esi,%esi
   5775b:	call   *0x832a7(%rip)        # daa08 <_DYNAMIC+0x250>
   57761:	ud2
   57763:	mov    %rax,%rbx
   57766:	cmpq   $0x0,0x8(%rsp)
   5776c:	je     57781 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc91>
   5776e:	mov    0x10(%rsp),%rdi
   57773:	jmp    5777b <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc8b>
   57775:	mov    %rax,%rbx
   57778:	mov    %r15,%rdi
   5777b:	call   *0x8323f(%rip)        # da9c0 <free@GLIBC_2.2.5>
   57781:	mov    %rbx,%rdi
   57784:	call   d6050 <_Unwind_Resume@plt>
