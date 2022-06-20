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

use std::slice::Iter;
use std::slice::SliceIndex;

use crate::classfile::ClassFile;
use crate::classfile::CpInfo;
use crate::classfile::MethodInfo;

pub trait RetrievalApi<'clazz> {
    fn constant_pool(&'clazz self) -> Vec<CpInfo<'clazz>> {
        unimplemented!("`constant_pool` function not implemented for this element")
    }

    fn methods_iter(&'clazz self) -> Iter<'clazz, MethodInfo<'clazz>> {
        unimplemented!("`methods` function not implemented for this element")
    }
}

pub trait RetrievalApiIterator<'clazz, T> {
    fn find_where<F>(&self, f: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        unimplemented!("`find_where` function not implemented for this element")
    }
}

pub trait RetrievalApiVector<'clazz, T> {
    fn element_at<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<Self> {
        unimplemented!("`get` function not implemented for this element")
    }
}

impl<'clazz> RetrievalApi<'clazz> for ClassFile<'clazz> {
    fn constant_pool(&'clazz self) -> Vec<CpInfo<'clazz>> {
        self.constant_pool.clone()
    }

    fn methods_iter(&'clazz self) -> Iter<'clazz, MethodInfo<'clazz>> {
        self.methods.iter()
    }
}

impl<'clazz> RetrievalApiIterator<'clazz, MethodInfo<'clazz>> for Iter<'clazz, MethodInfo<'clazz>> {
    fn find_where<F>(&mut self, f: F) -> Option<MethodInfo<'clazz>> {
        self.find(f).map(|t| t.clone())
    }
}

impl<'clazz> RetrievalApiVector<'clazz, CpInfo<'clazz>> for Vec<CpInfo<'clazz>> {
    fn element_at<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<Self> {
        self.get(index - 1)
    }
}
