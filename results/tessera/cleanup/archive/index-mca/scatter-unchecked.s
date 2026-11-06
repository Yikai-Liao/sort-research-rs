.text
.L3c500:
  mov    -0x18(%r12),%rax
.L3c505:
  mov    %rax,%rbx
.L3c508:
  and    %r14,%rbx
.L3c50b:
  mov    %r9d,%ecx
.L3c50e:
  shr    %cl,%rbx
.L3c511:
  mov    %rbx,%r15
.L3c514:
  shl    $0x4,%r15
.L3c518:
  mov    0x0(%r13,%r15,1),%rcx
.L3c51d:
  lea    0x8(%rcx),%rdx
.L3c521:
  cmp    0x8(%r13,%r15,1),%rdx
.L3c526:
  mov    %rax,(%rcx)
.L3c529:
  mov    %rdx,0x0(%r13,%r15,1)
.L3c52e:
  je .Lexit
.L3c534:
  mov    -0x10(%r12),%rax
.L3c539:
  mov    %rax,%r15
.L3c53c:
  and    %r14,%r15
.L3c53f:
  mov    %r9d,%ecx
.L3c542:
  shr    %cl,%r15
.L3c545:
  mov    %r15,%rbx
.L3c548:
  shl    $0x4,%rbx
.L3c54c:
  mov    0x0(%r13,%rbx,1),%rcx
.L3c551:
  lea    0x8(%rcx),%rdx
.L3c555:
  cmp    0x8(%r13,%rbx,1),%rdx
.L3c55a:
  mov    %rax,(%rcx)
.L3c55d:
  mov    %rdx,0x0(%r13,%rbx,1)
.L3c562:
  je .Lexit
.L3c568:
  mov    -0x8(%r12),%rax
.L3c56d:
  mov    %rax,%r15
.L3c570:
  and    %r14,%r15
.L3c573:
  mov    %r9d,%ecx
.L3c576:
  shr    %cl,%r15
.L3c579:
  mov    %r15,%rbx
.L3c57c:
  shl    $0x4,%rbx
.L3c580:
  mov    0x0(%r13,%rbx,1),%rcx
.L3c585:
  lea    0x8(%rcx),%rdx
.L3c589:
  cmp    0x8(%r13,%rbx,1),%rdx
.L3c58e:
  mov    %rax,(%rcx)
.L3c591:
  mov    %rdx,0x0(%r13,%rbx,1)
.L3c596:
  je .Lexit
.L3c59c:
  mov    (%r12),%rax
.L3c5a0:
  mov    %rax,%r15
.L3c5a3:
  and    %r14,%r15
.L3c5a6:
  mov    %r9d,%ecx
.L3c5a9:
  shr    %cl,%r15
.L3c5ac:
  mov    %r15,%rbx
.L3c5af:
  shl    $0x4,%rbx
.L3c5b3:
  mov    0x0(%r13,%rbx,1),%rcx
.L3c5b8:
  lea    0x8(%rcx),%rdx
.L3c5bc:
  cmp    0x8(%r13,%rbx,1),%rdx
.L3c5c1:
  mov    %rax,(%rcx)
.L3c5c4:
  mov    %rdx,0x0(%r13,%rbx,1)
.L3c5c9:
  je .Lexit
.L3c5cf:
  dec    %rbp
.L3c5d2:
  add    $0x20,%r12
.L3c5d6:
  test   %rbp,%rbp
.L3c5d9:
  jne .L3c500
.Lexit:
