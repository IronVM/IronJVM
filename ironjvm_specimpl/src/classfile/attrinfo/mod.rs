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

use rend::{u16_be, u32_be};

use crate::classfile::AttributeInfo;

pub mod smtattr;

pub enum AttributeInfoType {
    ConstantValueAttribute {
        constantvalue_index: u16_be,
    },
    CodeAttribute {
        max_stack: u16_be,
        max_locals: u16_be,
        code_length: u32_be,
        code: Vec<u8>,
        exception_table_length: u16_be,
        exception_table: Vec<CodeAttributeExceptionTableEntry>,
        attributes_count: u16_be,
        attributes: Vec<AttributeInfo>,
    },
}

pub struct CodeAttributeExceptionTableEntry {
    pub start_pc: u16_be,
    pub end_pc: u16_be,
    pub handler_pc: u16_be,
    pub catch_type: u16_be,
}
