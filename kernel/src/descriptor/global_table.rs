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
use super::segment::SegmentDescriptor;

pub static GDT: GlobalDescriptorTable<3> = GlobalDescriptorTable::new([
    // Null segment
    SegmentDescriptor::new(0, 0x00000, 0x00, 0x0).unwrap(),
    // Kernel code segment
    SegmentDescriptor::new(0, 0xFFFFF, 0x9B, 0xA).unwrap(),
    // Kernel data segment
    SegmentDescriptor::new(0, 0xFFFFF, 0x93, 0xA).unwrap(),
]);

#[derive(Debug)]
#[repr(transparent)]
pub struct GlobalDescriptorTable<const N: usize> {
    inner: cell::UnsafeCell<[SegmentDescriptor; N]>,
}

impl<const N: usize> GlobalDescriptorTable<N> {
    #[inline]
    const fn new(value: [SegmentDescriptor; N]) -> Self {
        Self {
            inner: cell::UnsafeCell::new(value),
        }
    }
    #[inline]
    pub const unsafe fn len(&self) -> usize {
        (*self.inner.get()).len()
    }
    #[inline]
    pub const unsafe fn as_ptr(&self) -> *const SegmentDescriptor {
        &raw const (*self.inner.get())[0]
    }
}

// not actually Sync, so any access is marked unsafe
unsafe impl<const N: usize> Sync for GlobalDescriptorTable<N> {}
