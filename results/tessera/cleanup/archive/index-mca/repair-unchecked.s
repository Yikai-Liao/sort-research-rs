.text
.L573e0:
  cmp    %rbx,%rsi
.L573e3:
  jae .Lexit
.L573e9:
  mov    %rsi,%rdi
.L573ec:
  lea    0x1(%rsi),%r9
.L573f0:
  cmp    %r9,%rbx
.L573f3:
  mov    %r9,%rsi
.L573f6:
  cmova  %rbx,%rsi
.L573fa:
  lea    -0x1(%rsi),%r10
.L573fe:
  cmp    %r10,%rdi
.L57401:
  je .L57423
.L57403:
  mov    (%r14,%rdi,8),%r10
.L57410:
  cmp    %r10,(%r14,%r9,8)
.L57414:
  jne .L57420
.L57420:
  mov    %r9,%rsi
.L57423:
  mov    %rsi,%r10
.L57426:
  sub    %rdi,%r10
.L57429:
  jb .Lexit
.L5742f:
  mov    %r10,%r9
.L57432:
  shr    $1,%r9
.L57435:
  je .L573e0
.Lexit:
