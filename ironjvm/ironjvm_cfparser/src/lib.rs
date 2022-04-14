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

use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;

use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::{ClassFile, CpInfo, FieldInfo};

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
        let magic = self.parse_magic()?;
        let minor_version = self.next_u2()?;
        let major_version = self.next_u2()?;
        let constant_pool_count = self.next_u2()?;
        let constant_pool = self.parse_constant_pool(constant_pool_count)?;
        let access_flags = self.next_u2()?;
        let this_class = self.next_u2()?;
        let super_class = self.next_u2()?;
        let interfaces_count = self.next_u2()?;
        let interfaces = self.parse_interfaces(interfaces_count)?;
        let fields_count = self.next_u2()?;
    }

    fn next_u1(&mut self) -> ParseResult<u8> {
        self.classfile
            .read_u8()
            .map_err(|src| ParseError::IoError { src })
    }

    fn next_u2(&mut self) -> ParseResult<u16> {
        self.classfile
            .read_u16::<BigEndian>()
            .map_err(|src| ParseError::IoError { src })
    }

    fn next_u4(&mut self) -> ParseResult<u32> {
        self.classfile
            .read_u32::<BigEndian>()
            .map_err(|src| ParseError::IoError { src })
    }

    fn parse_magic(&mut self) -> ParseResult<u32> {
        self.next_u4().and_then(|magic| {
            if magic == 0xCAFEBABE {
                Ok(magic)
            } else {
                Err(ParseError::InvalidMagic)
            }
        })
    }

    fn parse_constant_pool(&mut self, count: u16) -> ParseResult<Vec<CpInfo>> {
        let capacity = (count - 1) as usize;
        let mut pool = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            let tag = self.next_u1()?;
            let info = match tag {
                1 => {
                    let length = self.next_u2()?;
                    let mut bytes = Vec::with_capacity(length as usize);
                    self.classfile.read_exact(bytes.as_mut_slice());

                    CpInfoType::ConstantUtf8 { length, bytes }
                }
                3 => {
                    let bytes = self.next_u4()?;

                    CpInfoType::ConstantInteger { bytes }
                }
                4 => {
                    let bytes = self.next_u4()?;

                    CpInfoType::ConstantFloat { bytes }
                }
                5 => {
                    let high_bytes = self.next_u4()?;
                    let low_bytes = self.next_u4()?;

                    CpInfoType::ConstantLong { high_bytes, low_bytes }
                }
                6 => {
                    let high_bytes = self.next_u4()?;
                    let low_bytes = self.next_u4()?;

                    CpInfoType::ConstantDouble { high_bytes, low_bytes }
                }
                7 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantClass { name_index }
                }
                8 => {
                    let string_index = self.next_u2()?;

                    CpInfoType::ConstantString { string_index }
                }
                9 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantFieldRef { class_index, name_and_type_index }
                }
                10 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantMethodRef { class_index, name_and_type_index }
                }
                11 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantInterfaceMethodRef { class_index, name_and_type_index }
                }
                12 => {
                    let name_index = self.next_u2()?;
                    let descriptor_index = self.next_u2()?;

                    CpInfoType::ConstantNameAndType { name_index, descriptor_index }
                }
                15 => {
                    let reference_kind = self.next_u1()?;
                    let reference_index = self.next_u2()?;

                    CpInfoType::ConstantMethodHandle { reference_kind, reference_index }
                }
                16 => {
                    let descriptor_index = self.next_u2()?;

                    CpInfoType::ConstantMethodType { descriptor_index }
                }
                17 => {
                    let bootstrap_method_attr_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantDynamic { bootstrap_method_attr_index, name_and_type_index }
                }
                18 => {
                    let bootstrap_method_attr_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantInvokeDynamic { bootstrap_method_attr_index, name_and_type_index }
                }
                19 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantModule { name_index }
                }
                20 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantPackage { name_index }
                }
                _ => unreachable!()
            };

            pool.push(CpInfo { tag, info });
        }

        Ok(pool)
    }

    fn parse_interfaces(&mut self, count: u16) -> ParseResult<Vec<u16>> {
        let mut vec = Vec::with_capacity(count as usize);
        self.classfile.read_u16_into(vec.as_mut_slice())?;

        Ok(vec)
    }
}
