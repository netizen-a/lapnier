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

#[derive(Debug)]
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

unsafe fn kprint_generic(
    is_foreground: bool,
    properties: &CharacterProperties,
) -> Result<(), Error> {
    let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() else {
        return Err(Error::NoResponse);
    };
    let Some(framebuffer) = framebuffer_response.framebuffers().next() else {
        return Err(Error::NoBuffers);
    };
    for y_thick in 0..(properties.scale + 1) {
        for x_thick in 0..(properties.scale + 1) {
            let row_offset = (properties.y + y_thick) * framebuffer.pitch() as usize;
            let col_offset = (properties.x + x_thick) * 4;
            let pixel_offset = row_offset + col_offset;

            let color = if is_foreground {
                properties.foreground
            } else {
                properties.background
            };
            *(framebuffer.addr().add(pixel_offset) as *mut u32) = color;
        }
    }
    Ok(())
}

pub unsafe fn kprint_char(character: u8, properties: &CharacterProperties) -> Result<(), Error> {
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
        let data = font[bitmap_index][row];
        for col in 2..8 {
            let is_foreground = data & (1 << (7 - col)) != 0;
            let local_prop = CharacterProperties {
                x: x_offset + col + x,
                y: y_offset + row + y,
                scale,
                foreground: properties.foreground,
                background: properties.background,
            };
            kprint_generic(is_foreground, &local_prop)?;
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
    let scale_offset = if scale == 0 { 1 } else { scale + 1 };
    let mut col = 0;
    let mut row = 0;
    for c in string {
        match c {
            b'\r' => {
                col = 0;
            }
            b'\n' => {
                row += 1;
                properties.y = scale_offset * row * 8;
            }
            _ => {
                properties.x = scale_offset * col * 6;
                kprint_char(*c, &properties)?;
                col += 1;
            }
        }
    }
    Ok(())
}
