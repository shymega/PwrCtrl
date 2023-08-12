// SPDX-FileCopyrightText: The WinLinTDPControl Developers
//
// SPDX-License-Identifier: MIT
//! Common (shared) crate for `WinLinTDPControl`.
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

use thiserror_no_std::Error;
use anyhow::Result;

pub enum TdpControlError {
    ControlInterfaceError,
    GenericDriverError,
    GetTdpError,
    SetTdpError,
}

pub type TdpControlResult<T, E = TdpControlError> = anyhow::Result<T>;

pub trait TdpControl {
    fn set_tdp(tdp: i64) -> TdpControlResult<()>;
    fn get_current_tdp() -> TdpControlResult<Option<i64>> {
        Ok(None)
    }
    fn is_control_available() -> TdpControlResult<bool> {
        Ok(false)
    }
}

pub mod reexports {
    //! Re-exports of `WinLinTDPControl` components.
}
