.text
.L89550:
  mov    %rax,%rsi
.L89553:
  mov    (%rcx),%rax
.L89556:
  add    $0x8,%rcx
.L8955a:
  cmp    %rax,%rsi
.L8955d:
  jb .Lexit
.L8955f:
  cmp    %rdx,%rcx
.L89562:
  jne .L89550
.Lexit:
