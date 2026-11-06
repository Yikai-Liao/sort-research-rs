.text
.L56d40:
  mov    %rdi,%rsi
.L56d43:
  mov    0x10(%r14,%rdx,1),%rdi
.L56d48:
  cmp    %rdi,%rsi
.L56d4b:
  jb .Lexit
.L56d4d:
  cmp    %rdx,%rax
.L56d50:
  lea    0x8(%rdx),%rdx
.L56d54:
  jne .L56d40
.Lexit:
