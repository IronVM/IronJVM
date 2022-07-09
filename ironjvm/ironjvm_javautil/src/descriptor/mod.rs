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

// Credit: code referenced from https://gitlab.com/frozo/noak/

use crate::descriptor::error::InvalidDescriptorError;
use crate::jstr::JStr;

pub mod error;
pub mod field;
pub mod method;

#[derive(Clone)]
pub enum BaseType<'a> {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Object(&'a JStr),
}

pub struct TypeDescriptor<'a> {
    pub dimensions: u8,
    pub r#type: BaseType<'a>,
}

impl<'a> TypeDescriptor<'a> {
    pub fn from_jstr(input: &'a JStr) -> Result<Self, InvalidDescriptorError> {
        let mut chars = input.chars_lossy().enumerate();
        let mut dimensions = 0u8;

        while let Some((index, char)) = chars.next() {
            if char == '[' {
                dimensions = if let Some(dimensions) = dimensions.checked_add(1) {
                    dimensions
                } else {
                    break;
                }
            } else {
                let r#type = match char {
                    'Z' => BaseType::Boolean,
                    'B' => BaseType::Byte,
                    'C' => BaseType::Char,
                    'D' => BaseType::Double,
                    'F' => BaseType::Float,
                    'I' => BaseType::Int,
                    'J' => BaseType::Long,
                    'S' => BaseType::Short,
                    'L' => {
                        if !chars.any(|(_, char)| char == ';') {
                            break;
                        }

                        let name = &input[index + 1..input.len() - 1];
                        if name.is_empty() {
                            break;
                        }

                        BaseType::Object(name)
                    }
                    _ => break,
                };

                if chars.next().is_some() {
                    break;
                }

                return Ok(Self { dimensions, r#type });
            }
        }

        Err(InvalidDescriptorError)
    }
}
