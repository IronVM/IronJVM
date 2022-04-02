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

use std::process::Command;

pub fn main() {
    let commit_hash = Command::new("git")
        .args(["rev-parse", "--short=9", "HEAD"])
        .output()
        .map(|output| {
            let full = String::from_utf8(output.stdout).unwrap_or(String::new());
            full.lines().collect::<Vec<_>>()[0].to_string()
        })
        .unwrap_or(String::new());
    let commit_date = Command::new("git")
        .args(["log", "--date=short", "--pretty=format:%cd"])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap_or(String::new()))
        .unwrap_or(String::new());

    println!("cargo:rustc-env=IRONJVM_REVISION_HASH_DATE={commit_hash} {commit_date}");
    println!("cargo:rustc-env=IRONJVM_JAVA_VERSION=18");
}
