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

#![feature(iter_advance_by)]
#![feature(let_else)]

use std::collections::BTreeSet;
use std::str;

use ironjvm_javautil::be::JavaBeUtil;
use ironjvm_javautil::jstr::JStr;
use ironjvm_specimpl::classfile::attrinfo::AttributeInfoType;
use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::flags::ClassAccessFlags;
use ironjvm_specimpl::classfile::flags::FieldAccessFlags;
use ironjvm_specimpl::classfile::flags::FlagsExt;
use ironjvm_specimpl::classfile::flags::MethodAccessFlags;
use ironjvm_specimpl::classfile::ClassFile;
use ironjvm_specimpl::classfile::FieldInfo;
use ironjvm_specimpl::classfile::MethodInfo;

use crate::error::CheckError;
use crate::error::CheckResult;

mod error;

pub struct ClassFileChecker<'clazz> {
    classfile: ClassFile<'clazz>,
    state: ClassFileCheckerState,
}

impl<'clazz> ClassFileChecker<'clazz> {
    pub fn new(classfile: ClassFile<'clazz>) -> Self {
        let major = classfile.major_version;

        Self {
            classfile,
            state: ClassFileCheckerState::new(major),
        }
    }

    pub fn check(&mut self) -> CheckResult<()> {
        self.check_classfile_version()?;
        self.check_class_access_flags()?;
        self.check_this_class()?;
        self.check_super_class()?;
        self.check_interfaces()?;
        self.check_fields()?;
        self.check_methods()?;

        todo!()
    }

    fn check_classfile_version(&self) -> CheckResult<()> {
        let minor = self.classfile.minor_version;

        if !(45u16..=62u16).contains(&self.state.major) {
            return Err(CheckError::UnsupportedMajor {
                major: self.state.major,
            });
        }

        if self.state.major > 56 && ![0, 65535].contains(&minor) {
            return Err(CheckError::InvalidMinor { minor });
        }

        Ok(())
    }

    fn check_class_access_flags(&mut self) -> CheckResult<()> {
        let access_flags = self.classfile.access_flags;

        if access_flags.flag_set(ClassAccessFlags::ACC_MODULE) {
            if access_flags != ClassAccessFlags::ACC_MODULE {
                return Err(CheckError::NotOnlyModuleFlagSet);
            }

            if self.state.major < 53 {
                return Err(CheckError::UnsupportedModuleFlagForVersion);
            }

            self.state.is_module = true;

            return Ok(());
        }

        if access_flags.flag_set(ClassAccessFlags::ACC_INTERFACE) {
            if access_flags.flag_set(ClassAccessFlags::ACC_ABSTRACT) {
                return Err(CheckError::InterfaceFlagWithoutAbstractFlag);
            }

            if access_flags.flag_set(ClassAccessFlags::ACC_FINAL)
                || access_flags.flag_set(ClassAccessFlags::ACC_SUPER)
                || access_flags.flag_set(ClassAccessFlags::ACC_ENUM)
                || access_flags.flag_set(ClassAccessFlags::ACC_MODULE)
            {
                return Err(CheckError::InvalidFlagsWithInterfaceFlag);
            }

            self.state.is_interface = true;
        }

        if access_flags.flag_set(ClassAccessFlags::ACC_ABSTRACT | ClassAccessFlags::ACC_FINAL) {
            return Err(CheckError::FinalAbstractFlagsSetSimultaneously);
        }

        if access_flags.flag_set(ClassAccessFlags::ACC_ANNOTATION)
            && !access_flags.flag_set(ClassAccessFlags::ACC_INTERFACE)
        {
            return Err(CheckError::AnnotationFlagWithoutInterfaceFlag);
        }

        Ok(())
    }

    fn check_this_class(&self) -> CheckResult<()> {
        let this_class = self.classfile.this_class;

        let Some(cp_info) = self.classfile.constant_pool.get(this_class as usize - 1) else {
            return Err(CheckError::InvalidConstantPoolIndex);
        };

        let CpInfoType::ConstantClass { .. } = cp_info.info else {
            return Err(CheckError::ThisClassIndexNotConstantClass);
        };

        Ok(())
    }

    fn check_super_class(&self) -> CheckResult<()> {
        let super_class = self.classfile.super_class;

        if super_class == 0 {
            // FIXME: verify that this class actually represents java/lang/Object
            return Ok(());
        }

        let Some(cp_info) = self.classfile.constant_pool.get(super_class as usize - 1) else {
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
                .get(interface_index.to_u16() as usize - 1);

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
        self.check_field_duplicates()?;
        self.check_field_access_flags()?;

        let mut fields_iter = self.classfile.fields.iter();

        if fields_iter.any(|field| {
            self.classfile
                .constant_pool
                .get((field.name_index - 1) as usize)
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

        if fields_iter.any(|field| {
            let descriptor_index = field.descriptor_index;
            let Some(CpInfoType::ConstantUtf8 { bytes, .. }) = self.classfile
                .constant_pool
                .get((descriptor_index - 1) as usize)
                .filter(|some| {
                    let CpInfoType::ConstantUtf8 { .. } = some.info else {
                        return false;
                    };

                    true
                })
                .map(|cp_info| cp_info.info.clone()) else {
                return false;
            };

            !self.check_field_descriptor(unsafe { JStr::from_jutf8_unchecked(bytes) })
        }) {
            return Err(CheckError::InvalidFieldDescriptor);
        }

        if fields_iter.any(|field| !self.check_field_attributes(field)) {
            return Err(CheckError::InvalidFieldAttributes);
        }

        Ok(())
    }

    // FIXME: optimize this function
    fn check_field_duplicates(&self) -> CheckResult<()> {
        let mut set = BTreeSet::new();
        let mut fields_iter = self.classfile.fields.iter();

        while let Some(field) = fields_iter.next() {
            let opt = self
                .classfile
                .constant_pool
                .get(field.name_index as usize - 1);
            if let Some(name) = opt {
                if let CpInfoType::ConstantUtf8 { bytes, .. } = name.info {
                    if !set.insert(unsafe { JStr::from_jutf8_unchecked(bytes) }) {
                        return Err(CheckError::DuplicatedField);
                    }
                }
            }
        }

        Ok(())
    }

    fn check_field_access_flags(&self) -> CheckResult<()> {
        let mut fields_iter = self.classfile.fields.iter();

        if self.state.is_interface {
            if fields_iter.any(|field| {
                let access_flags = field.access_flags;

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
            if fields_iter.any(|field| {
                let access_flags = field.access_flags;

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

        Ok(())
    }

    fn check_field_attributes(&self, field: &FieldInfo) -> bool {
        assert_eq!(field.attributes_count as usize, field.attributes.len());

        if field
            .attributes
            .iter()
            .any(|attribute| !match attribute.info {
                AttributeInfoType::ConstantValueAttribute { .. }
                | AttributeInfoType::DeprecatedAttribute
                | AttributeInfoType::SyntheticAttribute => true,
                AttributeInfoType::SignatureAttribute { .. }
                | AttributeInfoType::RuntimeInvisibleAnnotationsAttribute { .. }
                | AttributeInfoType::RuntimeVisibleAnnotationsAttribute { .. }
                    if self.state.major >= 49 =>
                {
                    true
                }
                AttributeInfoType::RuntimeInvisibleTypeAnnotationsAttribute { .. }
                | AttributeInfoType::RuntimeVisibleTypeAnnotationsAttribute { .. }
                    if self.state.major >= 52 =>
                {
                    true
                }
                _ => false,
            })
        {
            return false;
        }

        true
    }

    fn check_field_descriptor(&self, descriptor: &JStr) -> bool {
        match **descriptor {
            "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z" => true,
            str if str.starts_with("L") && str.ends_with(";") => true,
            str if str.starts_with("[") => {
                let dimensions = str.matches("[").count();

                if dimensions > 255 {
                    return false;
                }

                let mut chars = str.chars();
                if chars.advance_by(dimensions).is_err() {
                    return false;
                }

                self.check_field_descriptor(chars.as_str())
            }
            _ => false,
        }
    }

    fn check_methods(&self) -> CheckResult<()> {
        self.check_method_duplicates()?;
        self.check_method_access_flags()?;

        Ok(())
    }

    // FIXME: optimize this function
    fn check_method_duplicates(&self) -> CheckResult<()> {
        let mut set = BTreeSet::new();
        let mut methods_iter = self.classfile.fields.iter();

        while let Some(method) = methods_iter.next() {
            let opt = self
                .classfile
                .constant_pool
                .get(method.name_index as usize - 1);
            if let Some(name) = opt {
                if let CpInfoType::ConstantUtf8 { bytes, .. } = name.info {
                    if !set.insert(unsafe { JStr::from_jutf8_unchecked(bytes) }) {
                        return Err(CheckError::DuplicatedMethod);
                    }
                }
            }
        }

        Ok(())
    }

    fn check_method_access_flags(&self) -> CheckResult<()> {
        let mut methods_iter = self.classfile.methods.iter();

        if self.state.is_interface {
            if methods_iter.any(|method| {
                let access_flags = method.access_flags;

                access_flags.flag_set(MethodAccessFlags::ACC_PROTECTED)
                    || access_flags.flag_set(MethodAccessFlags::ACC_FINAL)
                    || access_flags.flag_set(MethodAccessFlags::ACC_SYNCHRONIZED)
                    || access_flags.flag_set(MethodAccessFlags::ACC_NATIVE)
            }) {
                return Err(CheckError::InvalidInterfaceMethodFlags);
            }

            if self.state.major < 52 {
                if methods_iter.any(|method| {
                    let access_flags = method.access_flags;

                    !access_flags
                        .flag_set(MethodAccessFlags::ACC_PUBLIC | MethodAccessFlags::ACC_ABSTRACT)
                }) {
                    return Err(CheckError::InvalidInterfaceMethodFlags);
                }
            } else {
                if methods_iter.any(|method| {
                    let access_flags = method.access_flags;

                    access_flags
                        .flag_set(MethodAccessFlags::ACC_PUBLIC | MethodAccessFlags::ACC_PRIVATE)
                }) {
                    return Err(CheckError::InvalidInterfaceMethodFlags);
                }
            }
        } else {
            if methods_iter.any(|method| {
                let access_flags = method.access_flags;

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
            }) {
                return Err(CheckError::InvalidMethodFlags);
            }
        }

        if methods_iter.any(|method| {
            let access_flags = method.access_flags;

            if access_flags.flag_set(MethodAccessFlags::ACC_ABSTRACT)
                && (access_flags.flag_set(MethodAccessFlags::ACC_PRIVATE)
                    || access_flags.flag_set(MethodAccessFlags::ACC_STATIC)
                    || access_flags.flag_set(MethodAccessFlags::ACC_FINAL)
                    || access_flags.flag_set(MethodAccessFlags::ACC_SYNCHRONIZED)
                    || access_flags.flag_set(MethodAccessFlags::ACC_NATIVE)
                    || if (46u16..=60u16).contains(&self.state.major) {
                        access_flags.flag_set(MethodAccessFlags::ACC_STRICT)
                    } else {
                        false
                    })
            {
                true
            } else {
                false
            }
        }) {
            return Err(CheckError::InvalidMethodFlags);
        }

        todo!()
    }

    fn check_methods_get_clinit(&self) -> Option<MethodInfo> {
        let mut methods_iter = self.classfile.methods.iter();
        methods_iter
            .find(|method| {
                let opt = &self
                    .classfile
                    .constant_pool
                    .get(method.name_index as usize - 1);
                if opt.is_none() {
                    return false;
                }

                let name_cp_info = opt.unwrap();
                let CpInfoType::ConstantUtf8 { bytes, .. } = &name_cp_info.info else {
                unreachable!()
            };

                let string = unsafe { JStr::from_jutf8_unchecked(bytes) };

                // FIXME: check method descriptor to be "V"
                **string == "<clinit>"
            })
            .map(|method| method.clone())
    }
}

struct ClassFileCheckerState {
    is_interface: bool,
    is_module: bool,
    major: u16,
}

impl ClassFileCheckerState {
    pub(crate) fn new(major: u16) -> Self {
        Self {
            is_interface: false,
            is_module: false,
            major,
        }
    }
}
