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

pub mod attrinfo;
pub mod cpinfo;
pub mod flags;

#[derive(Debug)]
pub struct AttributeInfo<'clazz> {
    pub attribute_name_index: [u8; 2],
    pub attribute_length: u32,
    pub info: attrinfo::AttributeInfoType<'clazz>,
}

#[derive(Debug)]
pub struct ClassFile<'clazz> {
    pub magic: u32,
    pub minor_version: [u8; 2],
    pub major_version: [u8; 2],
    pub constant_pool_count: [u8; 2],
    pub constant_pool: Vec<CpInfo<'clazz>>,
    pub access_flags: [u8; 2],
    pub this_class: [u8; 2],
    pub super_class: [u8; 2],
    pub interfaces_count: [u8; 2],
    pub interfaces: &'clazz [[u8; 2]],
    pub fields_count: [u8; 2],
    pub fields: Vec<FieldInfo<'clazz>>,
    pub methods_count: [u8; 2],
    pub methods: Vec<MethodInfo<'clazz>>,
    pub attributes_count: [u8; 2],
    pub attributes: Vec<AttributeInfo<'clazz>>,
}

#[derive(Debug)]
pub struct CpInfo<'clazz> {
    pub tag: u8,
    pub info: cpinfo::CpInfoType<'clazz>,
}

#[derive(Debug)]
pub struct FieldInfo<'clazz> {
    pub access_flags: [u8; 2],
    pub name_index: [u8; 2],
    pub descriptor_index: [u8; 2],
    pub attributes_count: [u8; 2],
    pub attributes: Vec<AttributeInfo<'clazz>>,
}

#[derive(Debug)]
pub struct MethodInfo<'clazz> {
    pub access_flags: [u8; 2],
    pub name_index: [u8; 2],
    pub descriptor_index: [u8; 2],
    pub attributes_count: [u8; 2],
    pub attributes: Vec<AttributeInfo<'clazz>>,
}
