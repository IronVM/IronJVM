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

use std::str;

use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::{ClassFile, MethodInfo};

pub trait JavaCfUtil<'clazz> {
    fn instance_initialization_methods(&'clazz self) -> Vec<MethodInfo<'clazz>>;
}

impl<'clazz> JavaCfUtil<'clazz> for ClassFile<'clazz> {
    fn instance_initialization_methods(&'clazz self) -> Vec<MethodInfo<'clazz>> {
        self.methods
            .iter()
            .filter(|method| {
                let name_index = method.name_index;
                let Some(CpInfoType::ConstantUtf8 { bytes, .. }) = self.constant_pool
                    .get((name_index - 1) as usize)
                    .filter(|some| {
                        let CpInfoType::ConstantUtf8 { .. } = some.info else {
                            return false;
                        };

                        true
                    })
                    .map(|cp_info| cp_info.info.clone()) else {
                        return false;
                    };

                unsafe { str::from_utf8_unchecked(bytes) } == "<init>"
            })
            .map(|method| method.clone())
            .collect()
    }
}
