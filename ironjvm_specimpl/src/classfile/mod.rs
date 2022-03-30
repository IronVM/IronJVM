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

pub mod attrinfo;
pub mod cpinfo;
pub mod flags;

pub struct AttributeInfo {
    pub attribute_name_index: u16_be,
    pub attribute_length: u32_be,
    pub info: attrinfo::AttributeInfoType,
}

pub struct ClassFile {
    pub magic: u32_be,
    pub minor_version: u16_be,
    pub major_version: u16_be,
    pub constant_pool_count: u16_be,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: u16_be,
    pub this_class: u16_be,
    pub super_class: u16_be,
    pub interfaces_count: u16_be,
    pub interfaces: Vec<u16_be>,
    pub fields_count: u16_be,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16_be,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16_be,
    pub attributes: Vec<AttributeInfo>,
}

pub struct CpInfo {
    pub tag: u8,
    pub info: cpinfo::CpInfoType,
}

pub struct FieldInfo {
    pub access_flags: u16_be,
    pub name_index: u16_be,
    pub descriptor_index: u16_be,
    pub attributes_count: u16_be,
    pub attributes: Vec<AttributeInfo>,
}

pub struct MethodInfo {
    pub access_flags: u16_be,
    pub name_index: u16_be,
    pub descriptor_index: u16_be,
    pub attributes_count: u16_be,
    pub attributes: Vec<AttributeInfo>,
}
