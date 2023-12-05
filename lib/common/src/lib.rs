// SPDX-FileCopyrightText: 2023 The PwrCtrl Developers
//
// SPDX-License-Identifier: Apache-2.0
//! Common (shared) crate for `PwrCtrl`.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
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

use anyhow::Result;
use thiserror_no_std::Error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum OsKind {
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "linux")]
    Linux,
    #[default]
    Unselected,
}

#[derive(Debug, PartialEq, Eq, Error, Clone, Copy)]
pub enum TdpControlError {
    ControlInterfaceError { os: OsKind },
    GenericDriverError { os: OsKind },
    GetTdpError { os: OsKind, cpu: CpuKind },
    SetTdpError { os: OsKind, cpu: CpuKind },
}

pub type TdpControlResult<T, E = TdpControlError> = Result<T, E>;

pub trait TdpControl {
    fn set_tdp(tdp: i64) -> TdpControlResult<()>;
    fn get_current_tdp() -> TdpControlResult<Option<i64>> {
        Ok(None)
    }
    fn is_control_available() -> TdpControlResult<bool> {
        Ok(false)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum CpuKind {
    #[cfg(feature = "amd_cpu")]
    AmdCpuFamily(AmdCpuFamilyKind),
    #[cfg(feature = "intel_cpu")]
    IntelCpuFamily(IntelCpuFamilyKind),
    #[default]
    UnknownCpuFamily,
}

#[cfg(feature = "amd_cpu")]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AmdCpuFamilyKind {
    Raven,
    Picasso,
    Renoir,
    Cezanne,
    Dali,
    Lucienne,
    Vangogh,
    Rembrandt,
    Mendocino,
    Phoenix,
    End,
    #[default]
    UnknownAmdCpuFamily,
}

#[cfg(feature = "intel_cpu")]
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum IntelCpuFamilyKind {
    #[default]
    UnknownIntelCpuFamily,
}

pub mod reexports {
    //! Re-exports of `PwrCtrl` components.
    pub use super::{OsKind, TdpControl, TdpControlError, TdpControlResult};
}
