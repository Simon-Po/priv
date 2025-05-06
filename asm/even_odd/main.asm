section .data
  user_prompt: db "Enter a number:"
  user_prompt_len: equ $-user_prompt
  user_answer:  db 0 ; a buffer of 1 bytes
 


  answer_is_even: db " is even",10
  answer_is_even_len: equ $-answer_is_even

  answer_is_odd: db " is odd",10
  answer_is_odd_len: equ $-answer_is_odd
section .text
  global _start

_exit:
  mov rax,60 ; exit syscall
  mov rdi,0  ; exit with exitcode 0
  syscall



_start:
  
  ; Print the Prompt
  mov rax,1
  mov rdi,1
  mov rsi,user_prompt
  mov rdx,user_prompt_len
  syscall

  mov rax,0
  mov rdi,0
  mov rsi,user_answer
  mov rdx,1
  syscall

  mov r9,rax 
  mov rax,[user_answer]
  sub rax,0x30 
  
  and rax,1 
  cmp rax,1 
  je isOdd
  jmp isEven

print_user_number:
  mov rax,1
  mov rdi,1
  mov rsi,user_answer
  mov rdx,r9
  syscall
  ret


isEven: 
  push r9       ; Save r9 onto stack
  call print_user_number
  pop r9        ; Restore r9 from stack
  mov rax,1
  mov rdi,1
  mov rsi,answer_is_even
  mov rdx,answer_is_even_len
  syscall
  jmp _exit

isOdd:
  push r9       ; Save r9 onto stack
  call print_user_number
  pop r9        ; Restore r9 from stack
  mov rax,1
  mov rdi,1
  mov rsi,answer_is_odd
  mov rdx,answer_is_odd_len
  syscall

  jmp _exit




