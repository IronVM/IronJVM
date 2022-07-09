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
use crate::descriptor::BaseType;
use crate::descriptor::TypeDescriptor;
use crate::jstr::CharsLossy;
use crate::jstr::JStr;

pub struct MethodDescriptor<'a> {
    input: &'a JStr,
    return_index: usize,
}

impl<'a> MethodDescriptor<'a> {
    pub fn from_jstr(input: &'a JStr) -> Result<Self, InvalidDescriptorError> {
        let mut chars = input.chars_lossy();
        if let Some('(') = chars.next() {
            loop {
                let char = chars.next();
                if let Some(')') = char {
                    break;
                }

                validate_descriptor(char, &mut chars, false)?
            }

            let return_index = input.len() - chars.as_jstr().len();
            validate_descriptor(chars.next(), &mut chars, true)?;

            if chars.next().is_none() {
                return Ok(Self {
                    input,
                    return_index,
                });
            }
        }

        Err(InvalidDescriptorError)
    }

    pub fn parameters_iter(&self) -> impl Iterator<Item = ParameterDescriptor<'a>> + 'a {
        ParametersDescriptorsIter {
            chars: self.input.chars_lossy(),
        }
    }

    pub fn return_type(&self) -> ReturnDescriptor<'a> {
        let input = &self.input[self.return_index..];
        if input.as_bytes() == b"V" {
            ReturnDescriptor::VoidDescriptor
        } else {
            let mut chars = input.chars_lossy();
            ReturnDescriptor::FieldType(read_descriptor(chars.next().unwrap(), &mut chars))
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ReturnDescriptor<'a> {
    FieldType(TypeDescriptor<'a>),
    VoidDescriptor,
}

pub struct ParametersDescriptorsIter<'a> {
    chars: CharsLossy<'a>,
}

impl<'a> Iterator for ParametersDescriptorsIter<'a> {
    type Item = ParameterDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars.next();
        if char == Some(')') || char == None {
            self.chars = <&JStr>::default().chars_lossy();
            None
        } else {
            Some(read_descriptor(char.unwrap(), &mut self.chars))
        }
    }
}

pub type ParameterDescriptor<'a> = TypeDescriptor<'a>;

fn read_descriptor<'a>(mut char: char, iter: &mut CharsLossy<'a>) -> TypeDescriptor<'a> {
    let mut dimensions = 0;
    while char == '[' {
        char = iter.next().unwrap();
        dimensions += 1;
    }

    let r#type = match char {
        'Z' => BaseType::Boolean,
        'B' => BaseType::Byte,
        'S' => BaseType::Short,
        'I' => BaseType::Int,
        'J' => BaseType::Long,
        'F' => BaseType::Float,
        'D' => BaseType::Double,
        'C' => BaseType::Char,
        'L' => {
            let input = iter.as_jstr();
            for char in iter.by_ref() {
                if char == ';' {
                    break;
                }
            }

            let name = &input[..input.len() - iter.as_jstr().len() - 1];
            BaseType::Object(name)
        }
        _ => unreachable!("the tag is guaranteed to be valid"),
    };

    TypeDescriptor { dimensions, r#type }
}

fn validate_descriptor(
    mut char: Option<char>,
    iter: &mut CharsLossy<'_>,
    is_return: bool,
) -> Result<(), InvalidDescriptorError> {
    if is_return && char == Some('V') {
        return Ok(());
    }

    let mut dimensions = 0u8;
    while let Some('[') = iter.next() {
        dimensions = dimensions.checked_add(1).ok_or(InvalidDescriptorError)?;
        char = iter.next();
    }

    if let Some(char) = char {
        match char {
            'Z' | 'B' | 'S' | 'I' | 'J' | 'F' | 'D' | 'C' => return Ok(()),
            'L' => {
                let mut found_semicolon = false;
                let mut found_character = false;
                for char in iter {
                    if char == ';' {
                        found_semicolon = true;
                        break;
                    }
                    found_character = true;
                }

                if found_semicolon && found_character {
                    return Ok(());
                }
            }
            _ => {}
        }
    }

    Err(InvalidDescriptorError)
}
