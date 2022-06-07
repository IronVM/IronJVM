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
use ironjvm_specimpl::classfile::flags::{ClassAccessFlags, FieldAccessFlags, FlagsExt};
use ironjvm_specimpl::classfile::ClassFile;

use crate::error::{CheckError, CheckResult};

mod error;

pub struct ClassFileChecker<'clazz> {
    classfile: ClassFile<'clazz>,
    state: ClassFileCheckerState,
}

impl<'clazz> ClassFileChecker<'clazz> {
    pub fn new(classfile: ClassFile<'clazz>) -> Self {
        Self {
            classfile,
            state: ClassFileCheckerState::default(),
        }
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

    fn u8_slice_to_u16(&self, bytes: [u8; 2]) -> u16 {
        u16::from_be_bytes(bytes)
    }

    fn check_cfver(&self) -> CheckResult<()> {
        let major = self.u8_slice_to_u16(self.classfile.major_version);
        let minor = self.u8_slice_to_u16(self.classfile.minor_version);

        if !(45u16..=62u16).contains(&major) {
            return Err(CheckError::UnsupportedMajor { major });
        }

        if major > 56 && ![0, 65535].contains(&minor) {
            return Err(CheckError::InvalidMinor { minor });
        }

        Ok(())
    }

    fn check_class_accflags(&mut self) -> CheckResult<()> {
        let access_flags = self.u8_slice_to_u16(self.classfile.access_flags);

        if access_flags.flag_set(ClassAccessFlags::ACC_MODULE) {
            if access_flags != ClassAccessFlags::ACC_MODULE {
                return Err(CheckError::NotOnlyModuleFlagSet);
            }

            if self.u8_slice_to_u16(self.classfile.major_version) < 53 {
                return Err(CheckError::UnsupportedModuleFlagForVersion);
            }

            self.state.is_module = true;

            return Ok(());
        }

        if access_flags.flag_set(ClassAccessFlags::ACC_INTERFACE) {
            if access_flags.flag_set(ClassAccessFlags::ACC_ABSTRACT) {
                return Err(CheckError::InterfaceFlagWithoutAbstractFlag);
            }

            if access_flags
                .flag_set(ClassAccessFlags::ACC_FINAL)
                || access_flags
                    .flag_set(ClassAccessFlags::ACC_SUPER)
                || access_flags
                    .flag_set(ClassAccessFlags::ACC_ENUM)
                || access_flags
                    .flag_set(ClassAccessFlags::ACC_MODULE)
            {
                return Err(CheckError::InvalidFlagsWithInterfaceFlag);
            }

            self.state.is_interface = true;
        }

        if access_flags
            .flag_set(ClassAccessFlags::ACC_ABSTRACT | ClassAccessFlags::ACC_FINAL)
        {
            return Err(CheckError::FinalAbstractFlagsSetSimultaneously);
        }

        if access_flags
            .flag_set(ClassAccessFlags::ACC_ANNOTATION)
            && !access_flags
                .flag_set(ClassAccessFlags::ACC_INTERFACE)
        {
            return Err(CheckError::AnnotationFlagWithoutInterfaceFlag);
        }

        Ok(())
    }

    fn check_this_class(&self) -> CheckResult<()> {
        let Some(cp_info) = self.classfile.constant_pool.get((self.u8_slice_to_u16(self.classfile.this_class) - 1) as usize) else {
            return Err(CheckError::InvalidConstantPoolIndex);
        };

        let CpInfoType::ConstantClass { .. } = cp_info.info else {
            return Err(CheckError::ThisClassIndexNotConstantClass);
        };

        Ok(())
    }

    fn check_super_class(&self) -> CheckResult<()> {
        let super_class = self.u8_slice_to_u16(self.classfile.super_class);

        if super_class  == 0 {
            // FIXME: verify that this class actually represents java/lang/Object
            return Ok(());
        }

        let Some(cp_info) = self.classfile.constant_pool.get((super_class - 1) as usize) else {
            return Err(CheckError::InvalidConstantPoolIndex);
        };

        let CpInfoType::ConstantClass { .. } = cp_info.info else {
            return Err(CheckError::SuperClassIndexNotConstantClass);
        };

        Ok(())
    }

    fn check_interfaces(&self) -> CheckResult<()> {
        assert_eq!(
            self.u8_slice_to_u16(self.classfile.interfaces_count) as usize,
            self.classfile.interfaces.len()
        );

        if self.classfile.interfaces.iter().any(|interface_index| {
            let cp_info_opt = self
                .classfile
                .constant_pool
                .get((self.u8_slice_to_u16(*interface_index) - 1) as usize);

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

    fn check_fields(&self) -> CheckResult<()> {
        if self.state.is_interface {
            if self.classfile.fields.iter().any(|field| {
                let access_flags = self.u8_slice_to_u16(field.access_flags);

                !access_flags.flag_set(
                    FieldAccessFlags::ACC_PUBLIC
                        | FieldAccessFlags::ACC_STATIC
                        | FieldAccessFlags::ACC_FINAL,
                ) || access_flags
                    != FieldAccessFlags::ACC_PUBLIC
                        | FieldAccessFlags::ACC_STATIC
                        | FieldAccessFlags::ACC_FINAL
                    || access_flags
                        != FieldAccessFlags::ACC_PUBLIC
                            | FieldAccessFlags::ACC_STATIC
                            | FieldAccessFlags::ACC_FINAL
                            | FieldAccessFlags::ACC_SYNTHETIC
            }) {
                return Err(CheckError::InvalidInterfaceFieldFlags);
            }
        } else {
            if self.classfile.fields.iter().any(|field| {
                let access_flags = self.u8_slice_to_u16(field.access_flags);

                access_flags.flag_set(
                    FieldAccessFlags::ACC_PUBLIC
                        | FieldAccessFlags::ACC_PRIVATE
                        | FieldAccessFlags::ACC_PROTECTED,
                ) || access_flags
                    .flag_set(FieldAccessFlags::ACC_PUBLIC | FieldAccessFlags::ACC_PRIVATE)
                    || access_flags
                        .flag_set(FieldAccessFlags::ACC_PRIVATE | FieldAccessFlags::ACC_PROTECTED)
                    || access_flags
                        .flag_set(FieldAccessFlags::ACC_PUBLIC | FieldAccessFlags::ACC_PROTECTED)
                    || access_flags
                        .flag_set(FieldAccessFlags::ACC_FINAL | FieldAccessFlags::ACC_VOLATILE)
            }) {
                return Err(CheckError::InvalidFieldFlags);
            }
        }

        if self.classfile.fields.iter().any(|field| {
            let cp_index = field.name_index;
            self.classfile
                .constant_pool
                .get((self.u8_slice_to_u16(cp_index) - 1) as usize)
                .filter(|some| {
                    let CpInfoType::ConstantUtf8 { .. } = some.info else {
                    return false;
                };

                    true
                })
                .is_none()
        }) {
            return Err(CheckError::FieldNameIndexNotConstantUtf8);
        }

        // if self.classfile.fields.iter().any(|field| {
        //     let descriptor_index = field.descriptor_index;
        //     let Some(CpInfoType::ConstantUtf8 { bytes, .. }) = self.classfile
        //         .constant_pool
        //         .get((descriptor_index - 1) as usize)
        //         .filter(|some| {
        //             let CpInfoType::ConstantUtf8 { .. } = some.info else {
        //                 return false;
        //             };
        //
        //             true
        //         })
        //         .map(|cp_info| cp_info.info) else {
        //         return false;
        //     };
        //
        //     let _ = unsafe { String::from_utf8_unchecked(bytes.clone()) };
        //
        //     todo!()
        // }) {}

        Ok(())
    }
}

struct ClassFileCheckerState {
    is_interface: bool,
    is_module: bool,
}

impl Default for ClassFileCheckerState {
    fn default() -> Self {
        Self {
            is_interface: false,
            is_module: false,
        }
    }
}
