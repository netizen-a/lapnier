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

pub enum Error {
    NoResponse,
    NoBuffers,
}

pub struct CharacterProperties {
    pub x: usize,
    pub y: usize,
    pub scale: usize,
    pub foreground: u32,
    pub background: u32,
}

pub unsafe fn kprint_char(character: u8, properties: &CharacterProperties) -> Result<(), Error> {
    let font = FONT_6X8;
    let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() else {
        return Err(Error::NoResponse);
    };
    let Some(framebuffer) = framebuffer_response.framebuffers().next() else {
        return Err(Error::NoBuffers);
    };
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
        let data = font[bitmap_index][row];
        for col in 2..8 {
            for y_thick in 0..(scale + 1) {
                for x_thick in 0..(scale + 1) {
                    let row_offset = (x + y_thick + y_offset + row) * framebuffer.pitch() as usize;
                    let col_offset = (y + x_thick + x_offset + col) * 4;
                    let pixel_offset = row_offset + col_offset;

                    let color = if data & (1 << (7 - col)) != 0 {
                        properties.foreground
                    } else {
                        properties.background
                    };
                    *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = color;
                }
            }
            x_offset += scale;
        }
        y_offset += scale;
        x_offset = 0;
    }
    Ok(())
}

pub unsafe fn kprint(
    string: &[u8],
    scale: usize,
    foreground: u32,
    background: u32,
) -> Result<(), Error> {
    let mut properties = CharacterProperties {
        x: 0,
        y: 0,
        scale,
        foreground,
        background,
    };
    let y_offset = if scale == 0 { 1 } else { scale + 1 };
    for (col, c) in string.iter().enumerate() {
        properties.y = y_offset * col * 6;
        kprint_char(*c, &properties)?;
    }
    Ok(())
}
