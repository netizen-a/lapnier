.intel_syntax noprefix

.globl reload_segments
.globl set_gdt

.data
gdt:
    gdt_null: .quad 0
    gdt_code: .quad 0x00af9b000000ffff
    gdt_data: .quad 0x00af93000000ffff
gdt_end:

gdtr:
    len:  .word gdt_end-gdt-1
    base: .quad gdt

.text
set_gdt:
    lgdt  [gdtr]
    ret

reload_segments:
    push 0x08
    lea rax, [rip+reload_CS]
    push rax
    retfq
reload_CS:
    // Reload data segment registers
    MOV   AX, 0x10
    MOV   DS, AX
    MOV   ES, AX
    MOV   FS, AX
    MOV   GS, AX
    MOV   SS, AX
    RET
