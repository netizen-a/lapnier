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
use core::arch::asm;

#[repr(C, packed)]
pub struct Gdtr {
    pub len: u16,
    pub base: *const u64,
}

pub unsafe fn reload_segments() {
    asm! {
        // Reload CS register:
        "push 0x08",
        "lea rax, [rip+2f]",
        "push rax",
        "retfq",
        // Reload data segment registers
        "2: mov   ax, 0x10",
        "mov   ds, ax",
        "mov   es, ax",
        "mov   fs, ax",
        "mov   gs, ax",
        "mov   ss, ax",
    }
}

#[inline(always)]
pub unsafe fn _load_gdt(gdtr: &Gdtr) {
    asm!("lgdt [{}]", in(reg) gdtr, options(nostack))
}
