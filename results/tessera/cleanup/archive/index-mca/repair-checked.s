.text
.L57230:
  cmp    %rbx,%rsi
.L57233:
  jae .Lexit
.L57239:
  mov    %rsi,%rdi
.L5723c:
  lea    0x1(%rsi),%r9
.L57240:
  cmp    %r9,%rbx
.L57243:
  mov    %r9,%rsi
.L57246:
  cmova  %rbx,%rsi
.L5724a:
  lea    -0x1(%rsi),%r10
.L5724e:
  cmp    %r10,%rdi
.L57251:
  je .L57273
.L57253:
  mov    (%r14,%rdi,8),%r10
.L57260:
  cmp    %r10,(%r14,%r9,8)
.L57264:
  jne .L57270
.L57270:
  mov    %r9,%rsi
.L57273:
  mov    %rsi,%r10
.L57276:
  sub    %rdi,%r10
.L57279:
  jb .Lexit
.L5727f:
  mov    %r10,%r9
.L57282:
  shr    $1,%r9
.L57285:
  je .L57230
.Lexit:
