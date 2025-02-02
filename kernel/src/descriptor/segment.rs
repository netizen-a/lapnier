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

#[repr(transparent)]
pub struct SegmentDescriptor(u64);

impl SegmentDescriptor {
    pub const fn new(base: u32, limit: u32, access: u8, flag: u8) -> Option<SegmentDescriptor> {
        if limit >= 2u32.pow(20) {
            return None;
        }
        if flag >= 2u8.pow(4) {
            return None;
        }
        let mut descriptor: u64;
        let base: u64 = base as u64;
        let limit = limit as u64;
        let flag = flag as u64;
        let access = access as u64;

        descriptor = limit & 0x000F0000;
        descriptor |= (access << 8) & 0x0000FF00;
        descriptor |= (flag << 20) & 0x00F00000;
        descriptor |= (base >> 16) & 0x000000FF;
        descriptor |= base & 0xFF000000;

        descriptor <<= 32;

        descriptor |= base << 16;
        descriptor |= limit & 0x0000FFFF;

        Some(SegmentDescriptor(descriptor))
    }
}
