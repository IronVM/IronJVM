// SPDX-License-Identifier: GPL-2.0
/*
 * IronJVM: JVM Implementation in Rust
 * Copyright (C) 2022 HTGAzureX1212.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use std::fs::File;
use byteorder::{BigEndian, ReadBytesExt};

use ironjvm_specimpl::classfile::ClassFile;

use crate::error::{ParseError, ParseResult};

mod error;

pub struct ClassFileParser {
    pub classfile: File,
}

impl ClassFileParser {
    pub fn new(classfile: File) -> Self {
        Self { classfile }
    }

    pub fn parse(mut self) -> ParseResult<ClassFile> {
        let _ = self.parse_magic()?;
    }

    fn next_u1(&mut self) -> ParseResult<u8> {
        self.classfile.read_u8().map_err(|src| ParseError::IoError { src })
    }

    fn next_u2(&mut self) -> ParseResult<u16> {
        self.classfile.read_u16::<BigEndian>().map_err(|src| ParseError::IoError { src })
    }

    fn next_u4(&mut self) -> ParseResult<u32> {
        self.classfile.read_u32::<BigEndian>().map_err(|src| ParseError::IoError { src })
    }

    fn parse_magic(&mut self) -> ParseResult<u32> {
        self.next_u4().and_then(|magic| if magic == 0xCAFEBABE { Ok(magic) } else { Err(ParseError::InvalidMagic) })
    }
}
