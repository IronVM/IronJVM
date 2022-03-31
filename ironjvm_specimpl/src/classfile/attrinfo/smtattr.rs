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

use rend::u16_be;

pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: Vec<VerificationTypeInfo>,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16_be,
        stack: Vec<VerificationTypeInfo>,
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16_be,
    },
    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16_be,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16_be,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: u8,
        offset_delta: u16_be,
        number_of_locals: u16_be,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16_be,
        stack: Vec<VerificationTypeInfo>,
    },
}

pub enum VerificationTypeInfo {
    TopVariableInfo { tag: u8 },
    IntegerVariableInfo { tag: u8 },
    FloatVariableInfo { tag: u8 },
    DoubleVariableInfo { tag: u8 },
    LongVariableInfo { tag: u8 },
    NullVariableInfo { tag: u8 },
    UninitializedThisVariableInfo { tag: u8 },
    ObjectVariableInfo { tag: u8 },
    UninitializedVariableInfo { tag: u8, cpool_index: u16_be },
}
