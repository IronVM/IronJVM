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

use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;

use ironjvm_specimpl::classfile::attrinfo::bmattr::BootstrapMethod;
use ironjvm_specimpl::classfile::attrinfo::cattr::CodeAttributeExceptionTableEntry;
use ironjvm_specimpl::classfile::attrinfo::icattr::InnerClass;
use ironjvm_specimpl::classfile::attrinfo::lntattr::LineNumber;
use ironjvm_specimpl::classfile::attrinfo::lvtattr::LocalVariable;
use ironjvm_specimpl::classfile::attrinfo::lvttattr::LocalVariableType;
use ironjvm_specimpl::classfile::attrinfo::mattr::{
    ModuleExport, ModuleOpen, ModuleProvide, ModuleRequire,
};
use ironjvm_specimpl::classfile::attrinfo::mpattr::MethodParameter;
use ironjvm_specimpl::classfile::attrinfo::rattr::RecordComponentInfo;
use ironjvm_specimpl::classfile::attrinfo::rvanriaattr::{
    Annotation, ElementValue, ElementValuePair, ElementValueValue,
};
use ironjvm_specimpl::classfile::attrinfo::rvpaattr::ParameterAnnotation;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::{
    TypeAnnotation, TypeAnnotationLocalVarTargetTableEntry, TypeAnnotationTargetInfo,
    TypeAnnotationTypePath, TypeAnnotationTypePathSegment,
};
use ironjvm_specimpl::classfile::attrinfo::smtattr::{StackMapFrame, VerificationTypeInfo};
use ironjvm_specimpl::classfile::attrinfo::AttributeInfoType;
use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::{AttributeInfo, ClassFile, CpInfo, FieldInfo, MethodInfo};

use crate::error::{ParseError, ParseResult};

mod error;

pub struct ClassFileParser {
    pub classfile: File,
}

impl ClassFileParser {
    pub fn new(classfile: File) -> Self {
        Self { classfile }
    }

    pub fn parse(mut self) -> ParseResult<ClassFile> {
        let magic = self.parse_magic()?;
        let minor_version = self.next_u2()?;
        let major_version = self.next_u2()?;
        let constant_pool_count = self.next_u2()?;
        let constant_pool = self.parse_constant_pool(constant_pool_count)?;
        let access_flags = self.next_u2()?;
        let this_class = self.next_u2()?;
        let super_class = self.next_u2()?;
        let interfaces_count = self.next_u2()?;
        let interfaces = self.parse_interfaces(interfaces_count)?;
        let fields_count = self.next_u2()?;
        let fields = self.parse_fields(fields_count, &constant_pool)?;
        let methods_count = self.next_u2()?;
        let methods = self.parse_methods(methods_count, &constant_pool)?;
        let attributes_count = self.next_u2()?;
        let attributes = self.parse_attributes(attributes_count, &constant_pool)?;

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
    }

    fn next_u1(&mut self) -> ParseResult<u8> {
        self.classfile
            .read_u8()
            .map_err(|src| ParseError::IoError { src })
    }

    fn next_u2(&mut self) -> ParseResult<u16> {
        self.classfile
            .read_u16::<BigEndian>()
            .map_err(|src| ParseError::IoError { src })
    }

    fn next_u4(&mut self) -> ParseResult<u32> {
        self.classfile
            .read_u32::<BigEndian>()
            .map_err(|src| ParseError::IoError { src })
    }

    fn parse_magic(&mut self) -> ParseResult<u32> {
        self.next_u4().and_then(|magic| {
            if magic == 0xCAFEBABE {
                Ok(magic)
            } else {
                Err(ParseError::InvalidMagic)
            }
        })
    }

