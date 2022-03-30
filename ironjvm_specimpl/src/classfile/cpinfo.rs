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

#[repr(u8)]
pub enum CpInfoType {
    ConstantUtf8 {
        length: u16_be,
        bytes: Vec<u8>,
    } = 1,
    ConstantInteger {
        bytes: u32_be,
    } = 3,
    ConstantFloat {
        bytes: u32_be,
    } = 4,
    ConstantLong {
        high_bytes: u32_be,
        low_bytes: u32_be,
    } = 5,
    ConstantDouble {
        high_bytes: u32_be,
        low_bytes: u32_be,
    } = 6,
    ConstantClass {
        name_index: u16_be,
    } = 7,
    ConstantString {
        string_index: u16_be,
    } = 8,
    ConstantFieldRef {
        class_index: u16_be,
        name_and_type_index: u16_be,
    } = 9,
    ConstantMethodRef {
        class_index: u16_be,
        name_and_type_index: u16_be,
    } = 10,
    ConstantInterfaceMethodRef {
        class_index: u16_be,
        name_and_type_index: u16_be,
    } = 11,
    ConstantNameAndTypeIndex {
        name_index: u16_be,
        descriptor_index: u16_be,
    } = 12,
    ConstantMethodHandle {
        reference_kind: u8,
        reference_index: u16_be,
    } = 15,
    ConstantMethodType {
        descriptor_index: u16_be,
    } = 16,
    ConstantDynamic {
        bootstrap_method_attr_index: u16_be,
        name_and_type_index: u16_be,
    } = 17,
    ConstantInvokeDynamic {
        bootstrap_method_attr_index: u16_be,
        name_and_type_index: u16_be,
    } = 18,
    ConstantModule {
        name_index: u16_be,
    } = 19,
    ConstantPackage {
        name_index: u16_be,
    } = 20,
}
