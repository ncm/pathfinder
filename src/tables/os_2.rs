// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use byteorder::{BigEndian, ReadBytesExt};
use error::FontError;
use font::FontTable;
use std::mem;
use util::Jump;

pub const TAG: u32 = ((b'O' as u32) << 24) |
                      ((b'S' as u32) << 16) |
                      ((b'/' as u32) << 8)  |
                       (b'2' as u32);

#[derive(Clone, Debug)]
pub struct Os2Table {
    pub typo_ascender: i16,
    pub typo_descender: i16,
    pub typo_line_gap: i16,
}

impl Os2Table {
    pub fn new(table: FontTable) -> Result<Os2Table, FontError> {
        let mut reader = table.bytes;

        // We should be compatible with all versions. If this is greater than version 5, follow
        // Postel's law and hope for the best.
        let version = try!(reader.read_u16::<BigEndian>().map_err(FontError::eof));

        // Skip to the line gap.
        try!(reader.jump(mem::size_of::<u16>() * 15).map_err(FontError::eof));
        try!(reader.jump(10).map_err(FontError::eof));
        if version == 0 {
            try!(reader.jump(mem::size_of::<u32>() * 2).map_err(FontError::eof));
        } else {
            try!(reader.jump(mem::size_of::<u32>() * 5).map_err(FontError::eof));
        }
        try!(reader.jump(mem::size_of::<u16>() * 3).map_err(FontError::eof));

        // Read the line spacing information.
        let typo_ascender = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let typo_descender = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));
        let typo_line_gap = try!(reader.read_i16::<BigEndian>().map_err(FontError::eof));

        Ok(Os2Table {
            typo_ascender: typo_ascender,
            typo_descender: typo_descender,
            typo_line_gap: typo_line_gap,
        })
    }
}


