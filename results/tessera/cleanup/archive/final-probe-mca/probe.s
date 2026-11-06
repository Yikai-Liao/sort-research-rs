.text
.L56910:
  add    %rcx,%rdx
.L56913:
  inc    %rdi
.L56916:
  je .Lexit
.L5691c:
  mov    %r8d,%r9d
.L5691f:
  mov    %eax,%r10d
.L56922:
  mov    -0x8(%rbx,%rdx,8),%rax
.L56927:
  cmp    (%rbx,%rdx,8),%rax
.L5692b:
  setbe  %al
.L5692e:
  setae  %r8b
.L56932:
  and    %r10b,%al
.L56935:
  and    %r9b,%r8b
.L56938:
  test   %al,%al
.L5693a:
  jne .L56910
.L5693c:
  test   %r8b,%r8b
.L5693f:
  jne .L56910
.Lexit:
