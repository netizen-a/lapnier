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
use core::cell;

pub static GDT: GlobalDescriptorTable<3> = GlobalDescriptorTable::new([
    // Null segment
    add_entry(0, 0x00000, 0x00, 0x00),
    // Kernel code segment
    add_entry(0, 0xFFFFF, 0xA0, 0x9B),
    // Kernel data segment
    add_entry(0, 0xFFFFF, 0xA0, 0x93),
]);

#[derive(Debug)]
#[repr(transparent)]
pub struct GlobalDescriptorTable<const N: usize> {
    inner: cell::UnsafeCell<[u64; N]>,
}

impl<const N: usize> GlobalDescriptorTable<N> {
    #[inline]
    const fn new(value: [u64; N]) -> Self {
        Self {
            inner: cell::UnsafeCell::new(value),
        }
    }
    #[inline]
    pub const unsafe fn len(&self) -> usize {
        (*self.inner.get()).len()
    }
    #[inline]
    pub const unsafe fn as_ptr(&self) -> *const u64 {
        &raw const (*self.inner.get())[0]
    }
}

// not actually Sync, so any access is marked unsafe
unsafe impl<const N: usize> Sync for GlobalDescriptorTable<N> {}

const fn add_entry(base: u32, limit: u32, access: u8, flag: u8) -> u64 {
    let mut descriptor: u64;
    let base: u64 = base as u64;
    let limit = limit as u64;
    let flag = flag as u64;
    let access = access as u64;

    descriptor = limit & 0x000F0000;
    descriptor |= (flag << 8) & 0x0000FF00;
    descriptor |= (access << 16) & 0x00F00000;
    descriptor |= (base >> 16) & 0x000000FF;
    descriptor |= base & 0xFF000000;

    descriptor <<= 32;

    descriptor |= base << 16;
    descriptor |= limit & 0x0000FFFF;

    descriptor
}
