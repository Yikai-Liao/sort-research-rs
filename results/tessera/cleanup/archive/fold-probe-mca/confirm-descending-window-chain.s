.text
.L572b0:
  mov    %rdx,%r9
.L572b3:
  mov    0x10(%rdi,%rcx,1),%rdx
.L572b8:
  cmp    %rdx,%r9
.L572bb:
  jb .Lexit
.L572bd:
  cmp    %rcx,%r8
.L572c0:
  lea    0x8(%rcx),%rcx
.L572c4:
  jne .L572b0
.Lexit:
