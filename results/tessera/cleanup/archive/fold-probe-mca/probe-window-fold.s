.text
.L56ba0:
  dec    %rdx
.L56ba3:
  sub    %rax,%rdx
.L56ba6:
  jb .Lexit
.L56bac:
  cmp    $0x1,%rdx
.L56bb0:
  jbe .Lexit
.L56bb6:
  mov    -0x8(%r10),%r11
.L56bba:
  cmp    (%r10),%r11
.L56bbd:
  lea    (%r10,%rcx,1),%r11
.L56bc1:
  setbe  %r10b
.L56bc5:
  setae  %bl
.L56bc8:
  and    %r10b,%r9b
.L56bcb:
  and    %bl,%r8b
.L56bce:
  mov    %r11,%r10
.L56bd1:
  test   %r9b,%r9b
.L56bd4:
  jne .L56ba0
.L56bd6:
  mov    %r11,%r10
.L56bd9:
  test   %r8b,%r8b
.L56bdc:
  jne .L56ba0
.Lexit:
