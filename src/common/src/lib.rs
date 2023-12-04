// SPDX-FileCopyrightText: The WinLinTDPControl Developers
//
// SPDX-License-Identifier: Apache-2.0
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

#[derive(Debug, Default, PartialEq, Eq)]
enum OsKind {
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "linux")]
    Linux,
    #[default]
    Unselected,
}

#[derive(Debug, PartialEq, Eq, Error)]
enum TdpControlError {
    ControlInterfaceError {
        os: OsKind,
    },
    GenericDriverError{
        os: OsKind,
    },
    GetTdpError {
        os: OsKind,
        cpu: CpuKind
    },
    SetTdpError {
        os: OsKind,
        cpu: CpuKind
    },
}

type TdpControlResult<T, E = TdpControlError> = Result<T, E>;

trait TdpControl {
    fn set_tdp(tdp: i64) -> TdpControlResult<()>;
    fn get_current_tdp() -> TdpControlResult<Option<i64>> {
        Ok(None)
    }
    fn is_control_available() -> TdpControlResult<bool> {
        Ok(false)
    }
}


#[derive(Debug, Default, PartialEq, Eq())]
enum CpuKind {
    #[cfg(feature = "amd_cpu_family")]
    AmdCpuFamily(AmdCpuFamilyKind),
    #[cfg(feature = "intel_cpu_family")]
    IntelCpuFamily(IntelCpuFamilyKind),
    #[default]
    UnknownCpuFamily,
}

#[cfg(feature = "amd_cpu_family")]
#[derive(Debug, Default, PartialEq, Eq())]
enum AmdCpuFamilyKind {
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
    UnknownAmdCpuFamily
}

#[cfg(feature = "intel_cpu_family")]
#[derive(Debug, Default, PartialEq, Eq())]
enum IntelCpuFamilyKind {
     #[default]
    UnknownIntelCpuFamily
}


pub mod reexports {
    //! Re-exports of `WinLinTDPControl` components.
    pub use super::{OsKind, TdpControlResult, TdpControlError, TdpControl};
}
