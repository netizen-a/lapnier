use limine::framebuffer::Framebuffer;

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
use crate::fonts::font_6x8::FONT_6X8;
use crate::FRAMEBUFFER_REQUEST;
use core::sync::atomic::*;

// cursor encodes row * pitch + col
static CURSOR: AtomicU64 = AtomicU64::new(0);
const SCALER: u64 = 1;

#[derive(Debug)]
pub enum Error {
    NoResponse,
    NoBuffers,
}

pub struct CharacterProperties {
    pub x: u64,
    pub y: u64,
    pub scale: u64,
    pub foreground: u32,
    pub background: u32,
}

pub unsafe fn cls() -> Result<(), Error> {
    CURSOR.store(0, Ordering::Relaxed);
    let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() else {
        return Err(Error::NoResponse);
    };
    let Some(framebuffer) = framebuffer_response.framebuffers().next() else {
        return Err(Error::NoBuffers);
    };
    let height = framebuffer.height() / 8;
    let pitch = framebuffer.pitch();

    let limit = pitch * height;
    for next in 0..limit {
        let addr = framebuffer.addr() as *mut u64;
        addr.add(next as usize).write(0);
    }
    CURSOR.store(0, Ordering::Relaxed);
    Ok(())
}

pub unsafe fn kprint<S>(ascii: S, foreground: u32, background: u32) -> Result<(), Error>
where
    S: AsRef<[u8]>,
{
    let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() else {
        return Err(Error::NoResponse);
    };
    let Some(framebuffer) = framebuffer_response.framebuffers().next() else {
        return Err(Error::NoBuffers);
    };
    let mut properties = CharacterProperties {
        x: 0,
        y: 0,
        scale: SCALER,
        foreground,
        background,
    };
    let scale_offset = if SCALER == 0 { 1 } else { SCALER + 1 };

    let mut cursor = CURSOR.load(Ordering::Relaxed);
    let mut col = cursor % framebuffer.pitch();
    let mut row = cursor / framebuffer.pitch();
    let ascii_str = ascii.as_ref();
    let mut ascii_iter = ascii_str.iter();
    while let Some(c) = ascii_iter.next() {
        match c {
            // Carriage Return
            b'\r' => col = 0,
            // Line Feed
            b'\n' => {
                row += 1;
            }
            // Tab
            b'\t' => col += 8,
            // printable characters
            0x20..=0x7F => {
                properties.y = scale_offset * row * 8;
                properties.x = scale_offset * col * 6;
                kprint_char(*c, &properties, &framebuffer)?;
                col += 1;
            }
            // unused control characters
            _ => {}
        }
    }
    cursor = row * framebuffer.pitch() + col;
    CURSOR.store(cursor, Ordering::Relaxed);
    Ok(())
}

unsafe fn kprint_char(
    character: u8,
    properties: &CharacterProperties,
    framebuffer: &Framebuffer,
) -> Result<(), Error> {
    let font = FONT_6X8;
    let bitmap_index: usize = match character {
        0x20..0x7F => character as usize - 0x20,
        _ => 0,
    };

    let x = properties.x;
    let y = properties.y;
    let scale = properties.scale;

    let mut y_offset = 0;
    let mut x_offset = 0;
    for row in 0..8 {
        let data = font[bitmap_index][row as usize];
        for col in 2..8 {
            let is_foreground = data & (1 << (7 - col)) != 0;
            let local_prop = CharacterProperties {
                x: x_offset + col + x,
                y: y_offset + row + y,
                scale,
                foreground: properties.foreground,
                background: properties.background,
            };
            kprint_generic(is_foreground, &local_prop, framebuffer)?;
            x_offset += scale;
        }
        y_offset += scale;
        x_offset = 0;
    }
    Ok(())
}

unsafe fn kprint_generic(
    is_foreground: bool,
    properties: &CharacterProperties,
    framebuffer: &Framebuffer,
) -> Result<(), Error> {
    for y_thick in 0..(properties.scale + 1) {
        for x_thick in 0..(properties.scale + 1) {
            let row_offset = (properties.y + y_thick) * framebuffer.pitch();
            let col_offset = (properties.x + x_thick) * 4;
            let pixel_offset = row_offset + col_offset;

            let color = if is_foreground {
                properties.foreground
            } else {
                properties.background
            };
            *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = color;
        }
    }
    Ok(())
}
