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

use crate::classfile::AttributeInfo;

pub mod bmattr;
pub mod cattr;
pub mod icattr;
pub mod lntattr;
pub mod lvtattr;
pub mod lvttattr;
pub mod mattr;
pub mod mpattr;
pub mod rattr;
pub mod rvanriaattr;
pub mod rvpaattr;
pub mod rvtnritaattr;
pub mod smtattr;

#[derive(Debug)]
pub enum AttributeInfoType<'clazz> {
    ConstantValueAttribute {
        constantvalue_index: [u8; 2],
    },
    CodeAttribute {
        max_stack: [u8; 2],
        max_locals: [u8; 2],
        code_length: u32,
        code: &'clazz [u8],
        exception_table_length: [u8; 2],
        exception_table: Vec<cattr::CodeAttributeExceptionTableEntry>,
        attributes_count: [u8; 2],
        attributes: Vec<AttributeInfo<'clazz>>,
    },
    StackMapTableAttribute {
        number_of_entries: [u8; 2],
        stack_map_table: Vec<smtattr::StackMapFrame>,
    },
    ExceptionsAttribute {
        number_of_exceptions: [u8; 2],
        exception_index_table: &'clazz [[u8; 2]],
    },
    InnerClassesAttribute {
        number_of_classes: [u8; 2],
        classes: Vec<icattr::InnerClass>,
    },
    EnclosingMethodAttribute {
        class_index: [u8; 2],
        method_index: [u8; 2],
    },
    SyntheticAttribute,
    SignatureAttribute {
        signature_index: [u8; 2],
    },
    SourceFileAttribute {
        sourcefile_index: [u8; 2],
    },
    SourceDebugExtensionAttribute {
        debug_extension: &'clazz [u8],
    },
    LineNumberTableAttribute {
        line_number_table_length: [u8; 2],
        line_number_table: Vec<lntattr::LineNumber>,
    },
    LocalVariableTableAttribute {
        local_variable_table_length: [u8; 2],
        local_variable_table: Vec<lvtattr::LocalVariable>,
    },
    LocalVariableTypeTableAttribute {
        local_variable_type_table_length: [u8; 2],
        local_variable_type_table: Vec<lvttattr::LocalVariableType>,
    },
    DeprecatedAttribute,
    RuntimeVisibleAnnotationsAttribute {
        num_annotations: [u8; 2],
        annotations: Vec<rvanriaattr::Annotation>,
    },
    RuntimeInvisibleAnnotationsAttribute {
        num_annotations: [u8; 2],
        annotations: Vec<rvanriaattr::Annotation>,
    },
    RuntimeVisibleParameterAnnotationsAttribute {
        num_parameters: [u8; 2],
        parameter_annotations: Vec<rvpaattr::ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotationsAttribute {
        num_parameters: [u8; 2],
        parameter_annotations: Vec<rvpaattr::ParameterAnnotation>,
    },
    RuntimeVisibleTypeAnnotationsAttribute {
        num_annotations: [u8; 2],
        annotations: Vec<rvtnritaattr::TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotationsAttribute {
        num_annotations: [u8; 2],
        annotations: Vec<rvtnritaattr::TypeAnnotation>,
    },
    AnnotationDefaultAttribute {
        default_value: rvanriaattr::ElementValue,
    },
    BootstrapMethodsAttribute {
        num_bootstrap_methods: [u8; 2],
        bootstrap_methods: Vec<bmattr::BootstrapMethod<'clazz>>,
    },
    MethodParametersAttribute {
        parameters_count: u8,
        parameters: Vec<mpattr::MethodParameter>,
    },
    ModuleAttribute {
        module_name_index: [u8; 2],
        module_flags: [u8; 2],
        module_version_index: [u8; 2],
        requires_count: [u8; 2],
        requires: Vec<mattr::ModuleRequire>,
        exports_count: [u8; 2],
        exports: Vec<mattr::ModuleExport<'clazz>>,
        opens_count: [u8; 2],
        opens: Vec<mattr::ModuleOpen<'clazz>>,
        uses_count: [u8; 2],
        uses_index: &'clazz [[u8; 2]],
        provides_count: [u8; 2],
        provides: Vec<mattr::ModuleProvide<'clazz>>,
    },
    ModulePackagesAttribute {
        package_count: [u8; 2],
        package_index: &'clazz [[u8; 2]],
    },
    ModuleMainClassAttribute {
        main_class_index: [u8; 2],
    },
    NestHostAttribute {
        host_class_index: [u8; 2],
    },
    NestMembersAttribute {
        number_of_classes: [u8; 2],
        classes: &'clazz [[u8; 2]],
    },
    RecordAttribute {
        components_count: [u8; 2],
        components: Vec<rattr::RecordComponentInfo<'clazz>>,
    },
    PermittedSubclassesAttribute {
        number_of_classes: [u8; 2],
        classes: &'clazz [[u8; 2]],
    },
}
