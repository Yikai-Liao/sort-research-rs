.text
.L56f15:
  mov    %rdx,%rdi
.L56f18:
  mov    (%rbx,%rcx,1),%rdx
.L56f1c:
  cmp    %rdx,%rdi
.L56f1f:
  jb .Lexit
.L56f25:
  add    $0x8,%rcx
.L56f29:
  cmp    %rcx,%rax
.L56f2c:
  jne .L56f15
.Lexit:
