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

use std::str;

use ironjvm_javautil::be::JavaBeUtil;
use ironjvm_javautil::jstr::JStr;
use ironjvm_specimpl::classfile::attrinfo::bmattr::BootstrapMethod;
use ironjvm_specimpl::classfile::attrinfo::cattr::CodeAttributeExceptionTableEntry;
use ironjvm_specimpl::classfile::attrinfo::icattr::InnerClass;
use ironjvm_specimpl::classfile::attrinfo::lntattr::LineNumber;
use ironjvm_specimpl::classfile::attrinfo::lvtattr::LocalVariable;
use ironjvm_specimpl::classfile::attrinfo::lvttattr::LocalVariableType;
use ironjvm_specimpl::classfile::attrinfo::mattr::ModuleExport;
use ironjvm_specimpl::classfile::attrinfo::mattr::ModuleOpen;
use ironjvm_specimpl::classfile::attrinfo::mattr::ModuleProvide;
use ironjvm_specimpl::classfile::attrinfo::mattr::ModuleRequire;
use ironjvm_specimpl::classfile::attrinfo::mpattr::MethodParameter;
use ironjvm_specimpl::classfile::attrinfo::rattr::RecordComponentInfo;
use ironjvm_specimpl::classfile::attrinfo::rvanriaattr::Annotation;
use ironjvm_specimpl::classfile::attrinfo::rvanriaattr::ElementValue;
use ironjvm_specimpl::classfile::attrinfo::rvanriaattr::ElementValuePair;
use ironjvm_specimpl::classfile::attrinfo::rvanriaattr::ElementValueValue;
use ironjvm_specimpl::classfile::attrinfo::rvpaattr::ParameterAnnotation;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::TypeAnnotation;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::TypeAnnotationLocalVarTargetTableEntry;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::TypeAnnotationTargetInfo;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::TypeAnnotationTypePath;
use ironjvm_specimpl::classfile::attrinfo::rvtnritaattr::TypeAnnotationTypePathSegment;
use ironjvm_specimpl::classfile::attrinfo::smtattr::StackMapFrame;
use ironjvm_specimpl::classfile::attrinfo::smtattr::VerificationTypeInfo;
use ironjvm_specimpl::classfile::attrinfo::AttributeInfoType;
use ironjvm_specimpl::classfile::cpinfo::CpInfoType;
use ironjvm_specimpl::classfile::AttributeInfo;
use ironjvm_specimpl::classfile::ClassFile;
use ironjvm_specimpl::classfile::CpInfo;
use ironjvm_specimpl::classfile::FieldInfo;
use ironjvm_specimpl::classfile::MethodInfo;

use crate::error::ParseError;
use crate::error::ParseResult;

mod error;

pub struct ClassFileParser<'clazz> {
    classfile: &'clazz [u8],
}

impl<'clazz> ClassFileParser<'clazz> {
    pub fn new(classfile: &'clazz [u8]) -> Self {
        Self { classfile }
    }

    pub fn parse(&mut self) -> ParseResult<ClassFile<'clazz>> {
        let magic = self.parse_magic()?;
        let minor_version = self.next_u2();
        let major_version = self.next_u2();
        let constant_pool_count = self.next_u2();
        let constant_pool = self.parse_constant_pool(constant_pool_count)?;
        let access_flags = self.next_u2();
        let this_class = self.next_u2();
        let super_class = self.next_u2();
        let interfaces_count = self.next_u2();
        let interfaces = self.parse_interfaces(interfaces_count);
        let fields_count = self.next_u2();
        let fields = self.parse_fields(fields_count, &constant_pool)?;
        let methods_count = self.next_u2();
        let methods = self.parse_methods(methods_count, &constant_pool)?;
        let attributes_count = self.next_u2();
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

    // Credit: code referenced from https://github.com/TapVM/Aftermath
    fn next_u1(&mut self) -> u8 {
        let ret = self.classfile[0];
        self.classfile = &self.classfile[1..];

        ret
    }

    // Credit: code referenced from https://github.com/TapVM/Aftermath
    fn next_u1_many(&mut self, len: usize) -> &'clazz [u8] {
        let output = &self.classfile[0..len];
        self.classfile = &self.classfile[len..];

        output
    }

    // Credit: code referenced from https://github.com/TapVM/Aftermath
    fn next_u2(&mut self) -> u16 {
        [self.next_u1(), self.next_u1()].to_u16()
    }

    // Credit: code referenced from https://github.com/TapVM/Aftermath
    fn next_u2_many(&mut self, length: usize) -> &'clazz [[u8; 2]] {
        unsafe {
            core::slice::from_raw_parts(self.next_u1_many(length * 2).as_ptr().cast(), length)
        }
    }

    // Credit: code referenced from https://github.com/TapVM/Aftermath
    fn next_u4(&mut self) -> u32 {
        u32::from_be_bytes(self.next_u1_many(4).try_into().unwrap())
    }

    fn parse_magic(&mut self) -> ParseResult<u32> {
        let output = self.next_u4();

        if output != 0xCAFEBABE {
            return Err(ParseError::InvalidMagic);
        }

        Ok(output)
    }

    fn parse_constant_pool(&mut self, count: u16) -> ParseResult<Vec<CpInfo<'clazz>>> {
        let mut pool = Vec::with_capacity(count as usize - 1);

        while pool.len() + 1 < count as usize {
            let tag = self.next_u1();
            let info = match tag {
                1 => {
                    let length = self.next_u2();

                    CpInfoType::ConstantUtf8 {
                        length,
                        bytes: self.next_u1_many(length as usize),
                    }
                }
                3 => CpInfoType::ConstantInteger {
                    bytes: self.next_u4(),
                },
                4 => CpInfoType::ConstantFloat {
                    bytes: self.next_u4(),
                },
                5 => CpInfoType::ConstantLong {
                    high_bytes: self.next_u4(),
                    low_bytes: self.next_u4(),
                },
                6 => CpInfoType::ConstantDouble {
                    high_bytes: self.next_u4(),
                    low_bytes: self.next_u4(),
                },
                7 => CpInfoType::ConstantClass {
                    name_index: self.next_u2(),
                },
                8 => CpInfoType::ConstantString {
                    string_index: self.next_u2(),
                },
                9 => CpInfoType::ConstantFieldRef {
                    class_index: self.next_u2(),
                    name_and_type_index: self.next_u2(),
                },
                10 => CpInfoType::ConstantMethodRef {
                    class_index: self.next_u2(),
                    name_and_type_index: self.next_u2(),
                },
                11 => CpInfoType::ConstantInterfaceMethodRef {
                    class_index: self.next_u2(),
                    name_and_type_index: self.next_u2(),
                },
                12 => CpInfoType::ConstantNameAndType {
                    name_index: self.next_u2(),
                    descriptor_index: self.next_u2(),
                },
                15 => CpInfoType::ConstantMethodHandle {
                    reference_kind: self.next_u1(),
                    reference_index: self.next_u2(),
                },
                16 => CpInfoType::ConstantMethodType {
                    descriptor_index: self.next_u2(),
                },
                17 => CpInfoType::ConstantDynamic {
                    bootstrap_method_attr_index: self.next_u2(),
                    name_and_type_index: self.next_u2(),
                },
                18 => CpInfoType::ConstantInvokeDynamic {
                    bootstrap_method_attr_index: self.next_u2(),
                    name_and_type_index: self.next_u2(),
                },
                19 => CpInfoType::ConstantModule {
                    name_index: self.next_u2(),
                },
                20 => CpInfoType::ConstantPackage {
                    name_index: self.next_u2(),
                },
                _ => unreachable!(),
            };

            pool.push(CpInfo { tag, info });
        }

        Ok(pool)
    }

    fn parse_interfaces(&mut self, count: u16) -> &'clazz [[u8; 2]] {
        self.next_u2_many(count as usize)
    }

    fn parse_fields(
        &mut self,
        count: u16,
        constant_pool: &[CpInfo],
    ) -> ParseResult<Vec<FieldInfo<'clazz>>> {
        let mut vec = Vec::with_capacity(count as usize);

        while vec.len() < count as usize {
            let access_flags = self.next_u2();
            let name_index = self.next_u2();
            let descriptor_index = self.next_u2();
            let attributes_count = self.next_u2();

            vec.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes: self.parse_attributes(attributes_count, constant_pool)?,
            });
        }

        Ok(vec)
    }

    fn parse_attributes(
        &mut self,
        count: u16,
        constant_pool: &[CpInfo],
    ) -> ParseResult<Vec<AttributeInfo<'clazz>>> {
        let mut vec = Vec::with_capacity(count as usize);

        while vec.len() < count as usize {
            let attribute_name_index = self.next_u2();
            let attribute_length = self.next_u4();

            let opt = &constant_pool.get(attribute_name_index as usize - 1);

            if opt.is_none() {
                return Err(ParseError::InvalidConstantPoolIndex);
            };

            let name_cp_info = opt.unwrap();
            let CpInfoType::ConstantUtf8 { bytes, .. } = &name_cp_info.info else {
                unreachable!()
            };

            let string = unsafe { JStr::from_jutf8_unchecked(bytes) };
            let info = match **string {
                "ConstantValue" => AttributeInfoType::ConstantValueAttribute {
                    constantvalue_index: self.next_u2(),
                },
                "Code" => {
                    let max_stack = self.next_u2();
                    let max_locals = self.next_u2();
                    let code_length = self.next_u4();

                    let code = self.next_u1_many(code_length as usize);

                    let exception_table_length = self.next_u2();
                    let exception_table = self.parse_exception_table(exception_table_length)?;

                    let attributes_count = self.next_u2();

                    AttributeInfoType::CodeAttribute {
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exception_table_length,
                        exception_table,
                        attributes_count,
                        attributes: self.parse_attributes(attributes_count, constant_pool)?,
                    }
                }
                "StackMapTable" => {
                    let number_of_entries = self.next_u2();
                    let mut stack_map_table = Vec::with_capacity(number_of_entries as usize);
                    while stack_map_table.len() < number_of_entries as usize {
                        stack_map_table.push(self.parse_stack_map_frame()?);
                    }

                    AttributeInfoType::StackMapTableAttribute {
                        number_of_entries,
                        stack_map_table,
                    }
                }
                "Exceptions" => {
                    let number_of_exceptions = self.next_u2();

                    AttributeInfoType::ExceptionsAttribute {
                        number_of_exceptions,
                        exception_index_table: self.next_u2_many(number_of_exceptions as usize),
                    }
                }
                "InnerClasses" => {
                    let number_of_classes = self.next_u2();
                    let mut classes = Vec::with_capacity(number_of_classes as usize);
                    while classes.len() < number_of_classes as usize {
                        classes.push(InnerClass {
                            inner_class_info_index: self.next_u2(),
                            outer_class_info_index: self.next_u2(),
                            inner_name_index: self.next_u2(),
                            inner_class_access_flags: self.next_u2(),
                        });
                    }

                    AttributeInfoType::InnerClassesAttribute {
                        number_of_classes,
                        classes,
                    }
                }
                "EnclosingMethod" => AttributeInfoType::EnclosingMethodAttribute {
                    class_index: self.next_u2(),
                    method_index: self.next_u2(),
                },
                "Synthetic" => AttributeInfoType::SyntheticAttribute,
                "Signature" => AttributeInfoType::SignatureAttribute {
                    signature_index: self.next_u2(),
                },
                "SourceFile" => AttributeInfoType::SourceFileAttribute {
                    sourcefile_index: self.next_u2(),
                },
                "SourceDebugExtension" => AttributeInfoType::SourceDebugExtensionAttribute {
                    debug_extension: self.next_u1_many(attribute_length as usize),
                },
                "LineNumberTable" => {
                    let line_number_table_length = self.next_u2();
                    let mut line_number_table =
                        Vec::with_capacity(line_number_table_length as usize);
                    while line_number_table.len() < line_number_table_length as usize {
                        line_number_table.push(LineNumber {
                            start_pc: self.next_u2(),
                            line_number: self.next_u2(),
                        });
                    }

                    AttributeInfoType::LineNumberTableAttribute {
                        line_number_table_length,
                        line_number_table,
                    }
                }
                "LocalVariableTable" => {
                    let local_variable_table_length = self.next_u2();
                    let mut local_variable_table =
                        Vec::with_capacity(local_variable_table_length as usize);
                    while local_variable_table.len() < local_variable_table_length as usize {
                        local_variable_table.push(LocalVariable {
                            start_pc: self.next_u2(),
                            length: self.next_u2(),
                            name_index: self.next_u2(),
                            descriptor_index: self.next_u2(),
                            index: self.next_u2(),
                        });
                    }

                    AttributeInfoType::LocalVariableTableAttribute {
                        local_variable_table_length,
                        local_variable_table,
                    }
                }
                "LocalVariableTypeTable" => {
                    let local_variable_type_table_length = self.next_u2();
                    let mut local_variable_type_table =
                        Vec::with_capacity(local_variable_type_table_length as usize);
                    while local_variable_type_table.len()
                        < local_variable_type_table_length as usize
                    {
                        local_variable_type_table.push(LocalVariableType {
                            start_pc: self.next_u2(),
                            length: self.next_u2(),
                            name_index: self.next_u2(),
                            signature_index: self.next_u2(),
                            index: self.next_u2(),
                        });
                    }

                    AttributeInfoType::LocalVariableTypeTableAttribute {
                        local_variable_type_table_length,
                        local_variable_type_table,
                    }
                }
                "Deprecated" => AttributeInfoType::DeprecatedAttribute,
                "RuntimeVisibleAnnotations" => {
                    let num_annotations = self.next_u2();
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.len() < num_annotations as usize {
                        annotations.push(self.parse_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeInvisibleAnnotations" => {
                    let num_annotations = self.next_u2();
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.len() < num_annotations as usize {
                        annotations.push(self.parse_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeVisibleParameterAnnotations" => {
                    let num_parameters = self.next_u2();
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    while parameter_annotations.len() < num_parameters as usize {
                        parameter_annotations.push(self.parse_parameter_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleParameterAnnotationsAttribute {
                        num_parameters,
                        parameter_annotations,
                    }
                }
                "RuntimeInvisibleParameterAnnotations" => {
                    let num_parameters = self.next_u2();
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    while parameter_annotations.len() < num_parameters as usize {
                        parameter_annotations.push(self.parse_parameter_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleParameterAnnotationsAttribute {
                        num_parameters,
                        parameter_annotations,
                    }
                }
                "RuntimeVisibleTypeAnnotations" => {
                    let num_annotations = self.next_u2();
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.len() < num_annotations as usize {
                        annotations.push(self.parse_type_annotation()?);
                    }

                    AttributeInfoType::RuntimeVisibleTypeAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "RuntimeInvisibleTypeAnnotations" => {
                    let num_annotations = self.next_u2();
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.len() < num_annotations as usize {
                        annotations.push(self.parse_type_annotation()?);
                    }

                    AttributeInfoType::RuntimeInvisibleTypeAnnotationsAttribute {
                        num_annotations,
                        annotations,
                    }
                }
                "AnnotationDefault" => AttributeInfoType::AnnotationDefaultAttribute {
                    default_value: self.parse_element_value()?,
                },
                "BootstrapMethods" => {
                    let num_bootstrap_methods = self.next_u2();
                    let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
                    while bootstrap_methods.len() < num_bootstrap_methods as usize {
                        let num_bootstrap_arguments = self.next_u2();

                        bootstrap_methods.push(BootstrapMethod {
                            bootstrap_method_ref: self.next_u2(),
                            num_bootstrap_arguments,
                            bootstrap_arguments: self
                                .next_u2_many(num_bootstrap_arguments as usize),
                        });
                    }

                    AttributeInfoType::BootstrapMethodsAttribute {
                        num_bootstrap_methods,
                        bootstrap_methods,
                    }
                }
                "MethodParameters" => {
                    let parameters_count = self.next_u1();
                    let mut parameters = Vec::with_capacity(parameters_count as usize);
                    while parameters.len() < parameters_count as usize {
                        parameters.push(MethodParameter {
                            name_index: self.next_u2(),
                            access_flags: self.next_u2(),
                        });
                    }

                    AttributeInfoType::MethodParametersAttribute {
                        parameters_count,
                        parameters,
                    }
                }
                "Module" => {
                    let module_name_index = self.next_u2();
                    let module_flags = self.next_u2();
                    let module_version_index = self.next_u2();

                    let requires_count = self.next_u2();
                    let mut requires = Vec::with_capacity(requires_count as usize);
                    while requires.len() < requires_count as usize {
                        requires.push(ModuleRequire {
                            requires_index: self.next_u2(),
                            requires_flags: self.next_u2(),
                            requires_version_index: self.next_u2(),
                        });
                    }

                    let exports_count = self.next_u2();
                    let mut exports = Vec::with_capacity(exports_count as usize);
                    while exports.len() < exports_count as usize {
                        let exports_index = self.next_u2();
                        let exports_flags = self.next_u2();
                        let exports_to_count = self.next_u2();

                        exports.push(ModuleExport {
                            exports_index,
                            exports_flags,
                            exports_to_count,
                            exports_to_index: self.next_u2_many(exports_to_count as usize),
                        });
                    }

                    let opens_count = self.next_u2();
                    let mut opens = Vec::with_capacity(opens_count as usize);
                    while opens.len() < opens_count as usize {
                        let opens_index = self.next_u2();
                        let opens_flags = self.next_u2();
                        let opens_to_count = self.next_u2();

                        opens.push(ModuleOpen {
                            opens_index,
                            opens_flags,
                            opens_to_count,
                            opens_to_index: self.next_u2_many(opens_to_count as usize),
                        });
                    }

                    let uses_count = self.next_u2();
                    let uses_index = self.next_u2_many(uses_count as usize);

                    let provides_count = self.next_u2();
                    let mut provides = Vec::with_capacity(provides_count as usize);
                    while provides.len() < provides_count as usize {
                        let provides_index = self.next_u2();
                        let provides_with_count = self.next_u2();

                        provides.push(ModuleProvide {
                            provides_index,
                            provides_with_count,
                            provides_with_index: self.next_u2_many(provides_with_count as usize),
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
                    let package_count = self.next_u2();

                    AttributeInfoType::ModulePackagesAttribute {
                        package_count,
                        package_index: self.next_u2_many(package_count as usize),
                    }
                }
                "ModuleMainClass" => AttributeInfoType::ModuleMainClassAttribute {
                    main_class_index: self.next_u2(),
                },
                "NestHost" => AttributeInfoType::NestHostAttribute {
                    host_class_index: self.next_u2(),
                },
                "NestMembers" => {
                    let number_of_classes = self.next_u2();

                    AttributeInfoType::NestMembersAttribute {
                        number_of_classes,
                        classes: self.next_u2_many(number_of_classes as usize),
                    }
                }
                "Record" => {
                    let components_count = self.next_u2();
                    let mut components = Vec::with_capacity(components_count as usize);
                    while components.len() < components_count as usize {
                        let name_index = self.next_u2();
                        let descriptor_index = self.next_u2();
                        let attributes_count = self.next_u2();

                        components.push(RecordComponentInfo {
                            name_index,
                            descriptor_index,
                            attributes_count,
                            attributes: self.parse_attributes(attributes_count, constant_pool)?,
                        });
                    }

                    AttributeInfoType::RecordAttribute {
                        components_count,
                        components,
                    }
                }
                "PermittedSubclasses" => {
                    let number_of_classes = self.next_u2();

                    AttributeInfoType::PermittedSubclassesAttribute {
                        number_of_classes,
                        classes: self.next_u2_many(number_of_classes as usize),
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

        while vec.len() < count as usize {
            vec.push(CodeAttributeExceptionTableEntry {
                start_pc: self.next_u2(),
                end_pc: self.next_u2(),
                handler_pc: self.next_u2(),
                catch_type: self.next_u2(),
            });
        }

        Ok(vec)
    }

    fn parse_stack_map_frame(&mut self) -> ParseResult<StackMapFrame> {
        let frame_type = self.next_u1();

        Ok(match frame_type {
            0..=63 => StackMapFrame::SameFrame { frame_type },
            64..=127 => {
                let stack = self.parse_verification_type_info();

                StackMapFrame::SameLocals1StackItemFrame { frame_type, stack }
            }
            247 => {
                let offset_delta = self.next_u2();
                let stack = self.parse_verification_type_info();

                StackMapFrame::SameLocals1StackItemFrameExtended {
                    frame_type,
                    offset_delta,
                    stack,
                }
            }
            248..=250 => {
                let offset_delta = self.next_u2();

                StackMapFrame::ChopFrame {
                    frame_type,
                    offset_delta,
                }
            }
            251 => {
                let offset_delta = self.next_u2();

                StackMapFrame::SameFrameExtended {
                    frame_type,
                    offset_delta,
                }
            }
            252..=254 => {
                let offset_delta = self.next_u2();

                let locals_length = frame_type - 251;
                let mut locals = Vec::with_capacity(locals_length as usize);
                while locals.len() < locals_length as usize {
                    locals.push(self.parse_verification_type_info());
                }

                StackMapFrame::AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                }
            }
            255 => {
                let offset_delta = self.next_u2();

                let number_of_locals = self.next_u2();
                let mut locals = Vec::with_capacity(number_of_locals as usize);
                while locals.len() < number_of_locals as usize {
                    locals.push(self.parse_verification_type_info());
                }

                let number_of_stack_items = self.next_u2();
                let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                while stack.len() < number_of_stack_items as usize {
                    stack.push(self.parse_verification_type_info());
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

    fn parse_verification_type_info(&mut self) -> VerificationTypeInfo {
        let tag = self.next_u1();

        match tag {
            0 => VerificationTypeInfo::TopVariableInfo { tag },
            1 => VerificationTypeInfo::IntegerVariableInfo { tag },
            2 => VerificationTypeInfo::FloatVariableInfo { tag },
            3 => VerificationTypeInfo::DoubleVariableInfo { tag },
            4 => VerificationTypeInfo::LongVariableInfo { tag },
            5 => VerificationTypeInfo::NullVariableInfo { tag },
            6 => VerificationTypeInfo::UninitializedThisVariableInfo { tag },
            7 => VerificationTypeInfo::ObjectVariableInfo {
                tag,
                cpool_index: self.next_u2(),
            },
            8 => VerificationTypeInfo::UninitializedVariableInfo {
                tag,
                offset: self.next_u2(),
            },
            _ => unreachable!(),
        }
    }

    fn parse_annotation(&mut self) -> ParseResult<Annotation> {
        let type_index = self.next_u2();

        let num_element_value_pairs = self.next_u2();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        while element_value_pairs.len() < num_element_value_pairs as usize {
            element_value_pairs.push(ElementValuePair {
                element_name_index: self.next_u2(),
                value: self.parse_element_value()?,
            });
        }

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn parse_element_value(&mut self) -> ParseResult<ElementValue> {
        let tag = self.next_u1();
        let value = match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                ElementValueValue::ConstValueIndex {
                    const_value_index: self.next_u2(),
                }
            }
            'e' => ElementValueValue::EnumConstValue {
                type_name_index: self.next_u2(),
                const_name_index: self.next_u2(),
            },
            'c' => ElementValueValue::ClassInfoIndex {
                class_info_index: self.next_u2(),
            },
            '@' => ElementValueValue::AnnotationValue {
                annotation_value: self.parse_annotation()?,
            },
            '[' => {
                let num_values = self.next_u2();
                let mut values = Vec::with_capacity(num_values as usize);
                while values.len() < num_values as usize {
                    values.push(self.parse_element_value()?);
                }

                ElementValueValue::ArrayValue { num_values, values }
            }
            _ => unreachable!(),
        };

        Ok(ElementValue { tag, value })
    }

    fn parse_parameter_annotation(&mut self) -> ParseResult<ParameterAnnotation> {
        let num_annotations = self.next_u2();
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        while annotations.len() < num_annotations as usize {
            annotations.push(self.parse_annotation()?);
        }

        Ok(ParameterAnnotation {
            num_annotations,
            annotations,
        })
    }

    fn parse_type_annotation(&mut self) -> ParseResult<TypeAnnotation> {
        let target_type = self.next_u1();
        let target_info = match target_type {
            0x00 | 0x01 => TypeAnnotationTargetInfo::TypeParameterTarget {
                type_parameter_index: self.next_u1(),
            },
            0x10 => TypeAnnotationTargetInfo::SupertypeTarget {
                supertype_index: self.next_u2(),
            },
            0x11 | 0x12 => TypeAnnotationTargetInfo::TypeParameterBoundTarget {
                type_parameter_index: self.next_u1(),
                bound_index: self.next_u1(),
            },
            0x13..=0x15 => TypeAnnotationTargetInfo::EmptyTarget,
            0x16 => TypeAnnotationTargetInfo::FormalParameterTarget {
                formal_parameter_index: self.next_u1(),
            },
            0x17 => TypeAnnotationTargetInfo::ThrowsTarget {
                throws_type_index: self.next_u2(),
            },
            0x40 | 0x41 => {
                let table_length = self.next_u2();
                let mut table = Vec::with_capacity(table_length as usize);
                while table.len() < table_length as usize {
                    table.push(TypeAnnotationLocalVarTargetTableEntry {
                        start_pc: self.next_u2(),
                        length: self.next_u2(),
                        index: self.next_u2(),
                    });
                }

                TypeAnnotationTargetInfo::LocalVarTarget {
                    table_length,
                    table,
                }
            }
            0x42 => TypeAnnotationTargetInfo::CatchTarget {
                catch_index: self.next_u2(),
            },
            0x43..=0x46 => TypeAnnotationTargetInfo::OffsetTarget {
                offset: self.next_u2(),
            },
            0x47..=0x4B => TypeAnnotationTargetInfo::TypeArgumentTarget {
                offset: self.next_u2(),
                type_argument_index: self.next_u1(),
            },
            _ => unreachable!(),
        };

        let path_length = self.next_u1();
        let mut path = Vec::with_capacity(path_length as usize);
        while path.len() < path_length as usize {
            path.push(TypeAnnotationTypePathSegment {
                type_path_kind: self.next_u1(),
                type_argument_index: self.next_u1(),
            });
        }
        let target_path = TypeAnnotationTypePath { path_length, path };

        let type_index = self.next_u2();

        let num_element_value_pairs = self.next_u2();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        while element_value_pairs.len() < num_element_value_pairs as usize {
            element_value_pairs.push(ElementValuePair {
                element_name_index: self.next_u2(),
                value: self.parse_element_value()?,
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
    ) -> ParseResult<Vec<MethodInfo<'clazz>>> {
        let mut vec = Vec::with_capacity(count as usize);

        while vec.len() < count as usize {
            let access_flags = self.next_u2();
            let name_index = self.next_u2();
            let descriptor_index = self.next_u2();
            let attributes_count = self.next_u2();
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
