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

#![feature(let_else)]

use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::flags::{ClassAccessFlags, FlagsExt};
use ironjvm_specimpl::classfile::ClassFile;

use crate::error::{CheckError, CheckResult};

mod error;

pub struct ClassFileChecker {
    classfile: ClassFile,
    state: ClassFileCheckerState,
}

impl ClassFileChecker {
    pub fn new(classfile: ClassFile) -> Self {
        Self { classfile, state: ClassFileCheckerState::default() }
    }

    pub fn check(&mut self) -> CheckResult<()> {
        self.check_cfver()?;
        self.check_class_accflags()?;
        self.check_this_class()?;
        self.check_super_class()?;
        self.check_interfaces()?;
        self.check_fields()?;

        todo!()
    }

    fn check_cfver(&self) -> CheckResult<()> {
        if !(45u16..=62u16).contains(&self.classfile.major_version) {
            return Err(CheckError::UnsupportedMajor {
                major: self.classfile.major_version,
            });
        }

        if self.classfile.major_version > 56 && ![0, 65535].contains(&self.classfile.minor_version)
        {
            return Err(CheckError::InvalidMinor {
                minor: self.classfile.minor_version,
            });
        }

        Ok(())
    }

    fn check_class_accflags(&mut self) -> CheckResult<()> {
        if self
            .classfile
            .access_flags
            .flag_set(ClassAccessFlags::ACC_MODULE)
        {
            if self.classfile.access_flags != ClassAccessFlags::ACC_MODULE {
                return Err(CheckError::NotOnlyModuleFlagSet);
            }

            if self.classfile.major_version < 53 {
                return Err(CheckError::UnsupportedModuleFlagForVersion);
            }

            return Ok(());
        }

        if self
            .classfile
            .access_flags
            .flag_set(ClassAccessFlags::ACC_INTERFACE)
        {
            if !self
                .classfile
                .access_flags
                .flag_set(ClassAccessFlags::ACC_ABSTRACT)
            {
                return Err(CheckError::InterfaceFlagWithoutAbstractFlag);
            }

            if self
                .classfile
                .access_flags
                .flag_set(ClassAccessFlags::ACC_FINAL)
                || self
                    .classfile
                    .access_flags
                    .flag_set(ClassAccessFlags::ACC_SUPER)
                || self
                    .classfile
                    .access_flags
                    .flag_set(ClassAccessFlags::ACC_ENUM)
                || self
                    .classfile
                    .access_flags
                    .flag_set(ClassAccessFlags::ACC_MODULE)
            {
                return Err(CheckError::InvalidFlagsWithInterfaceFlag);
            }
        }

        if self
            .classfile
            .access_flags
            .flag_set(ClassAccessFlags::ACC_ABSTRACT | ClassAccessFlags::ACC_FINAL)
        {
            return Err(CheckError::FinalAbstractFlagsSetSimultaneously);
        }

        if self
            .classfile
            .access_flags
            .flag_set(ClassAccessFlags::ACC_ANNOTATION)
            && !self
                .classfile
                .access_flags
                .flag_set(ClassAccessFlags::ACC_INTERFACE)
        {
            return Err(CheckError::AnnotationFlagWithoutInterfaceFlag);
        }

        Ok(false)
    }

    fn check_this_class(&self) -> CheckResult<()> {
        let Some(cp_info) = self.classfile.constant_pool.get((self.classfile.this_class - 1) as usize) else {
            return Err(CheckError::InvalidConstantPoolIndex);
        };

        let CpInfoType::ConstantClass { .. } = cp_info.info else {
            return Err(CheckError::ThisClassIndexNotConstantClass);
        };

        Ok(())
    }

    fn check_super_class(&self) -> CheckResult<()> {
        if self.classfile.super_class == 0 {
            // FIXME: verify that this class actually represents java/lang/Object
            return Ok(());
        }

        let Some(cp_info) = self.classfile.constant_pool.get((self.classfile.super_class - 1) as usize) else {
            return Err(CheckError::InvalidConstantPoolIndex);
        };

        let CpInfoType::ConstantClass { .. } = cp_info.info else {
            return Err(CheckError::SuperClassIndexNotConstantClass);
        };

        Ok(())
    }

    fn check_interfaces(&self) -> CheckResult<()> {
        assert_eq!(
            self.classfile.interfaces_count as usize,
            self.classfile.interfaces.len()
        );

        if self.classfile.interfaces.iter().any(|interface_index| {
            let cp_info_opt = self
                .classfile
                .constant_pool
                .get((interface_index - 1) as usize);

            if cp_info_opt.is_none() {
                return true;
            }

            let CpInfoType::ConstantClass { .. } = cp_info_opt.unwrap().info else {
                return true;
            };

            false
        }) {
            return Err(CheckError::InvalidConstantPoolIndex);
        }

        Ok(())
    }
}

struct ClassFileCheckerState {
    is_module: bool,
}

impl Default for ClassFileCheckerState {
    fn default() -> Self {
        Self { is_module: false }
    }
}
