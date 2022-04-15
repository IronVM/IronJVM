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

pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

pub struct ElementValue {
    pub tag: u8,
    pub value: ElementValueValue,
}

pub enum ElementValueValue {
    ConstValueIndex { const_value_index: u16 },
    EnumConstValue { type_name_index: u16, const_name_index: u16 },
    ClassInfoIndex { class_info_index: u16 },
    AnnotationValue { annotation_value: Annotation },
    ArrayValue { num_values: u16, values: Vec<ElementValue> },
}

pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}
