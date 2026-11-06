.text
.L57290:
  mov    %rdx,%r8
.L57293:
  mov    (%rdi,%rcx,1),%rdx
.L57297:
  cmp    %rdx,%r8
.L5729a:
  jb .Lexit
.L572a0:
  add    $0x8,%rcx
.L572a4:
  cmp    %rcx,%rax
.L572a7:
  jne .L57290
.Lexit:
