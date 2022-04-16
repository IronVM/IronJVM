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

use ironjvm_specimpl::classfile::ClassFile;

use crate::error::{CheckError, CheckResult};

mod error;

pub struct ClassFileChecker {
    classfile: ClassFile,
}

impl ClassFileChecker {
    pub fn new(classfile: ClassFile) -> Self {
        Self { classfile }
    }

    pub fn check(&self) -> CheckResult<()> {
        self.check_cfver()
    }

    fn check_cfver(&self) -> CheckResult<()> {
        if !(45u16..=62u16).contains(&self.classfile.major_version) {
            return Err(CheckError::UnsupportedMajor { major: self.classfile.major_version });
        }

        if self.classfile.major_version > 56 && ![0, 65535].contains(&self.classfile.minor_version) {
            return Err(CheckError::InvalidMinor { minor: self.classfile.minor_version })
        }

        Ok(())
    }
}
