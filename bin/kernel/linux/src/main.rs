// SPDX-FileCopyrightText: 2023 The PwrCtrl Developers
//
// SPDX-License-Identifier: Apache-2.0
//! Entrypoint for `PwrCtrl` Linux kernel module .
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![no_main]
#![no_std]

#[cfg(target_os = "linux")]
fn _start() {
    unimplemented!()
}