    fn parse_constant_pool(&mut self, count: u16) -> ParseResult<Vec<CpInfo>> {
        let capacity = (count - 1) as usize;
        let mut pool = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            let tag = self.next_u1()?;
            let info = match tag {
                1 => {
                    let length = self.next_u2()?;
                    let mut bytes = vec![0; length as usize];
                    self.classfile.read_exact(bytes.as_mut_slice());

                    CpInfoType::ConstantUtf8 { length, bytes }
                }
                3 => {
                    let bytes = self.next_u4()?;

                    CpInfoType::ConstantInteger { bytes }
                }
                4 => {
                    let bytes = self.next_u4()?;

                    CpInfoType::ConstantFloat { bytes }
                }
                5 => {
                    let high_bytes = self.next_u4()?;
                    let low_bytes = self.next_u4()?;

                    CpInfoType::ConstantLong {
                        high_bytes,
                        low_bytes,
                    }
                }
                6 => {
                    let high_bytes = self.next_u4()?;
                    let low_bytes = self.next_u4()?;

                    CpInfoType::ConstantDouble {
                        high_bytes,
                        low_bytes,
                    }
                }
                7 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantClass { name_index }
                }
                8 => {
                    let string_index = self.next_u2()?;

                    CpInfoType::ConstantString { string_index }
                }
                9 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantFieldRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                10 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantMethodRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                11 => {
                    let class_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantInterfaceMethodRef {
                        class_index,
                        name_and_type_index,
                    }
                }
                12 => {
                    let name_index = self.next_u2()?;
                    let descriptor_index = self.next_u2()?;

                    CpInfoType::ConstantNameAndType {
                        name_index,
                        descriptor_index,
                    }
                }
                15 => {
                    let reference_kind = self.next_u1()?;
                    let reference_index = self.next_u2()?;

                    CpInfoType::ConstantMethodHandle {
                        reference_kind,
                        reference_index,
                    }
                }
                16 => {
                    let descriptor_index = self.next_u2()?;

                    CpInfoType::ConstantMethodType { descriptor_index }
                }
                17 => {
                    let bootstrap_method_attr_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }
                }
                18 => {
                    let bootstrap_method_attr_index = self.next_u2()?;
                    let name_and_type_index = self.next_u2()?;

                    CpInfoType::ConstantInvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }
                }
                19 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantModule { name_index }
                }
                20 => {
                    let name_index = self.next_u2()?;

                    CpInfoType::ConstantPackage { name_index }
                }
                _ => unreachable!(),
            };

            pool.push(CpInfo { tag, info });
        }

        Ok(pool)
    }

    fn parse_interfaces(&mut self, count: u16) -> ParseResult<Vec<u16>> {
        let mut vec = vec![0; count as usize];
        self.classfile
            .read_u16_into::<BigEndian>(vec.as_mut_slice())?;

        Ok(vec)
    }

    fn parse_fields(
        &mut self,
        count: u16,
        constant_pool: &[CpInfo],
    ) -> ParseResult<Vec<FieldInfo>> {
        let mut vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let access_flags = self.next_u2()?;
            let name_index = self.next_u2()?;
            let descriptor_index = self.next_u2()?;
            let attributes_count = self.next_u2()?;
            let attributes = self.parse_attributes(attributes_count, constant_pool)?;

            vec.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(vec)
    }

    fn parse_attributes(
        &mut self,
        count: u16,
        constant_pool: &[CpInfo],
    ) -> ParseResult<Vec<AttributeInfo>> {
        let mut vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let attribute_name_index = self.next_u2()?;
            let attribute_length = self.next_u4()?;

            let name_cp_info = &constant_pool[attribute_name_index as usize].info;
            let CpInfoType::ConstantUtf8 { bytes, .. } = name_cp_info else {
                unreachable!()
            };

            let string = unsafe {
                // FIXME: JVM spec specifies these are modified UTF8
                String::from_utf8_unchecked(bytes.clone())
            };
            let info = match &*string {
                "ConstantValue" => {
                    let constantvalue_index = self.next_u2()?;

                    AttributeInfoType::ConstantValueAttribute {
                        constantvalue_index,
                    }
                }
                "Code" => {
                    let max_stack = self.next_u2()?;
                    let max_locals = self.next_u2()?;
                    let code_length = self.next_u4()?;

                    let mut code = vec![0; code_length as usize];
                    self.classfile.read_exact(code.as_mut_slice())?;

                    let exception_table_length = self.next_u2()?;
                    let exception_table = self.parse_exception_table(exception_table_length)?;

                    let attributes_count = self.next_u2()?;
                    let attributes = self.parse_attributes(attributes_count, constant_pool)?;

                    AttributeInfoType::CodeAttribute {
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exception_table_length,
                        exception_table,
                        attributes_count,
                        attributes,
                    }
                }
                "StackMapTable" => {
                    let number_of_entries = self.next_u2()?;
                    let mut stack_map_table = Vec::with_capacity(number_of_entries as usize);
                    for _ in 0..number_of_entries {
                        stack_map_table.push(self.parse_stack_map_frame()?);
                    }

                    AttributeInfoType::StackMapTableAttribute {
                        number_of_entries,
                        stack_map_table,
                    }
                }
                "Exceptions" => {
                    let number_of_exceptions = self.next_u2()?;
                    let mut exception_index_table = vec![0; number_of_exceptions as usize];
                    self.classfile
                        .read_u16_into::<BigEndian>(exception_index_table.as_mut_slice())?;

                    AttributeInfoType::ExceptionsAttribute {
                        number_of_exceptions,
                        exception_index_table,
                    }
                }
                "InnerClasses" => {
                    let number_of_classes = self.next_u2()?;
                    let mut classes = Vec::with_capacity(number_of_classes as usize);
                    for _ in 0..number_of_classes {
                        let inner_class_info_index = self.next_u2()?;
                        let outer_class_info_index = self.next_u2()?;
                        let inner_name_index = self.next_u2()?;
                        let inner_class_access_flags = self.next_u2()?;

                        classes.push(InnerClass {
                            inner_class_info_index,
                            outer_class_info_index,
                            inner_name_index,
                            inner_class_access_flags,
                        });
                    }

                    AttributeInfoType::InnerClassesAttribute {
                        number_of_classes,
                        classes,
                    }
                }
                "EnclosingMethod" => {
                    let class_index = self.next_u2()?;
                    let method_index = self.next_u2()?;

                    AttributeInfoType::EnclosingMethodAttribute {
                        class_index,
                        method_index,
                    }
                }
                "Synthetic" => AttributeInfoType::SyntheticAttribute,
                "Signature" => {
                    let signature_index = self.next_u2()?;

                    AttributeInfoType::SignatureAttribute { signature_index }
                }
                "SourceFile" => {
                    let sourcefile_index = self.next_u2()?;

                    AttributeInfoType::SourceFileAttribute { sourcefile_index }
                }
                "SourceDebugExtension" => {
                    let mut debug_extension = vec![0; attribute_length as usize];
                    self.classfile.read_exact(debug_extension.as_mut_slice())?;

                    AttributeInfoType::SourceDebugExtensionAttribute { debug_extension }
                }
                "LineNumberTable" => {
                    let line_number_table_length = self.next_u2()?;
                    let mut line_number_table =
                        Vec::with_capacity(line_number_table_length as usize);
                    for _ in 0..line_number_table_length {
                        let start_pc = self.next_u2()?;
                        let line_number = self.next_u2()?;

                        line_number_table.push(LineNumber {
                            start_pc,
                            line_number,
                        });
                    }

                    AttributeInfoType::LineNumberTableAttribute {
                        line_number_table_length,
                        line_number_table,
                    }
                }
                "LocalVariableTable" => {
                    let local_variable_table_length = self.next_u2()?;
                    let mut local_variable_table =
                        Vec::with_capacity(local_variable_table_length as usize);
                    for _ in 0..local_variable_table_length {
                        let start_pc = self.next_u2()?;
                        let length = self.next_u2()?;
                        let name_index = self.next_u2()?;
                        let descriptor_index = self.next_u2()?;
                        let index = self.next_u2()?;

                        local_variable_table.push(LocalVariable {
                            start_pc,
                            length,
                            name_index,
                            descriptor_index,
                            index,
                        });
                    }

                    AttributeInfoType::LocalVariableTableAttribute {
                        local_variable_table_length,
                        local_variable_table,
                    }
                }
                "LocalVariableTypeTable" => {
                    let local_variable_type_table_length = self.next_u2()?;
                    let mut local_variable_type_table =
                        Vec::with_capacity(local_variable_type_table_length as usize);
                    for _ in 0..local_variable_type_table_length {
                        let start_pc = self.next_u2()?;
                        let length = self.next_u2()?;
                        let name_index = self.next_u2()?;
                        let signature_index = self.next_u2()?;
                        let index = self.next_u2()?;

                        local_variable_type_table.push(LocalVariableType {
                            start_pc,
                            length,
                            name_index,
                            signature_index,
                            index,
                        });
                    }

                    AttributeInfoType::LocalVariableTypeTableAttribute {
                        local_variable_type_table_length,
                        local_variable_type_table,
                    }
                }
                "Deprecated" => AttributeInfoType::DeprecatedAttribute,
                "RuntimeVisibleAnnotations" => {
                    let num_annotations = self.next_u2()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(self.parse_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeInvisibleAnnotations" => {
                    let num_annotations = self.next_u2()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(self.parse_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeVisibleParameterAnnotations" => {
                    let num_parameters = self.next_u2()?;
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    for _ in 0..num_parameters {
                        parameter_annotations.push(self.parse_parameter_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleParameterAnnotationsAttribute {
                        num_parameters,
                        parameter_annotations,
                    }
                }
                "RuntimeInvisibleParameterAnnotations" => {
                    let num_parameters = self.next_u2()?;
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    for _ in 0..num_parameters {
                        parameter_annotations.push(self.parse_parameter_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleParameterAnnotationsAttribute {
                        num_parameters,
                        parameter_annotations,
                    }
                }
                "RuntimeVisibleTypeAnnotations" => {
                    let num_annotations = self.next_u2()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(self.parse_type_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleTypeAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeInvisibleTypeAnnotations" => {
                    let num_annotations = self.next_u2()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(self.parse_type_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleTypeAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "AnnotationDefault" => {
                    let default_value = self.parse_element_value()?;

                    AttributeInfoType::AnnotationDefaultAttribute { default_value }
                }
                "BootstrapMethods" => {
                    let num_bootstrap_methods = self.next_u2()?;
                    let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
                    for _ in 0..num_bootstrap_methods {
                        let bootstrap_method_ref = self.next_u2()?;
                        let num_bootstrap_arguments = self.next_u2()?;
                        let mut bootstrap_arguments = vec![0; num_bootstrap_arguments as usize];
                        self.classfile
                            .read_u16_into::<BigEndian>(bootstrap_arguments.as_mut_slice());

                        bootstrap_methods.push(BootstrapMethod {
                            bootstrap_method_ref,
                            num_bootstrap_arguments,
                            bootstrap_arguments,
                        });
                    }

                    AttributeInfoType::BootstrapMethodsAttribute {
                        num_bootstrap_methods,
                        bootstrap_methods,
                    }
                }
                "MethodParameters" => {
                    let parameters_count = self.next_u1()?;
                    let mut parameters = Vec::with_capacity(parameters_count as usize);
                    for _ in 0..parameters_count {
                        let name_index = self.next_u2()?;
                        let access_flags = self.next_u2()?;

                        parameters.push(MethodParameter {
                            name_index,
                            access_flags,
                        });
                    }

                    AttributeInfoType::MethodParametersAttribute {
                        parameters_count,
                        parameters,
                    }
                }
                "Module" => {
                    let module_name_index = self.next_u2()?;
                    let module_flags = self.next_u2()?;
                    let module_version_index = self.next_u2()?;

                    let requires_count = self.next_u2()?;
                    let mut requires = Vec::with_capacity(requires_count as usize);
                    for _ in 0..requires_count {
                        let requires_index = self.next_u2()?;
                        let requires_flags = self.next_u2()?;
                        let requires_version_index = self.next_u2()?;

                        requires.push(ModuleRequire {
                            requires_index,
                            requires_flags,
                            requires_version_index,
                        });
                    }

                    let exports_count = self.next_u2()?;
                    let mut exports = Vec::with_capacity(exports_count as usize);
                    for _ in 0..exports_count {
                        let exports_index = self.next_u2()?;
                        let exports_flags = self.next_u2()?;
                        let exports_to_count = self.next_u2()?;
                        let mut exports_to_index = vec![0; exports_to_count as usize];
                        self.classfile
                            .read_u16_into::<BigEndian>(exports_to_index.as_mut_slice());

                        exports.push(ModuleExport {
                            exports_index,
                            exports_flags,
                            exports_to_count,
                            exports_to_index,
                        });
                    }

                    let opens_count = self.next_u2()?;
                    let mut opens = Vec::with_capacity(opens_count as usize);
                    for _ in 0..exports_count {
                        let opens_index = self.next_u2()?;
                        let opens_flags = self.next_u2()?;
                        let opens_to_count = self.next_u2()?;
                        let mut opens_to_index = vec![0; opens_to_count as usize];
                        self.classfile
                            .read_u16_into::<BigEndian>(opens_to_index.as_mut_slice());

                        opens.push(ModuleOpen {
                            opens_index,
                            opens_flags,
                            opens_to_count,
                            opens_to_index,
                        });
                    }

                    let uses_count = self.next_u2()?;
                    let mut uses_index = vec![0; uses_count as usize];
                    self.classfile
                        .read_u16_into::<BigEndian>(uses_index.as_mut_slice());

                    let provides_count = self.next_u2()?;
                    let mut provides = Vec::with_capacity(provides_count as usize);
                    for _ in 0..provides_count {
                        let provides_index = self.next_u2()?;
                        let provides_with_count = self.next_u2()?;
                        let mut provides_with_index = vec![0; provides_with_count as usize];
                        self.classfile
                            .read_u16_into(provides_with_index.as_mut_slice());

                        provides.push(ModuleProvide {
                            provides_index,
                            provides_with_count,
                            provides_with_index,
                        });
                    }

                    AttributeInfoType::ModuleAttribute {
                        module_name_index,
                        module_flags,
                        module_version_index,
                        requires_count,
                        requires,
                        exports_count,
                        exports,
                        opens_count,
                        opens,
                        uses_count,
                        uses_index,
                        provides_count,
                        provides,
                    }
                }
                "ModulePackages" => {
                    let package_count = self.next_u2()?;
                    let mut package_index = vec![0; package_count as usize];
                    self.classfile.read_u16_into(package_index.as_mut_slice());

                    AttributeInfoType::ModulePackagesAttribute {
                        package_count,
                        package_index,
                    }
                }
                "ModuleMainClass" => {
                    let main_class_index = self.next_u2()?;

                    AttributeInfoType::ModuleMainClassAttribute { main_class_index }
                }
                "NestHost" => {
                    let host_class_index = self.next_u2()?;

                    AttributeInfoType::NestHostAttribute { host_class_index }
                }
                "NestMembers" => {
                    let number_of_classes = self.next_u2()?;
                    let mut classes = vec![0; number_of_classes as usize];
                    self.classfile.read_u16_into(classes.as_mut_slice());

                    AttributeInfoType::NestMembersAttribute {
                        number_of_classes,
                        classes,
                    }
                }
                "Record" => {
                    let components_count = self.next_u2()?;
                    let mut components = Vec::with_capacity(components_count as usize);
                    for _ in 0..components_count {
                        let name_index = self.next_u2()?;
                        let descriptor_index = self.next_u2()?;
                        let attributes_count = self.next_u2()?;
                        let attributes = self.parse_attributes(attributes_count, constant_pool)?;

                        components.push(RecordComponentInfo {
                            name_index,
                            descriptor_index,
                            attributes_count,
                            attributes,
                        });
                    }
                }
                "PermittedSubclasses" => {
                    let number_of_classes = self.next_u2()?;
                    let mut classes = vec![0; number_of_classes as usize];
                    self.classfile.read_u16_into(classes.as_mut_slice());

                    AttributeInfoType::PermittedSubclassesAttribute {
                        number_of_classes,
                        classes,
                    }
                }
                _ => unreachable!(),
            };

            vec.push(AttributeInfo {
                attribute_name_index,
                attribute_length,
                info,
            });
        }

        Ok(vec)
    }

    fn parse_exception_table(
        &mut self,
        count: u16,
    ) -> ParseResult<Vec<CodeAttributeExceptionTableEntry>> {
        let mut vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let start_pc = self.next_u2()?;
            let end_pc = self.next_u2()?;
            let handler_pc = self.next_u2()?;
            let catch_type = self.next_u2()?;

            vec.push(CodeAttributeExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }

        Ok(vec)
    }

    fn parse_stack_map_frame(&mut self) -> ParseResult<StackMapFrame> {
        let frame_type = self.next_u1()?;

        Ok(match frame_type {
            0..=63 => StackMapFrame::SameFrame { frame_type },
            64..=127 => {
                let stack = self.parse_verification_type_info()?;

                StackMapFrame::SameLocals1StackItemFrame { frame_type, stack }
            }
            247 => {
                let offset_delta = self.next_u2()?;
                let stack = self.parse_verification_type_info()?;

                StackMapFrame::SameLocals1StackItemFrameExtended {
                    frame_type,
                    offset_delta,
                    stack,
                }
            }
            248..=250 => {
                let offset_delta = self.next_u2()?;

                StackMapFrame::ChopFrame {
                    frame_type,
                    offset_delta,
                }
            }
            251 => {
                let offset_delta = self.next_u2()?;

                StackMapFrame::SameFrameExtended {
                    frame_type,
                    offset_delta,
                }
            }
            252..=254 => {
                let offset_delta = self.next_u2()?;

                let locals_length = frame_type - 251;
                let mut locals = Vec::with_capacity(locals_length as usize);
                for _ in 0..locals_length {
                    locals.push(self.parse_verification_type_info()?);
                }

                StackMapFrame::AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                }
            }
            255 => {
                let offset_delta = self.next_u2()?;

                let number_of_locals = self.next_u2()?;
                let mut locals = Vec::with_capacity(number_of_locals as usize);
                for _ in 0..number_of_locals {
                    locals.push(self.parse_verification_type_info()?);
                }

                let number_of_stack_items = self.next_u2()?;
                let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                for _ in 0..number_of_stack_items {
                    stack.push(self.parse_verification_type_info()?);
                }

                StackMapFrame::FullFrame {
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            }
            _ => unreachable!(),
        })
    }

    fn parse_verification_type_info(&mut self) -> ParseResult<VerificationTypeInfo> {
        let tag = self.next_u1()?;

        Ok(match tag {
            0 => VerificationTypeInfo::TopVariableInfo { tag },
            1 => VerificationTypeInfo::IntegerVariableInfo { tag },
            2 => VerificationTypeInfo::FloatVariableInfo { tag },
            3 => VerificationTypeInfo::DoubleVariableInfo { tag },
            4 => VerificationTypeInfo::LongVariableInfo { tag },
            5 => VerificationTypeInfo::NullVariableInfo { tag },
            6 => VerificationTypeInfo::UninitializedThisVariableInfo { tag },
            7 => {
                let cpool_index = self.next_u2()?;

                VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index }
            }
            8 => {
                let offset = self.next_u2()?;

                VerificationTypeInfo::UninitializedVariableInfo { tag, offset }
            }
            _ => unreachable!(),
        })
    }

    fn parse_annotation(&mut self) -> ParseResult<Annotation> {
        let type_index = self.next_u2()?;

        let num_element_value_pairs = self.next_u2()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let element_name_index = self.next_u2()?;
            let value = self.parse_element_value()?;

            element_value_pairs.push(ElementValuePair {
                element_name_index,
                value,
            });
        }

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn parse_element_value(&mut self) -> ParseResult<ElementValue> {
        let tag = self.next_u1()?;
        let value = match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                let const_value_index = self.next_u2()?;

                ElementValueValue::ConstValueIndex { const_value_index }
            }
            'e' => {
                let type_name_index = self.next_u2()?;
                let const_name_index = self.next_u2()?;

                ElementValueValue::EnumConstValue {
                    type_name_index,
                    const_name_index,
                }
            }
            'c' => {
                let class_info_index = self.next_u2()?;

                ElementValueValue::ClassInfoIndex { class_info_index }
            }
            '@' => {
                let annotation_value = self.parse_annotation()?;

                ElementValueValue::AnnotationValue { annotation_value }
            }
            '[' => {
                let num_values = self.next_u2()?;
                let mut values = Vec::with_capacity(num_values as usize);
                for _ in 0..num_values {
                    values.push(self.parse_element_value()?);
                }

                ElementValueValue::ArrayValue { num_values, values }
            }
            _ => unreachable!(),
        };

        Ok(ElementValue { tag, value })
    }

    fn parse_parameter_annotation(&mut self) -> ParseResult<ParameterAnnotation> {
        let num_annotations = self.next_u2()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            annotations.push(self.parse_annotation()?);
        }

        Ok(ParameterAnnotation {
            num_annotations,
            annotations,
        })
    }

    fn parse_type_annotation(&mut self) -> ParseResult<TypeAnnotation> {
        let target_type = self.next_u1()?;
        let target_info = match target_type {
            0x00 | 0x01 => {
                let type_parameter_index = self.next_u1()?;

                TypeAnnotationTargetInfo::TypeParameterTarget {
                    type_parameter_index,
                }
            }
            0x10 => {
                let supertype_index = self.next_u2()?;

                TypeAnnotationTargetInfo::SupertypeTarget { supertype_index }
            }
            0x11 | 0x12 => {
                let type_parameter_index = self.next_u1()?;
                let bound_index = self.next_u1()?;

                TypeAnnotationTargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                }
            }
            0x13..=0x15 => TypeAnnotationTargetInfo::EmptyTarget,
            0x16 => {
                let formal_parameter_index = self.next_u1()?;

                TypeAnnotationTargetInfo::FormalParameterTarget {
                    formal_parameter_index,
                }
            }
            0x17 => {
                let throws_type_index = self.next_u2()?;

                TypeAnnotationTargetInfo::ThrowsTarget { throws_type_index }
            }
            0x40 | 0x41 => {
                let table_length = self.next_u2()?;
                let mut table = Vec::with_capacity(table_length as usize);

                for _ in 0..table_length {
                    let start_pc = self.next_u2()?;
                    let length = self.next_u2()?;
                    let index = self.next_u2()?;

                    table.push(TypeAnnotationLocalVarTargetTableEntry {
                        start_pc,
                        length,
                        index,
                    });
                }

                TypeAnnotationTargetInfo::LocalVarTarget {
                    table_length,
                    table,
                }
            }
            0x42 => {
                let catch_index = self.next_u2()?;

                TypeAnnotationTargetInfo::CatchTarget { catch_index }
            }
            0x43..=0x46 => {
                let offset = self.next_u2()?;

                TypeAnnotationTargetInfo::OffsetTarget { offset }
            }
            0x47..=0x4B => {
                let offset = self.next_u2()?;
                let type_argument_index = self.next_u1()?;

                TypeAnnotationTargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                }
            }
            _ => unreachable!(),
        };

        let path_length = self.next_u1()?;
        let mut path = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            let type_path_kind = self.next_u1()?;
            let type_argument_index = self.next_u1()?;
            let segment = TypeAnnotationTypePathSegment {
                type_path_kind,
                type_argument_index,
            };

            path.push(segment);
        }
        let target_path = TypeAnnotationTypePath { path_length, path };

        let type_index = self.next_u2()?;

        let num_element_value_pairs = self.next_u2()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let element_name_index = self.next_u2()?;
            let value = self.parse_element_value()?;

            element_value_pairs.push(ElementValuePair {
                element_name_index,
                value,
            });
        }

        Ok(TypeAnnotation {
            target_type,
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn parse_methods(
        &mut self,
        count: u16,
        constant_pool: &[CpInfo],
    ) -> ParseResult<Vec<MethodInfo>> {
        let mut vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let access_flags = self.next_u2()?;
            let name_index = self.next_u2()?;
            let descriptor_index = self.next_u2()?;
            let attributes_count = self.next_u2()?;
            let attributes = self.parse_attributes(attributes_count, constant_pool)?;

            vec.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(vec)
    }
}
