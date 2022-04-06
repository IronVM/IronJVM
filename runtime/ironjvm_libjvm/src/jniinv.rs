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

//! JNI Invocation API implementation

use std::ffi::c_void;

use jni_sys::{JavaVM, JavaVMInitArgs, jint, JNI_ERR, JNI_OK, JNI_VERSION_10};

#[no_mangle]
pub extern "C" fn JNI_CreateJavaVM(_: *mut *mut JavaVM, _: *mut *mut c_void, args: *mut c_void) -> jint {
    let _ = args as *mut JavaVMInitArgs;

    JNI_OK
}

#[no_mangle]
pub extern "C" fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint {
    let args = args as *mut JavaVMInitArgs;

    unsafe {
        let version = (*args).version;
        (*args).version = JNI_VERSION_10;

        if version < JNI_VERSION_10 {
            return JNI_ERR;
        }
    }

    JNI_OK
}
