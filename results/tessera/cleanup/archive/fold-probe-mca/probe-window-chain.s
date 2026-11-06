.text
.L56be0:
  dec    %rdx
.L56be3:
  sub    %rax,%rdx
.L56be6:
  jb .Lexit
.L56bec:
  cmp    $0x1,%rdx
.L56bf0:
  jbe .Lexit
.L56bf6:
  mov    -0x8(%r10),%r11
.L56bfa:
  cmp    (%r10),%r11
.L56bfd:
  lea    (%r10,%rcx,1),%r11
.L56c01:
  setbe  %r10b
.L56c05:
  setae  %bl
.L56c08:
  and    %r10b,%r9b
.L56c0b:
  and    %bl,%r8b
.L56c0e:
  mov    %r11,%r10
.L56c11:
  test   %r9b,%r9b
.L56c14:
  jne .L56be0
.L56c16:
  mov    %r11,%r10
.L56c19:
  test   %r8b,%r8b
.L56c1c:
  jne .L56be0
.Lexit:
