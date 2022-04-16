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

#[derive(Clone, Debug)]
pub enum AttributeInfoType {
    ConstantValueAttribute {
        constantvalue_index: u16,
    },
    CodeAttribute {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<cattr::CodeAttributeExceptionTableEntry>,
        attributes_count: u16,
        attributes: Vec<AttributeInfo>,
    },
    StackMapTableAttribute {
        number_of_entries: u16,
        stack_map_table: Vec<smtattr::StackMapFrame>,
    },
    ExceptionsAttribute {
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },
    InnerClassesAttribute {
        number_of_classes: u16,
        classes: Vec<icattr::InnerClass>,
    },
    EnclosingMethodAttribute {
        class_index: u16,
        method_index: u16,
    },
    SyntheticAttribute,
    SignatureAttribute {
        signature_index: u16,
    },
    SourceFileAttribute {
        sourcefile_index: u16,
    },
    SourceDebugExtensionAttribute {
        debug_extension: Vec<u8>,
    },
    LineNumberTableAttribute {
        line_number_table_length: u16,
        line_number_table: Vec<lntattr::LineNumber>,
    },
    LocalVariableTableAttribute {
        local_variable_table_length: u16,
        local_variable_table: Vec<lvtattr::LocalVariable>,
    },
    LocalVariableTypeTableAttribute {
        local_variable_type_table_length: u16,
        local_variable_type_table: Vec<lvttattr::LocalVariableType>,
    },
    DeprecatedAttribute,
    RuntimeVisibleAnnotationsAttribute {
        num_annotations: u16,
        annotations: Vec<rvanriaattr::Annotation>,
    },
    RuntimeInvisibleAnnotationsAttribute {
        num_annotations: u16,
        annotations: Vec<rvanriaattr::Annotation>,
    },
    RuntimeVisibleParameterAnnotationsAttribute {
        num_parameters: u16,
        parameter_annotations: Vec<rvpaattr::ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotationsAttribute {
        num_parameters: u16,
        parameter_annotations: Vec<rvpaattr::ParameterAnnotation>,
    },
    RuntimeVisibleTypeAnnotationsAttribute {
        num_annotations: u16,
        annotations: Vec<rvtnritaattr::TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotationsAttribute {
        num_annotations: u16,
        annotations: Vec<rvtnritaattr::TypeAnnotation>,
    },
    AnnotationDefaultAttribute {
        default_value: rvanriaattr::ElementValue,
    },
    BootstrapMethodsAttribute {
        num_bootstrap_methods: u16,
        bootstrap_methods: Vec<bmattr::BootstrapMethod>,
    },
    MethodParametersAttribute {
        parameters_count: u8,
        parameters: Vec<mpattr::MethodParameter>,
    },
    ModuleAttribute {
        module_name_index: u16,
        module_flags: u16,
        module_version_index: u16,
        requires_count: u16,
        requires: Vec<mattr::ModuleRequire>,
        exports_count: u16,
        exports: Vec<mattr::ModuleExport>,
        opens_count: u16,
        opens: Vec<mattr::ModuleOpen>,
        uses_count: u16,
        uses_index: Vec<u16>,
        provides_count: u16,
        provides: Vec<mattr::ModuleProvide>,
    },
    ModulePackagesAttribute {
        package_count: u16,
        package_index: Vec<u16>,
    },
    ModuleMainClassAttribute {
        main_class_index: u16,
    },
    NestHostAttribute {
        host_class_index: u16,
    },
    NestMembersAttribute {
        number_of_classes: u16,
        classes: Vec<u16>,
    },
    RecordAttribute {
        components_count: u16,
        components: Vec<rattr::RecordComponentInfo>,
    },
    PermittedSubclassesAttribute {
        number_of_classes: u16,
        classes: Vec<u16>,
    },
}
