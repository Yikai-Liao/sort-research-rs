.text
.L3f0d0:
  mov    -0x18(%rbx),%rax
.L3f0d4:
  mov    %rax,%rbp
.L3f0d7:
  and    %r14,%rbp
.L3f0da:
  mov    0xf0(%rsp),%rcx
.L3f0e2:
  shr    %cl,%rbp
.L3f0e5:
  mov    %rbp,%r12
.L3f0e8:
  shl    $0x4,%r12
.L3f0ec:
  mov    0x0(%r13,%r12,1),%rcx
.L3f0f1:
  lea    0x8(%rcx),%rdx
.L3f0f5:
  cmp    0x8(%r13,%r12,1),%rdx
.L3f0fa:
  mov    %rax,(%rcx)
.L3f0fd:
  mov    %rdx,0x0(%r13,%r12,1)
.L3f102:
  je .Lexit
.L3f108:
  mov    -0x10(%rbx),%rax
.L3f10c:
  mov    %rax,%rbp
.L3f10f:
  and    %r14,%rbp
.L3f112:
  mov    0xf0(%rsp),%rcx
.L3f11a:
  shr    %cl,%rbp
.L3f11d:
  mov    %rbp,%r12
.L3f120:
  shl    $0x4,%r12
.L3f124:
  mov    0x0(%r13,%r12,1),%rcx
.L3f129:
  lea    0x8(%rcx),%rdx
.L3f12d:
  cmp    0x8(%r13,%r12,1),%rdx
.L3f132:
  mov    %rax,(%rcx)
.L3f135:
  mov    %rdx,0x0(%r13,%r12,1)
.L3f13a:
  je .Lexit
.L3f140:
  mov    -0x8(%rbx),%rax
.L3f144:
  mov    %rax,%rbp
.L3f147:
  and    %r14,%rbp
.L3f14a:
  mov    0xf0(%rsp),%rcx
.L3f152:
  shr    %cl,%rbp
.L3f155:
  mov    %rbp,%r12
.L3f158:
  shl    $0x4,%r12
.L3f15c:
  mov    0x0(%r13,%r12,1),%rcx
.L3f161:
  lea    0x8(%rcx),%rdx
.L3f165:
  cmp    0x8(%r13,%r12,1),%rdx
.L3f16a:
  mov    %rax,(%rcx)
.L3f16d:
  mov    %rdx,0x0(%r13,%r12,1)
.L3f172:
  je .Lexit
.L3f178:
  mov    (%rbx),%rax
.L3f17b:
  mov    %rax,%rbp
.L3f17e:
  and    %r14,%rbp
.L3f181:
  mov    0xf0(%rsp),%rcx
.L3f189:
  shr    %cl,%rbp
.L3f18c:
  mov    %rbp,%r12
.L3f18f:
  shl    $0x4,%r12
.L3f193:
  mov    0x0(%r13,%r12,1),%rcx
.L3f198:
  lea    0x8(%rcx),%rdx
.L3f19c:
  cmp    0x8(%r13,%r12,1),%rdx
.L3f1a1:
  mov    %rax,(%rcx)
.L3f1a4:
  mov    %rdx,0x0(%r13,%r12,1)
.L3f1a9:
  je .Lexit
.L3f1af:
  dec    %r15
.L3f1b2:
  add    $0x20,%rbx
.L3f1b6:
  test   %r15,%r15
.L3f1b9:
  jne .L3f0d0
.Lexit:
