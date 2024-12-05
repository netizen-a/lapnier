/*
 * Copyright (C) 2024  Jonathan Thomason
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
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
    // Reload CS register:
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
