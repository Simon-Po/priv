section .data
  buffer: times 1 db 0        ; Single byte buffer for holding ASCII character of number
  new_line: db 10              ; ASCII newline character ('\n')
  zero_char: db 0x30           ; ASCII character for '0'
  ten_char: db "10",10         ; String "10" followed by newline character

section .text
  global _start                ; Define the program entry point for linker

; ----------------------------
; Exit procedure
; ----------------------------
_exit:
  ; Print '0' character
  mov rax, 1                   ; Syscall number for write (1 = sys_write)
  mov rdi, 1                   ; File descriptor 1 = stdout
  mov rsi, zero_char            ; Address of '0' character in memory
  mov rdx, 1                   ; Write 1 byte
  syscall                      ; Make syscall to write '0' to terminal

  ; Exit the program cleanly
  mov rax, 60                  ; Syscall number for exit (60 = sys_exit)
  mov rdi, 0                   ; Exit code 0 (success)
  syscall                      ; Make syscall to exit

; ----------------------------
; Start procedure
; ----------------------------
_start:
  ; Print "10" and a newline
  mov rax, 1                   ; Syscall number for write
  mov rdi, 1                   ; File descriptor 1 = stdout
  mov rsi, ten_char             ; Address of "10\n" string
  mov rdx, 3                   ; Write 3 bytes ("1", "0", "\n")
  syscall                      ; Make syscall to write "10" to terminal

  ; Initialize counter to 9
  mov r9, 9                    ; Set counter register to 9
  jmp _loop                    ; Jump to loop start

; ----------------------------
; Loop procedure
; ----------------------------
_loop:
  ; Check if counter is zero
  cmp r9, 0                    ; Compare counter (r9) to 0
  je _exit                     ; If counter is zero, jump to _exit

  ; Convert number to ASCII character
  mov rax, r9                  ; Move counter value into rax
  add rax, 0x30                ; Convert number to corresponding ASCII character ('0' + number)
  mov [buffer], al             ; Store the character into buffer

  ; Decrement counter after preparing value
  dec r9                       ; Decrease counter by 1

  ; Write the number to terminal
  mov rax, 1                   ; Syscall number for write
  mov rdi, 1                   ; File descriptor 1 = stdout
  mov rsi, buffer              ; Address of buffer containing the ASCII character
  mov rdx, 1                   ; Write 1 byte
  syscall                      ; Make syscall to write number

  ; Write a newline after the number
  mov rax, 1                   ; Syscall number for write
  mov rdi, 1                   ; File descriptor 1 = stdout
  mov rsi, new_line            ; Address of newline character
  mov rdx, 1                   ; Write 1 byte
  syscall                      ; Make syscall to write newline

  ; Repeat the loop
  jmp _loop                    ; Jump back to the start of the loop


