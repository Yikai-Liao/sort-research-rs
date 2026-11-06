.text
.L56b50:
  lea    -0x1(%rdi),%rax
.L56b54:
  cmp    %rbx,%rax
.L56b57:
  jae .Lexit
.L56b5d:
  cmp    %rbx,%rdi
.L56b60:
  jae .Lexit
.L56b66:
  mov    (%r14,%rdi,8),%rax
.L56b6a:
  cmp    %rax,-0x8(%r14,%rdi,8)
.L56b6f:
  setbe  %al
.L56b72:
  setae  %r9b
.L56b76:
  and    %al,%dl
.L56b78:
  and    %r9b,%r8b
.L56b7b:
  mov    %edx,%eax
.L56b7d:
  or     %r8b,%al
.L56b80:
  je .Lexit
.L56b86:
  add    %rcx,%rdi
.L56b89:
  inc    %rsi
.L56b8c:
  jne .L56b50
.Lexit:
