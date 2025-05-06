section .data
  buffer: times 10 db 0   ; allocate a buffer of length 200 and initiallize it all to 0
  buffer_len: equ $-buffer ; defining the length of the buffer

  error_msg: db "Message too long",10
  error_msg_len: equ $-error_msg

section .text
  global _start

_exit:
  mov rax,60               ; exit syscall
  mov rdi,0                ; exit with exitcode 0
  syscall

_exit_error:            ; if we have an error we want to print the message to the stdout
                        ; and exit with a non 0 exit code
  mov rax,1             ; write syscall
  mov rdi,1             ; to stdout
  mov rsi,error_msg
  mov rdx,error_msg_len
  syscall

  mov rax,60            ; exit syscall
  mov rdi,1             ; move non 0 exit code into rdi
  syscall


_start:
  mov rax,0                ; 0 is the read syscall
  mov rdi,0                ; read from stdin
  mov rsi,buffer           ; pointer to our buffer in the data section
  mov rdx,buffer_len       ; the length of the buffer
  syscall                  ; Read into the buffer this will be stored in rax
  
  cmp rax,buffer_len       ; Compare the length of the message we received in the rax register with the size of our buffer
  jz _exit_error           ; if the buffer is not big enough we will exit with an error 

  mov rdx,rax              ; The read syscall will return us the length of
                           ; bytes read in rax wo save this into rdx to be used by the write syscall
  mov rax,1                ; we want to do a write syscall
  mov rdi,1                ; to standard out
  mov rsi,buffer           ; read from the buffer
  syscall

  jmp _exit
