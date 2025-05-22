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
#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod arch;
mod descriptor;
mod fonts;
mod io;
mod panic;

use core::u32;
use arrayvec::ArrayString;
use descriptor::{gdt as gdt, tss};

use descriptor::segment::SegmentDescriptor64;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;
use core::fmt::Write;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
/// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());

    let tss_base = (&raw const tss::TSS) as usize;
    let tss_limit = (core::mem::size_of_val(&tss::TSS) - 1) as u32;
    (*gdt::GDT.get()).tss = SegmentDescriptor64::new(tss_base as u64, tss_limit, 0x89, 0x0).unwrap();

    let gdtr = arch::x86_64::Gdtr {
        len: (core::mem::size_of_val(&gdt::GDT) - 1) as u16,
        base: gdt::GDT.get() as *const u64,
    };
    arch::x86_64::_load_gdt(&gdtr);
    arch::x86_64::flush_tss();
    arch::x86_64::reload_segments();
    io::cls(0).unwrap();
    // io::kprint("Hello, World!", u32::MAX, 0).unwrap();
    let mut line_astr = ArrayString::<50>::new();
    write!(&mut line_astr, "0x{:x}", tss_base).unwrap();
    io::kprint(line_astr.as_bytes(), u32::MAX, 0).unwrap();

    arch::hcf();
}