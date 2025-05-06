section .data
  buffer: times 200 db 0   ; allocate a buffer of length 200 and initiallize it all to 0
  buffer_len: equ $-buffer ; defining the length of the buffer

section .text
  global _start

_exit:
  mov rax,60               ; exit syscall
  mov rdi,0                ; exit with exitcode 0
  syscall



_start:
  mov rax,0                ; 0 is the read syscall
  mov rdi,0                ; read from stdin
  mov rsi,buffer           ; pointer to our buffer in the data section
  mov rdx,buffer_len       ; the length of the buffer
  syscall                  ; Read into the buffer this will be stored in rax


  mov rdx,rax              ; The read syscall will return us the length of
                           ; bytes read in rax wo save this into rdx to be used by the write syscall
  mov rax,1                ; we want to do a write syscall
  mov rdi,1                ; to standard out
  mov rsi,buffer           ; read from the buffer
  syscall

  jmp _exit
