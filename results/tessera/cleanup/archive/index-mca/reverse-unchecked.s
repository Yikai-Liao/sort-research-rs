.text
.L57333:
  movdqu -0x10(%rsi),%xmm0
.L57338:
  movdqu (%rsi),%xmm1
.L5733c:
  movdqu -0x10(%rdi,%r8,8),%xmm2
.L57343:
  movdqu (%rdi,%r8,8),%xmm3
.L57349:
  pshufd $0x4e,%xmm3,%xmm3
.L5734e:
  pshufd $0x4e,%xmm2,%xmm2
.L57353:
  movdqu %xmm3,-0x10(%rsi)
.L57358:
  movdqu %xmm2,(%rsi)
.L5735c:
  pshufd $0x4e,%xmm0,%xmm0
.L57361:
  pshufd $0x4e,%xmm1,%xmm1
.L57366:
  movdqu %xmm0,(%rdi,%r8,8)
.L5736c:
  movdqu %xmm1,-0x10(%rdi,%r8,8)
.L57373:
  add    $0xfffffffffffffffc,%r8
.L57377:
  add    $0x20,%rsi
.L5737b:
  cmp    %r8,%rdx
.L5737e:
  jne .L57333
.Lexit:
