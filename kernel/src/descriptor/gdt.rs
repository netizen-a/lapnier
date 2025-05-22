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
use super::segment::{SegmentDescriptor32, SegmentDescriptor64};
use core::cell::SyncUnsafeCell;

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    pub null: SegmentDescriptor32,
    pub kern_code: SegmentDescriptor32,
    pub kern_data: SegmentDescriptor32,
    pub user_code: SegmentDescriptor32,
    pub user_data: SegmentDescriptor32,
    pub tss: SegmentDescriptor64,
}

pub static GDT: SyncUnsafeCell<GlobalDescriptorTable> = SyncUnsafeCell::new(
    GlobalDescriptorTable{
    // Null Descriptor
    null: SegmentDescriptor32::null(),
    // Kernel Mode Code Segment
    kern_code: SegmentDescriptor32::new(0, 0xFFFFF, 0x9A, 0xA).unwrap(),
    // Kernel Mode Data Segment
    kern_data: SegmentDescriptor32::new(0, 0xFFFFF, 0x92, 0xC).unwrap(),
    // User Mode Code Segment
    user_code: SegmentDescriptor32::new(0, 0xFFFFF, 0xFA, 0xA).unwrap(),
    // User Mode Data Segment
    user_data: SegmentDescriptor32::new(0, 0xFFFFF, 0xF2, 0xC).unwrap(),
    // Task State Segment
    tss: SegmentDescriptor64::null(),
});
