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

use rend::u16_be;

pub struct ClassAccessFlags;

impl ClassAccessFlags {
    pub const ACC_PUBLIC: u16_be = u16_be::new(0x0001);
    pub const ACC_FINAL: u16_be = u16_be::new(0x0010);
    pub const ACC_SUPER: u16_be = u16_be::new(0x0020);
    pub const ACC_INTERFACE: u16_be = u16_be::new(0x0200);
    pub const ACC_ABSTRACT: u16_be = u16_be::new(0x0400);
    pub const ACC_SYNTHETIC: u16_be = u16_be::new(0x1000);
    pub const ACC_ANNOTATION: u16_be = u16_be::new(0x2000);
    pub const ACC_ENUM: u16_be = u16_be::new(0x4000);
    pub const ACC_MODULE: u16_be = u16_be::new(0x8000);
}

pub struct FieldAccessFlags;

impl FieldAccessFlags {
    pub const ACC_PUBLIC: u16_be = u16_be::new(0x0001);
    pub const ACC_PRIVATE: u16_be = u16_be::new(0x0002);
    pub const ACC_PROTECTED: u16_be = u16_be::new(0x0004);
    pub const ACC_STATIC: u16_be = u16_be::new(0x0008);
    pub const ACC_FINAL: u16_be = u16_be::new(0x0010);
    pub const ACC_VOLATILE: u16_be = u16_be::new(0x0040);
    pub const ACC_TRANSIENT: u16_be = u16_be::new(0x0080);
    pub const ACC_SYNTHETIC: u16_be = u16_be::new(0x1000);
    pub const ACC_ENUM: u16_be = u16_be::new(0x4000);
}

pub struct MethodAccessFlags;

impl MethodAccessFlags {
    pub const ACC_PUBLIC: u16_be = u16_be::new(0x0001);
    pub const ACC_PRIVATE: u16_be = u16_be::new(0x0002);
    pub const ACC_PROTECTED: u16_be = u16_be::new(0x0004);
    pub const ACC_STATIC: u16_be = u16_be::new(0x0008);
    pub const ACC_FINAL: u16_be = u16_be::new(0x0010);
    pub const ACC_SYNCHRONIZED: u16_be = u16_be::new(0x0020);
    pub const ACC_BRIDGE: u16_be = u16_be::new(0x0040);
    pub const ACC_VARARGS: u16_be = u16_be::new(0x0080);
    pub const ACC_NATIVE: u16_be = u16_be::new(0x0100);
    pub const ACC_ABSTRACT: u16_be = u16_be::new(0x0400);
    pub const ACC_STRICT: u16_be = u16_be::new(0x0800);
    pub const ACC_SYNTHETIC: u16_be = u16_be::new(0x1000);
}
