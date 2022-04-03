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

use ironjvm_session::config;
use ironjvm_session::getopts;

pub fn ironjvm_usage() {
    let jvm_options = config::jvm_options();

    let mut options = getopts::Options::new();
    jvm_options.iter().for_each(|option| {
        (option.apply)(&mut options);
    });

    println!(
        "{options}",
        options = options.usage("Usage: ironjvm [options] <mainclass> [args...]"),
    );
}

pub fn ironjvm_version() {
    let java_version = env!("IRONJVM_JAVA_VERSION");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let git_revision_hash = env!("IRONJVM_REVISION_HASH");
    let git_revision_date = env!("IRONJVM_REVISION_DATE");

    println!("IronJVM for Java {java_version}");
    println!("IronJVM {pkg_version} ({git_revision_hash} {git_revision_date})");
}
