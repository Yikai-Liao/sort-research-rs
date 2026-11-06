.text
.L56a10:
  add    %rcx,%rdx
.L56a13:
  inc    %rsi
.L56a16:
  je .Lexit
.L56a1c:
  mov    %edi,%r8d
.L56a1f:
  mov    %eax,%r9d
.L56a22:
  mov    -0x8(%r14,%rdx,8),%rax
.L56a27:
  cmp    (%r14,%rdx,8),%rax
.L56a2b:
  setbe  %al
.L56a2e:
  setae  %dil
.L56a32:
  and    %r9b,%al
.L56a35:
  and    %r8b,%dil
.L56a38:
  test   %al,%al
.L56a3a:
  jne .L56a10
.L56a3c:
  test   %dil,%dil
.L56a3f:
  jne .L56a10
.Lexit:
