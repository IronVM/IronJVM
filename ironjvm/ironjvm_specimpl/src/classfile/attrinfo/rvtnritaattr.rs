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

use super::rvanriaattr;

#[derive(Clone, Debug)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TypeAnnotationTargetInfo,
    pub target_path: TypeAnnotationTypePath,
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<rvanriaattr::ElementValuePair>,
}

#[derive(Clone, Debug)]
pub enum TypeAnnotationTargetInfo {
    TypeParameterTarget {
        type_parameter_index: u8,
    },
    SupertypeTarget {
        supertype_index: u16,
    },
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: u8,
    },
    ThrowsTarget {
        throws_type_index: u16,
    },
    LocalVarTarget {
        table_length: u16,
        table: Vec<TypeAnnotationLocalVarTargetTableEntry>,
    },
    CatchTarget {
        catch_index: u16,
    },
    OffsetTarget {
        offset: u16,
    },
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Clone, Debug)]
pub struct TypeAnnotationLocalVarTargetTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Clone, Debug)]
pub struct TypeAnnotationTypePath {
    pub path_length: u8,
    pub path: Vec<TypeAnnotationTypePathSegment>,
}

#[derive(Clone, Debug)]
pub struct TypeAnnotationTypePathSegment {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}
