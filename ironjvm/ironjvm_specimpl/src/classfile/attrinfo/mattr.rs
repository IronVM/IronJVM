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

use std::borrow::Cow;

#[derive(Debug)]
pub struct ModuleExport<'clazz> {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Cow<'clazz, [u16]>,
}

#[derive(Debug)]
pub struct ModuleOpen<'clazz> {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Cow<'clazz, [u16]>,
}

#[derive(Debug)]
pub struct ModuleProvide<'clazz> {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Cow<'clazz, [u16]>,
}

#[derive(Debug)]
pub struct ModuleRequire {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}
