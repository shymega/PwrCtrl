// SPDX-FileCopyrightText: The WinLinTDPControl Developers
//
// SPDX-License-Identifier: Apache-2.0
//! Entrypoint for `WinLinTDPControl` Windows (Win32) kernel module .
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

#[cfg(target_os = "windows")]
fn _start() {
    unimplemented!()
}
