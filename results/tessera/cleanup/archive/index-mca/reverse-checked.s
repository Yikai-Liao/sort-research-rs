.text
.L57180:
  movdqu -0x10(%rdi),%xmm0
.L57185:
  movdqu (%rdi),%xmm1
.L57189:
  movdqu -0x10(%r8,%r9,8),%xmm2
.L57190:
  movdqu (%r8,%r9,8),%xmm3
.L57196:
  pshufd $0x4e,%xmm3,%xmm3
.L5719b:
  pshufd $0x4e,%xmm2,%xmm2
.L571a0:
  movdqu %xmm3,-0x10(%rdi)
.L571a5:
  movdqu %xmm2,(%rdi)
.L571a9:
  pshufd $0x4e,%xmm0,%xmm0
.L571ae:
  pshufd $0x4e,%xmm1,%xmm1
.L571b3:
  movdqu %xmm0,(%r8,%r9,8)
.L571b9:
  movdqu %xmm1,-0x10(%r8,%r9,8)
.L571c0:
  add    $0xfffffffffffffffc,%r9
.L571c4:
  add    $0x20,%rdi
.L571c8:
  cmp    %r9,%rsi
.L571cb:
  jne .L57180
.Lexit:
