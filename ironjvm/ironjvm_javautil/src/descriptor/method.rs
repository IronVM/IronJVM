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

use crate::descriptor::TypeDescriptor;

pub struct MethodDescriptor<'a> {
    pub length: u8,
    pub parameters: Vec<ParameterDescriptor<'a>>,
    pub return_descriptor: ReturnDescriptor<'a>,
}

pub enum ReturnDescriptor<'a> {
    FieldType(TypeDescriptor<'a>),
    VoidDescriptor,
}

pub type ParameterDescriptor<'a> = TypeDescriptor<'a>;
