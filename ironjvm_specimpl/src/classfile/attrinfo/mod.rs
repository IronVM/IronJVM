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

pub mod icattr;
pub mod lntattr;
pub mod lvtattr;
pub mod lvttattr;
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
    StackMapTableAttribute {
        number_of_entries: u16_be,
        stack_map_table: Vec<smtattr::StackMapFrame>,
    },
    ExceptionsAttribute {
        number_of_exceptions: u16_be,
        exception_index_table: Vec<u16_be>,
    },
    InnerClassesAttribute {
        number_of_classes: u16_be,
        classes: Vec<icattr::InnerClass>,
    },
    EnclosingMethodAttribute {
        class_index: u16_be,
        method_index: u16_be,
    },
    SyntheticAttribute,
    SignatureAttribute {
        signature_index: u16_be,
    },
    SourceFileAttribute {
        sourcefile_index: u16_be,
    },
    SourceDebugExtensionAttribute {
        debug_extension: Vec<u8>,
    },
    LineNumberTableAttribute {
        line_number_table_length: u16_be,
        line_numer_table: Vec<lntattr::LineNumber>,
    },
    LocalVariableTableAttribute {
        local_variable_table_length: u16_be,
        local_variable_table: Vec<lvtattr::LocalVariable>
    },
    LocalVariableTypeTableAttribute {
        local_variable_type_table_length: u16_be,
        local_variable_type_table: Vec<lvttattr::LocalVariableType>,
    },
    DeprecatedAttribute,
}

pub struct CodeAttributeExceptionTableEntry {
    pub start_pc: u16_be,
    pub end_pc: u16_be,
    pub handler_pc: u16_be,
    pub catch_type: u16_be,
}
