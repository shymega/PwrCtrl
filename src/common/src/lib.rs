// SPDX-FileCopyrightText: 2023 The WinLinTDPControl Developers
//
// SPDX-License-Identifier: MIT

#![no_main]
#![no_std]

pub enum TdpControlError {
    SetError
}

pub trait TdpControl {
    fn set_tdp(tdp: i64) -> Result<(), TdpControlError>;
    fn get_tdp() -> Option<i64> {
        None
    }
}
