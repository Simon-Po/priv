section .data
  hi: db "Hello, World! and simon",10
  hi_len: equ $-hi ; Calculate the length of hi

section .text
  global _start ; export _start label

_exit:
  mov rax,60 ; exit syscall
  mov rdi,0  ; exit with exitcode 0
  syscall



_start:
  mov rax,1 ; write syscall
  mov rdi,1 ; first argument is where to write -> in this case stdout
  mov rsi,hi; pointer to the message in the data section
  mov rdx,hi_len; length of the message in bytes
  syscall 
  

  jmp _exit

