section .data



section .text
  global _start

_exit:
  mov rax,60 ; exit syscall
  mov rdi,0  ; exit with exitcode 0
  syscall



_start:
  jmp _exit
