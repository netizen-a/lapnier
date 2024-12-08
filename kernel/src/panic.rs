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
use crate::hcf;
use crate::io;
use arrayvec::ArrayString;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let _ = io::cls(0);
        let _ = io::kprint(b"kernel panicked!\n", 0x00ff0000, 0);
    }
    if let Some(location) = info.location() {
        let mut line_astr = ArrayString::<50>::new();
        if let Ok(_) = write!(line_astr, "line: {}\n", location.line()) {
            unsafe {
                let _ = io::kprint(line_astr.as_str(), 0x00ff0000, 0);
            }
        } else {
            unsafe {
                let _ = io::kprint(b"error: failed convert line\n", 0x00ff0000, 0);
            }
        }
        unsafe {
            let _ = io::kprint(b"file: ", 0x00ff0000, 0);
            let _ = io::kprint(location.file().as_bytes(), 0x00ff0000, 0);
            let _ = io::kprint(b"\n", 0x00ff0000, 0);
        }
    }
    if let Some(msg) = info.message().as_str() {
        unsafe {
            let _ = io::kprint(b"msg: ", 0x00ff0000, 0);
            let _ = io::kprint(msg.as_bytes(), 0x00ff0000, 0);
            let _ = io::kprint(b"\n", 0x00ff0000, 0);
        }
    }
    hcf();
}
