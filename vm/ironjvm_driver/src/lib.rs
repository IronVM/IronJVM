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

use std::env;

use ironjvm_session::config;
use ironjvm_session::getopts;

mod opthandle;

pub fn ironjvm_main() {
    let args = env::args().collect::<Vec<_>>();
    handle_vm_options(&args);
}

pub fn handle_vm_options(args: &[String]) -> Option<getopts::Matches> {
    let args = &args[1..];

    if args.is_empty() {
        opthandle::ironjvm_usage();
        return None;
    }

    let mut options = getopts::Options::new();
    config::jvm_options().iter().for_each(|option| {
        (option.apply)(&mut options);
    });

    todo!()
}
