section .data
    msg db "Hello, World!", 0xA
    len equ $ - msg

section .text
    global _start

_start: 
    
    mov rax,5
    call main_loop
    mov rax,60
    mov rdi,0
    syscall

print_Hello:
    mov rax,1
    mov rdi,1
    mov rsi,msg
    mov rdx,len
    syscall
    ret

main_loop:
    push rax
    call print_Hello
    pop rax
    dec rax
    cmp rax,0
    jne main_loop
    ret
