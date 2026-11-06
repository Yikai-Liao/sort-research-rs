
/root/code/sort-research-rs/logs/tessera-cleanup/api-compat-bench:     file format elf64-x86-64


Disassembly of section .text:

0000000000056b30 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>>:
   56b30:	push   %rbp
   56b31:	push   %r15
   56b33:	push   %r14
   56b35:	push   %r13
   56b37:	push   %r12
   56b39:	push   %rbx
   56b3a:	sub    $0x28,%rsp
   56b3e:	cmp    $0x2,%rsi
   56b42:	jb     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   56b48:	cmp    $0x9,%rdx
   56b4c:	mov    $0x8,%eax
   56b51:	cmovae %rdx,%rax
   56b55:	cmp    $0x7fff8,%rax
   56b5b:	mov    $0x7fff8,%r13d
   56b61:	cmovb  %rax,%r13
   56b65:	mov    %rsi,%rcx
   56b68:	shr    $0xc,%rcx
   56b6c:	xor    %eax,%eax
   56b6e:	mov    %rcx,%rdx
   56b71:	sub    $0x1,%rdx
   56b75:	cmovae %rdx,%rax
   56b79:	mov    %rsi,%rdx
   56b7c:	sub    %rax,%rdx
   56b7f:	jb     57735 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc05>
   56b85:	cmp    $0x2,%rdx
   56b89:	jb     56d7a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x24a>
   56b8f:	mov    (%rdi,%rax,8),%r8
   56b93:	cmp    0x8(%rdi,%rax,8),%r8
   56b98:	setae  %r8b
   56b9c:	setbe  %r9b
   56ba0:	mov    %rcx,%r10
   56ba3:	shl    $0x4,%r10
   56ba7:	cmp    $0x1,%rcx
   56bab:	mov    $0x1,%r11d
   56bb1:	cmovb  %rcx,%r11
   56bb5:	mov    %r11d,%ebx
   56bb8:	shl    $0x4,%ebx
   56bbb:	sub    %rbx,%r10
   56bbe:	add    %rdi,%r10
   56bc1:	add    $0x10,%r10
   56bc5:	shl    $0x3,%rcx
   56bc9:	shl    $0x3,%r11d
   56bcd:	sub    %r11,%rcx
   56bd0:	add    $0x8,%rcx
   56bd4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   56be0:	dec    %rdx
   56be3:	sub    %rax,%rdx
   56be6:	jb     56d6d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x23d>
   56bec:	cmp    $0x1,%rdx
   56bf0:	jbe    56d6d <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x23d>
   56bf6:	mov    -0x8(%r10),%r11
   56bfa:	cmp    (%r10),%r11
   56bfd:	lea    (%r10,%rcx,1),%r11
   56c01:	setbe  %r10b
   56c05:	setae  %bl
   56c08:	and    %r10b,%r9b
   56c0b:	and    %bl,%r8b
   56c0e:	mov    %r11,%r10
   56c11:	test   %r9b,%r9b
   56c14:	jne    56be0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb0>
   56c16:	mov    %r11,%r10
   56c19:	test   %r8b,%r8b
   56c1c:	jne    56be0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb0>
   56c1e:	mov    %rdi,%rbx
   56c21:	mov    %r13,%rax
   56c24:	shr    $0x3,%rax
   56c28:	cmp    $0x3,%rax
   56c2c:	mov    $0x2,%r14d
   56c32:	cmovae %rax,%r14
   56c36:	cmp    %rsi,%r14
   56c39:	mov    %rsi,(%rsp)
   56c3d:	mov    %rsi,%rbp
   56c40:	cmovb  %r14,%rbp
   56c44:	lea    0x0(,%rbp,8),%r12
   56c4c:	mov    %r12,%rdi
   56c4f:	call   *0x83b4b(%rip)        # da7a0 <malloc@GLIBC_2.2.5>
   56c55:	test   %rax,%rax
   56c58:	je     57772 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc42>
   56c5e:	mov    %rax,%r15
   56c61:	mov    %rax,%rdi
   56c64:	mov    %rbx,%rsi
   56c67:	mov    %r12,%rdx
   56c6a:	call   *0x83b50(%rip)        # da7c0 <memcpy@GLIBC_2.14>
   56c70:	mov    %rbp,%rax
   56c73:	neg    %rax
   56c76:	xor    %r12d,%r12d
   56c79:	xor    %ecx,%ecx
   56c7b:	lea    0x0(,%rcx,8),%rdx
   56c83:	mov    (%r15,%rcx,8),%rsi
   56c87:	inc    %rcx
   56c8a:	neg    %rdx
   56c8d:	nopl   (%rax)
   56c90:	mov    %rsi,%rdi
   56c93:	mov    (%r15,%rcx,8),%rsi
   56c97:	cmp    %rsi,%rdi
   56c9a:	ja     56cad <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x17d>
   56c9c:	inc    %rcx
   56c9f:	add    $0xfffffffffffffff8,%rdx
   56ca3:	cmp    %rcx,%rbp
   56ca6:	jne    56c90 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x160>
   56ca8:	jmp    56d3c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x20c>
   56cad:	cmp    %rcx,%rbp
   56cb0:	jb     57748 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc18>
   56cb6:	test   %rcx,%rcx
   56cb9:	je     57763 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc33>
   56cbf:	mov    %rbp,%rsi
   56cc2:	sub    %rcx,%rsi
   56cc5:	je     57780 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc50>
   56ccb:	mov    -0x8(%r15,%rcx,8),%r8
   56cd0:	mov    (%r15,%rcx,8),%rdi
   56cd4:	mov    %rdi,-0x8(%r15,%rcx,8)
   56cd9:	mov    %r8,(%r15,%rcx,8)
   56cdd:	inc    %r12
   56ce0:	cmp    %rbp,%r12
   56ce3:	ja     5728f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x75f>
   56ce9:	lea    (%r15,%rcx,8),%r8
   56ced:	add    $0xfffffffffffffff8,%r8
   56cf1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56d00:	test   %rdx,%rdx
   56d03:	je     56d32 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x202>
   56d05:	mov    -0x8(%r8),%r9
   56d09:	cmp    %rdi,%r9
   56d0c:	jbe    56d32 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x202>
   56d0e:	mov    %rdi,-0x8(%r8)
   56d12:	mov    %r9,(%r8)
   56d15:	add    $0xfffffffffffffff8,%r8
   56d19:	add    $0x8,%rdx
   56d1d:	lea    (%rax,%r12,1),%r9
   56d21:	inc    %r9
   56d24:	inc    %r12
   56d27:	cmp    $0x1,%r9
   56d2b:	jne    56d00 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x1d0>
   56d2d:	jmp    5728f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x75f>
   56d32:	cmp    $0x2,%rsi
   56d36:	jae    56c7b <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x14b>
   56d3c:	mov    %r15,%rdi
   56d3f:	call   *0x83a6b(%rip)        # da7b0 <free@GLIBC_2.2.5>
   56d45:	test   %r12,%r12
   56d48:	je     57084 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x554>
   56d4e:	imul   (%rsp),%r12
   56d53:	mov    %r12,%rax
   56d56:	shr    $0x20,%rax
   56d5a:	je     572e6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7b6>
   56d60:	mov    %r12,%rax
   56d63:	xor    %edx,%edx
   56d65:	div    %rbp
   56d68:	jmp    572ed <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7bd>
   56d6d:	and    %r8b,%r9b
   56d70:	test   $0x1,%r9b
   56d74:	je     56e9c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x36c>
   56d7a:	cmp    $0x401,%rsi
   56d81:	mov    $0x401,%r10d
   56d87:	cmovb  %rsi,%r10
   56d8b:	mov    (%rdi),%rax
   56d8e:	lea    0x0(,%r10,8),%r9d
   56d96:	lea    (%rdi,%r9,1),%r8
   56d9a:	lea    0x8(%rdi),%rdx
   56d9e:	lea    -0x10(%r9),%r14
   56da2:	mov    %rdx,%rcx
   56da5:	mov    %rax,%r11
   56da8:	mov    %rax,%rbx
   56dab:	cmp    $0x18,%r14
   56daf:	jb     56e40 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x310>
   56db5:	shr    $0x3,%r14
   56db9:	inc    %r14
   56dbc:	mov    %r14,%r15
   56dbf:	and    $0xfffffffffffffffc,%r15
   56dc3:	lea    (%rdx,%r15,8),%rcx
   56dc7:	movq   %rax,%xmm2
   56dcc:	shufpd $0x2,-0x4cff5(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56dd5:	movq   %rax,%xmm1
   56dda:	pxor   %xmm0,%xmm0
   56dde:	pcmpeqd %xmm3,%xmm3
   56de2:	xor    %r11d,%r11d
   56de5:	data16 cs nopw 0x0(%rax,%rax,1)
   56df0:	movdqu 0x8(%rdi,%r11,8),%xmm4
   56df7:	movdqu 0x18(%rdi,%r11,8),%xmm5
   56dfe:	por    %xmm4,%xmm1
   56e02:	por    %xmm5,%xmm0
   56e06:	andpd  %xmm4,%xmm2
   56e0a:	pand   %xmm5,%xmm3
   56e0e:	add    $0x4,%r11
   56e12:	cmp    %r11,%r15
   56e15:	jne    56df0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x2c0>
   56e17:	pand   %xmm2,%xmm3
   56e1b:	pshufd $0xee,%xmm3,%xmm2
   56e20:	pand   %xmm3,%xmm2
   56e24:	movq   %xmm2,%r11
   56e29:	por    %xmm1,%xmm0
   56e2d:	pshufd $0xee,%xmm0,%xmm1
   56e32:	por    %xmm0,%xmm1
   56e36:	movq   %xmm1,%rbx
   56e3b:	cmp    %r15,%r14
   56e3e:	je     56e52 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x322>
   56e40:	mov    (%rcx),%r14
   56e43:	or     %r14,%rbx
   56e46:	and    %r14,%r11
   56e49:	add    $0x8,%rcx
   56e4d:	cmp    %r8,%rcx
   56e50:	jne    56e40 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x310>
   56e52:	mov    %r11,%rcx
   56e55:	xor    %rbx,%rcx
   56e58:	cmp    $0xffffffffffffffff,%rcx
   56e5c:	je     56e90 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x360>
   56e5e:	lea    0x0(,%rsi,8),%r14
   56e66:	cmp    %r14,%r9
   56e69:	je     56f85 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x455>
   56e6f:	not    %r10
   56e72:	add    %rsi,%r10
   56e75:	movabs $0x1fffffffffffffff,%r14
   56e7f:	and    %r10,%r14
   56e82:	cmp    $0x3,%r14
   56e86:	jae    56edc <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3ac>
   56e88:	mov    %rbx,%rcx
   56e8b:	jmp    56f5f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x42f>
   56e90:	mov    $0xffffffffffffffff,%rcx
   56e97:	jmp    56f8e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x45e>
   56e9c:	mov    (%rdi),%r8
   56e9f:	lea    0x0(,%rsi,8),%rax
   56ea7:	lea    -0x8(,%rsi,8),%r9
   56eaf:	xor    %ecx,%ecx
   56eb1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56ec0:	mov    0x8(%rdi,%rcx,1),%rdx
   56ec5:	cmp    %r8,%rdx
   56ec8:	jne    57042 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x512>
   56ece:	add    $0x8,%rcx
   56ed2:	cmp    %rcx,%r9
   56ed5:	jne    56ec0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x390>
   56ed7:	jmp    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   56edc:	inc    %r14
   56edf:	mov    %r14,%r10
   56ee2:	and    $0xfffffffffffffffc,%r10
   56ee6:	lea    (%r8,%r10,8),%r8
   56eea:	movq   %r11,%xmm2
   56eef:	shufpd $0x2,-0x4d118(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   56ef8:	movq   %rbx,%xmm1
   56efd:	lea    (%r9,%rdi,1),%rcx
   56f01:	add    $0x10,%rcx
   56f05:	pxor   %xmm0,%xmm0
   56f09:	pcmpeqd %xmm3,%xmm3
   56f0d:	xor    %r9d,%r9d
   56f10:	movdqu -0x10(%rcx,%r9,8),%xmm4
   56f17:	movdqu (%rcx,%r9,8),%xmm5
   56f1d:	por    %xmm4,%xmm1
   56f21:	por    %xmm5,%xmm0
   56f25:	andpd  %xmm4,%xmm2
   56f29:	pand   %xmm5,%xmm3
   56f2d:	add    $0x4,%r9
   56f31:	cmp    %r9,%r10
   56f34:	jne    56f10 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x3e0>
   56f36:	pand   %xmm2,%xmm3
   56f3a:	pshufd $0xee,%xmm3,%xmm2
   56f3f:	pand   %xmm3,%xmm2
   56f43:	movq   %xmm2,%r11
   56f48:	por    %xmm1,%xmm0
   56f4c:	pshufd $0xee,%xmm0,%xmm1
   56f51:	por    %xmm0,%xmm1
   56f55:	movq   %xmm1,%rcx
   56f5a:	cmp    %r10,%r14
   56f5d:	je     56f82 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x452>
   56f5f:	lea    (%rdi,%rsi,8),%r9
   56f63:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56f70:	mov    (%r8),%r10
   56f73:	add    $0x8,%r8
   56f77:	or     %r10,%rcx
   56f7a:	and    %r10,%r11
   56f7d:	cmp    %r9,%r8
   56f80:	jne    56f70 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x440>
   56f82:	xor    %r11,%rcx
   56f85:	test   %rcx,%rcx
   56f88:	je     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   56f8e:	lea    0x0(,%rsi,8),%r8
   56f96:	lea    -0x8(,%rsi,8),%r11
   56f9e:	xor    %r9d,%r9d
   56fa1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   56fb0:	mov    (%rdx,%r9,1),%r10
   56fb4:	cmp    %rax,%r10
   56fb7:	jne    56fc7 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x497>
   56fb9:	add    $0x8,%r9
   56fbd:	cmp    %r9,%r11
   56fc0:	jne    56fb0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x480>
   56fc2:	jmp    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   56fc7:	lea    -0x10(%r8),%rdx
   56fcb:	cmp    %r10,%rax
   56fce:	jae    57004 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4d4>
   56fd0:	cmp    %r9,%rdx
   56fd3:	je     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   56fd9:	add    $0xffffffffffffffe8,%r8
   56fdd:	nopl   (%rax)
   56fe0:	mov    %r10,%rax
   56fe3:	mov    0x10(%rdi,%r9,1),%r10
   56fe8:	cmp    %r10,%rax
   56feb:	ja     56ff6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4c6>
   56fed:	cmp    %r9,%r8
   56ff0:	lea    0x8(%r9),%r9
   56ff4:	jne    56fe0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4b0>
   56ff6:	cmp    %r10,%rax
   56ff9:	ja     574da <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9aa>
   56fff:	jmp    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57004:	cmp    %r9,%rdx
   57007:	je     5702f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4ff>
   57009:	lea    -0x18(%r8),%rax
   5700d:	nopl   (%rax)
   57010:	mov    %r10,%rdx
   57013:	mov    0x10(%rdi,%r9,1),%r10
   57018:	cmp    %r10,%rdx
   5701b:	jb     57026 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4f6>
   5701d:	cmp    %r9,%rax
   57020:	lea    0x8(%r9),%r9
   57024:	jne    57010 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x4e0>
   57026:	cmp    %r10,%rdx
   57029:	jb     574da <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9aa>
   5702f:	mov    %rsi,%rax
   57032:	shr    $1,%rax
   57035:	cmp    $0x8,%rsi
   57039:	jae    57099 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x569>
   5703b:	xor    %ecx,%ecx
   5703d:	jmp    57116 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x5e6>
   57042:	lea    -0x10(%rax),%r9
   57046:	cmp    %rdx,%r8
   57049:	jae    572a1 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x771>
   5704f:	cmp    %rcx,%r9
   57052:	je     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57058:	add    $0xffffffffffffffe8,%rax
   5705c:	nopl   0x0(%rax)
   57060:	mov    %rdx,%r8
   57063:	mov    0x10(%rdi,%rcx,1),%rdx
   57068:	cmp    %rdx,%r8
   5706b:	ja     57076 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x546>
   5706d:	cmp    %rcx,%rax
   57070:	lea    0x8(%rcx),%rcx
   57074:	jne    57060 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x530>
   57076:	cmp    %rdx,%r8
   57079:	ja     56c1e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xee>
   5707f:	jmp    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57084:	mov    (%rsp),%rsi
   57088:	cmp    %r14,%rsi
   5708b:	mov    %rbx,%rdi
   5708e:	ja     5730c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7dc>
   57094:	jmp    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57099:	movabs $0x7fffffffffffffc,%rcx
   570a3:	and    %rax,%rcx
   570a6:	mov    %rax,%rdx
   570a9:	and    $0xfffffffffffffffc,%rdx
   570ad:	neg    %rdx
   570b0:	lea    0x10(%rdi),%r9
   570b4:	lea    (%r8,%rdi,1),%r10
   570b8:	add    $0xfffffffffffffff0,%r10
   570bc:	xor    %r11d,%r11d
   570bf:	nop
   570c0:	movdqu -0x10(%r9),%xmm0
   570c6:	movdqu (%r9),%xmm1
   570cb:	movdqu -0x10(%r10,%r11,8),%xmm2
   570d2:	movdqu (%r10,%r11,8),%xmm3
   570d8:	pshufd $0x4e,%xmm3,%xmm3
   570dd:	pshufd $0x4e,%xmm2,%xmm2
   570e2:	movdqu %xmm3,-0x10(%r9)
   570e8:	movdqu %xmm2,(%r9)
   570ed:	pshufd $0x4e,%xmm0,%xmm0
   570f2:	pshufd $0x4e,%xmm1,%xmm1
   570f7:	movdqu %xmm0,(%r10,%r11,8)
   570fd:	movdqu %xmm1,-0x10(%r10,%r11,8)
   57104:	add    $0xfffffffffffffffc,%r11
   57108:	add    $0x20,%r9
   5710c:	cmp    %r11,%rdx
   5710f:	jne    570c0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x590>
   57111:	cmp    %rcx,%rax
   57114:	je     5714a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x61a>
   57116:	neg    %rax
   57119:	lea    (%rdi,%rcx,8),%rdx
   5711d:	neg    %rcx
   57120:	add    %rdi,%r8
   57123:	add    $0xfffffffffffffff8,%r8
   57127:	nopw   0x0(%rax,%rax,1)
   57130:	mov    (%rdx),%r9
   57133:	mov    (%r8,%rcx,8),%r10
   57137:	mov    %r10,(%rdx)
   5713a:	mov    %r9,(%r8,%rcx,8)
   5713e:	dec    %rcx
   57141:	add    $0x8,%rdx
   57145:	cmp    %rcx,%rax
   57148:	jne    57130 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x600>
   5714a:	movabs $0x7ffffffffffffffc,%rcx
   57154:	lea    0x10(%rdi),%r8
   57158:	lea    -0x10(%rdi),%r9
   5715c:	lea    -0x8(%rdi),%r10
   57160:	xor    %eax,%eax
   57162:	jmp    57179 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x649>
   57164:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57170:	cmp    %rsi,%rax
   57173:	jae    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57179:	mov    %rax,%rdx
   5717c:	lea    0x1(%rax),%r11
   57180:	cmp    %r11,%rsi
   57183:	mov    %r11,%rax
   57186:	cmova  %rsi,%rax
   5718a:	lea    -0x1(%rax),%rbx
   5718e:	cmp    %rbx,%rdx
   57191:	je     571b3 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x683>
   57193:	mov    (%rdi,%rdx,8),%rbx
   57197:	nopw   0x0(%rax,%rax,1)
   571a0:	cmp    %rbx,(%rdi,%r11,8)
   571a4:	jne    571b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x680>
   571a6:	inc    %r11
   571a9:	cmp    %r11,%rax
   571ac:	jne    571a0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x670>
   571ae:	jmp    571b3 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x683>
   571b0:	mov    %r11,%rax
   571b3:	mov    %rax,%rbx
   571b6:	sub    %rdx,%rbx
   571b9:	jb     5771f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbef>
   571bf:	mov    %rbx,%r11
   571c2:	shr    $1,%r11
   571c5:	je     57170 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x640>
   571c7:	cmp    $0x8,%rbx
   571cb:	jae    571e0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6b0>
   571cd:	xor    %ebx,%ebx
   571cf:	jmp    5725a <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x72a>
   571d4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   571e0:	mov    %r11,%rbx
   571e3:	and    %rcx,%rbx
   571e6:	mov    %r11,%r14
   571e9:	and    $0xfffffffffffffffc,%r14
   571ed:	neg    %r14
   571f0:	lea    (%r8,%rdx,8),%r15
   571f4:	lea    (%r9,%rax,8),%r12
   571f8:	xor    %r13d,%r13d
   571fb:	nopl   0x0(%rax,%rax,1)
   57200:	movdqu -0x10(%r15),%xmm0
   57206:	movdqu (%r15),%xmm1
   5720b:	movdqu -0x10(%r12,%r13,8),%xmm2
   57212:	movdqu (%r12,%r13,8),%xmm3
   57218:	pshufd $0x4e,%xmm3,%xmm3
   5721d:	pshufd $0x4e,%xmm2,%xmm2
   57222:	movdqu %xmm3,-0x10(%r15)
   57228:	movdqu %xmm2,(%r15)
   5722d:	pshufd $0x4e,%xmm0,%xmm0
   57232:	pshufd $0x4e,%xmm1,%xmm1
   57237:	movdqu %xmm0,(%r12,%r13,8)
   5723d:	movdqu %xmm1,-0x10(%r12,%r13,8)
   57244:	add    $0xfffffffffffffffc,%r13
   57248:	add    $0x20,%r15
   5724c:	cmp    %r13,%r14
   5724f:	jne    57200 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x6d0>
   57251:	cmp    %rbx,%r11
   57254:	je     57170 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x640>
   5725a:	lea    (%rdi,%rdx,8),%rdx
   5725e:	lea    0x0(,%rbx,8),%r15
   57266:	lea    (%r10,%rax,8),%r14
   5726a:	sub    %r15,%r14
   5726d:	nopl   (%rax)
   57270:	mov    (%rdx,%rbx,8),%r15
   57274:	mov    (%r14),%r12
   57277:	mov    %r12,(%rdx,%rbx,8)
   5727b:	mov    %r15,(%r14)
   5727e:	inc    %rbx
   57281:	add    $0xfffffffffffffff8,%r14
   57285:	cmp    %rbx,%r11
   57288:	jne    57270 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x740>
   5728a:	jmp    57170 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x640>
   5728f:	mov    %r15,%rdi
   57292:	call   *0x83518(%rip)        # da7b0 <free@GLIBC_2.2.5>
   57298:	mov    (%rsp),%rsi
   5729c:	mov    %rbx,%rdi
   5729f:	jmp    5730c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x7dc>
   572a1:	cmp    %rcx,%r9
   572a4:	je     572cf <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x79f>
   572a6:	lea    -0x18(%rax),%r8
   572aa:	nopw   0x0(%rax,%rax,1)
   572b0:	mov    %rdx,%r9
   572b3:	mov    0x10(%rdi,%rcx,1),%rdx
   572b8:	cmp    %rdx,%r9
   572bb:	jb     572c6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x796>
   572bd:	cmp    %rcx,%r8
   572c0:	lea    0x8(%rcx),%rcx
   572c4:	jne    572b0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x780>
   572c6:	cmp    %rdx,%r9
   572c9:	jb     56c1e <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xee>
   572cf:	mov    %rsi,%rcx
   572d2:	shr    $1,%rcx
   572d5:	cmp    $0x8,%rsi
   572d9:	jae    57535 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa05>
   572df:	xor    %edx,%edx
   572e1:	jmp    575b6 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa86>
   572e6:	mov    %r12d,%eax
   572e9:	xor    %edx,%edx
   572eb:	div    %ebp
   572ed:	mov    %rbx,%rdi
   572f0:	mov    (%rsp),%rsi
   572f4:	mov    %rax,%rdx
   572f7:	call   58190 <sort_research_rs::other::rust_tessera_radix::engine::budgeted_insertion_sort::<u64, u64, <u64 as sort_research_rs::other::radix_support::Key>::key>>
   572fc:	mov    %rbx,%rdi
   572ff:	mov    (%rsp),%rsi
   57303:	test   %rax,%rax
   57306:	jne    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   5730c:	cmp    $0x401,%rsi
   57313:	mov    $0x401,%r9d
   57319:	cmovb  %rsi,%r9
   5731d:	lea    0x0(,%r9,8),%r8d
   57325:	lea    (%rdi,%r8,1),%rax
   57329:	mov    (%rdi),%rdx
   5732c:	lea    0x8(%rdi),%rcx
   57330:	lea    -0x10(%r8),%r11
   57334:	cmp    $0x18,%r11
   57338:	jae    57342 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x812>
   5733a:	mov    %rdx,%r10
   5733d:	jmp    573d0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8a0>
   57342:	shr    $0x3,%r11
   57346:	inc    %r11
   57349:	mov    %r11,%rbx
   5734c:	and    $0xfffffffffffffffc,%rbx
   57350:	lea    (%rcx,%rbx,8),%rcx
   57354:	movq   %rdx,%xmm2
   57359:	shufpd $0x2,-0x4d582(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   57362:	movq   %rdx,%xmm1
   57367:	pxor   %xmm0,%xmm0
   5736b:	pcmpeqd %xmm3,%xmm3
   5736f:	xor    %edx,%edx
   57371:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   57380:	movdqu 0x8(%rdi,%rdx,8),%xmm4
   57386:	movdqu 0x18(%rdi,%rdx,8),%xmm5
   5738c:	por    %xmm4,%xmm1
   57390:	por    %xmm5,%xmm0
   57394:	andpd  %xmm4,%xmm2
   57398:	pand   %xmm5,%xmm3
   5739c:	add    $0x4,%rdx
   573a0:	cmp    %rdx,%rbx
   573a3:	jne    57380 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x850>
   573a5:	pand   %xmm2,%xmm3
   573a9:	pshufd $0xee,%xmm3,%xmm2
   573ae:	pand   %xmm3,%xmm2
   573b2:	movq   %xmm2,%rdx
   573b7:	por    %xmm1,%xmm0
   573bb:	pshufd $0xee,%xmm0,%xmm1
   573c0:	por    %xmm0,%xmm1
   573c4:	movq   %xmm1,%r10
   573c9:	cmp    %rbx,%r11
   573cc:	je     573e2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8b2>
   573ce:	xchg   %ax,%ax
   573d0:	mov    (%rcx),%r11
   573d3:	or     %r11,%r10
   573d6:	and    %r11,%rdx
   573d9:	add    $0x8,%rcx
   573dd:	cmp    %rax,%rcx
   573e0:	jne    573d0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8a0>
   573e2:	mov    %rdx,%rcx
   573e5:	xor    %r10,%rcx
   573e8:	cmp    $0xffffffffffffffff,%rcx
   573ec:	je     57420 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8f0>
   573ee:	lea    0x0(,%rsi,8),%r11
   573f6:	cmp    %r11,%r8
   573f9:	je     574d5 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9a5>
   573ff:	not    %r9
   57402:	add    %rsi,%r9
   57405:	movabs $0x1fffffffffffffff,%r11
   5740f:	and    %r9,%r11
   57412:	cmp    $0x3,%r11
   57416:	jae    5742c <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x8fc>
   57418:	mov    %r10,%rcx
   5741b:	jmp    574ad <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x97d>
   57420:	mov    $0xffffffffffffffff,%rcx
   57427:	jmp    574da <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9aa>
   5742c:	inc    %r11
   5742f:	mov    %r11,%r9
   57432:	and    $0xfffffffffffffffc,%r9
   57436:	lea    (%rax,%r9,8),%rax
   5743a:	movq   %rdx,%xmm2
   5743f:	shufpd $0x2,-0x4d668(%rip),%xmm2        # 9de0 <anon.3ef396a44d8e72e24100c5f0b77e19c4.2.llvm.5815919481666169068+0x172>
   57448:	movq   %r10,%xmm1
   5744d:	lea    (%r8,%rdi,1),%rcx
   57451:	add    $0x10,%rcx
   57455:	pxor   %xmm0,%xmm0
   57459:	pcmpeqd %xmm3,%xmm3
   5745d:	xor    %edx,%edx
   5745f:	nop
   57460:	movdqu -0x10(%rcx,%rdx,8),%xmm4
   57466:	movdqu (%rcx,%rdx,8),%xmm5
   5746b:	por    %xmm4,%xmm1
   5746f:	por    %xmm5,%xmm0
   57473:	andpd  %xmm4,%xmm2
   57477:	pand   %xmm5,%xmm3
   5747b:	add    $0x4,%rdx
   5747f:	cmp    %rdx,%r9
   57482:	jne    57460 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x930>
   57484:	pand   %xmm2,%xmm3
   57488:	pshufd $0xee,%xmm3,%xmm2
   5748d:	pand   %xmm3,%xmm2
   57491:	movq   %xmm2,%rdx
   57496:	por    %xmm1,%xmm0
   5749a:	pshufd $0xee,%xmm0,%xmm1
   5749f:	por    %xmm0,%xmm1
   574a3:	movq   %xmm1,%rcx
   574a8:	cmp    %r9,%r11
   574ab:	je     574d2 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9a2>
   574ad:	lea    (%rdi,%rsi,8),%r8
   574b1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   574c0:	mov    (%rax),%r9
   574c3:	add    $0x8,%rax
   574c7:	or     %r9,%rcx
   574ca:	and    %r9,%rdx
   574cd:	cmp    %r8,%rax
   574d0:	jne    574c0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x990>
   574d2:	xor    %rdx,%rcx
   574d5:	test   %rcx,%rcx
   574d8:	je     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   574da:	movq   $0x0,0x8(%rsp)
   574e3:	movq   $0x8,0x10(%rsp)
   574ec:	movq   $0x0,0x18(%rsp)
   574f5:	mov    %r13,0x20(%rsp)
   574fa:	lea    0x8(%rsp),%rax
   574ff:	mov    %rdi,%r8
   57502:	mov    %rax,%rdi
   57505:	mov    %rsi,%rdx
   57508:	mov    %r8,%rsi
   5750b:	xor    %r8d,%r8d
   5750e:	call   664f0 <<sort_research_rs::other::rust_tessera_radix::engine::Sorter<u64>>::sort_node>
   57513:	cmpq   $0x0,0x8(%rsp)
   57519:	je     57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   5751b:	mov    0x10(%rsp),%rdi
   57520:	call   *0x8328a(%rip)        # da7b0 <free@GLIBC_2.2.5>
   57526:	add    $0x28,%rsp
   5752a:	pop    %rbx
   5752b:	pop    %r12
   5752d:	pop    %r13
   5752f:	pop    %r14
   57531:	pop    %r15
   57533:	pop    %rbp
   57534:	ret
   57535:	movabs $0x7fffffffffffffc,%rdx
   5753f:	and    %rcx,%rdx
   57542:	mov    %rcx,%r8
   57545:	and    $0xfffffffffffffffc,%r8
   57549:	neg    %r8
   5754c:	lea    0x10(%rdi),%r9
   57550:	lea    (%rax,%rdi,1),%r10
   57554:	add    $0xfffffffffffffff0,%r10
   57558:	xor    %r11d,%r11d
   5755b:	nopl   0x0(%rax,%rax,1)
   57560:	movdqu -0x10(%r9),%xmm0
   57566:	movdqu (%r9),%xmm1
   5756b:	movdqu -0x10(%r10,%r11,8),%xmm2
   57572:	movdqu (%r10,%r11,8),%xmm3
   57578:	pshufd $0x4e,%xmm3,%xmm3
   5757d:	pshufd $0x4e,%xmm2,%xmm2
   57582:	movdqu %xmm3,-0x10(%r9)
   57588:	movdqu %xmm2,(%r9)
   5758d:	pshufd $0x4e,%xmm0,%xmm0
   57592:	pshufd $0x4e,%xmm1,%xmm1
   57597:	movdqu %xmm0,(%r10,%r11,8)
   5759d:	movdqu %xmm1,-0x10(%r10,%r11,8)
   575a4:	add    $0xfffffffffffffffc,%r11
   575a8:	add    $0x20,%r9
   575ac:	cmp    %r11,%r8
   575af:	jne    57560 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xa30>
   575b1:	cmp    %rdx,%rcx
   575b4:	je     575ea <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaba>
   575b6:	neg    %rcx
   575b9:	lea    (%rdi,%rdx,8),%r8
   575bd:	neg    %rdx
   575c0:	add    %rdi,%rax
   575c3:	add    $0xfffffffffffffff8,%rax
   575c7:	nopw   0x0(%rax,%rax,1)
   575d0:	mov    (%r8),%r9
   575d3:	mov    (%rax,%rdx,8),%r10
   575d7:	mov    %r10,(%r8)
   575da:	mov    %r9,(%rax,%rdx,8)
   575de:	dec    %rdx
   575e1:	add    $0x8,%r8
   575e5:	cmp    %rdx,%rcx
   575e8:	jne    575d0 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xaa0>
   575ea:	movabs $0x7ffffffffffffffc,%rcx
   575f4:	lea    0x10(%rdi),%r8
   575f8:	lea    -0x10(%rdi),%r9
   575fc:	lea    -0x8(%rdi),%r10
   57600:	xor    %eax,%eax
   57602:	jmp    57619 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xae9>
   57604:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   57610:	cmp    %rsi,%rax
   57613:	jae    57526 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0x9f6>
   57619:	mov    %rax,%rdx
   5761c:	lea    0x1(%rax),%r11
   57620:	cmp    %r11,%rsi
   57623:	mov    %r11,%rax
   57626:	cmova  %rsi,%rax
   5762a:	lea    -0x1(%rax),%rbx
   5762e:	cmp    %rbx,%rdx
   57631:	je     57653 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb23>
   57633:	mov    (%rdi,%rdx,8),%rbx
   57637:	nopw   0x0(%rax,%rax,1)
   57640:	cmp    %rbx,(%rdi,%r11,8)
   57644:	jne    57650 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb20>
   57646:	inc    %r11
   57649:	cmp    %r11,%rax
   5764c:	jne    57640 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb10>
   5764e:	jmp    57653 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb23>
   57650:	mov    %r11,%rax
   57653:	mov    %rax,%rbx
   57656:	sub    %rdx,%rbx
   57659:	jb     5771f <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbef>
   5765f:	mov    %rbx,%r11
   57662:	shr    $1,%r11
   57665:	je     57610 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xae0>
   57667:	cmp    $0x8,%rbx
   5766b:	jae    57671 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb41>
   5766d:	xor    %ebx,%ebx
   5766f:	jmp    576ea <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbba>
   57671:	mov    %r11,%rbx
   57674:	and    %rcx,%rbx
   57677:	mov    %r11,%r14
   5767a:	and    $0xfffffffffffffffc,%r14
   5767e:	neg    %r14
   57681:	lea    (%r8,%rdx,8),%r15
   57685:	lea    (%r9,%rax,8),%r12
   57689:	xor    %r13d,%r13d
   5768c:	nopl   0x0(%rax)
   57690:	movdqu -0x10(%r15),%xmm0
   57696:	movdqu (%r15),%xmm1
   5769b:	movdqu -0x10(%r12,%r13,8),%xmm2
   576a2:	movdqu (%r12,%r13,8),%xmm3
   576a8:	pshufd $0x4e,%xmm3,%xmm3
   576ad:	pshufd $0x4e,%xmm2,%xmm2
   576b2:	movdqu %xmm3,-0x10(%r15)
   576b8:	movdqu %xmm2,(%r15)
   576bd:	pshufd $0x4e,%xmm0,%xmm0
   576c2:	pshufd $0x4e,%xmm1,%xmm1
   576c7:	movdqu %xmm0,(%r12,%r13,8)
   576cd:	movdqu %xmm1,-0x10(%r12,%r13,8)
   576d4:	add    $0xfffffffffffffffc,%r13
   576d8:	add    $0x20,%r15
   576dc:	cmp    %r13,%r14
   576df:	jne    57690 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xb60>
   576e1:	cmp    %rbx,%r11
   576e4:	je     57610 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xae0>
   576ea:	lea    (%rdi,%rdx,8),%rdx
   576ee:	lea    0x0(,%rbx,8),%r15
   576f6:	lea    (%r10,%rax,8),%r14
   576fa:	sub    %r15,%r14
   576fd:	nopl   (%rax)
   57700:	mov    (%rdx,%rbx,8),%r15
   57704:	mov    (%r14),%r12
   57707:	mov    %r12,(%rdx,%rbx,8)
   5770b:	mov    %r15,(%r14)
   5770e:	inc    %rbx
   57711:	add    $0xfffffffffffffff8,%r14
   57715:	cmp    %rbx,%r11
   57718:	jne    57700 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xbd0>
   5771a:	jmp    57610 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xae0>
   5771f:	lea    0x80b02(%rip),%rcx        # d8228 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7f8>
   57726:	mov    %rdx,%rdi
   57729:	mov    %rsi,%rdx
   5772c:	mov    %rax,%rsi
   5772f:	call   *0x830f3(%rip)        # da828 <_DYNAMIC+0x280>
   57735:	lea    0x80aa4(%rip),%rcx        # d81e0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x7b0>
   5773c:	mov    %rax,%rdi
   5773f:	mov    %rsi,%rdx
   57742:	call   *0x830e0(%rip)        # da828 <_DYNAMIC+0x280>
   57748:	lea    -0x4ab8e(%rip),%rdi        # cbc1 <panic_unwind::imp::CANARY+0x1f2>
   5774f:	lea    0x80b32(%rip),%rdx        # d8288 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x858>
   57756:	mov    $0x13,%esi
   5775b:	call   *0x830f7(%rip)        # da858 <_DYNAMIC+0x2b0>
   57761:	jmp    57791 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc61>
   57763:	lea    0x80b36(%rip),%rdi        # d82a0 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x870>
   5776a:	call   *0x83220(%rip)        # da990 <_DYNAMIC+0x3e8>
   57770:	jmp    57791 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc61>
   57772:	mov    $0x8,%edi
   57777:	mov    %r12,%rsi
   5777a:	call   *0x83068(%rip)        # da7e8 <_DYNAMIC+0x240>
   57780:	lea    0x80b31(%rip),%rdx        # d82b8 <anon.93ac70e8ca7ae50c83cd5c36a588fb7c.2.llvm.12263198691361062877+0x888>
   57787:	xor    %edi,%edi
   57789:	xor    %esi,%esi
   5778b:	call   *0x83067(%rip)        # da7f8 <_DYNAMIC+0x250>
   57791:	ud2
   57793:	mov    %rax,%rbx
   57796:	cmpq   $0x0,0x8(%rsp)
   5779c:	je     577b1 <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc81>
   5779e:	mov    0x10(%rsp),%rdi
   577a3:	jmp    577ab <sort_research_rs::other::rust_tessera_radix::engine::sort_with_tile_bytes::<u64>+0xc7b>
   577a5:	mov    %rax,%rbx
   577a8:	mov    %r15,%rdi
   577ab:	call   *0x82fff(%rip)        # da7b0 <free@GLIBC_2.2.5>
   577b1:	mov    %rbx,%rdi
   577b4:	call   d5e40 <_Unwind_Resume@plt>
