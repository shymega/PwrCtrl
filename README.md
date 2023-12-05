<!--
SPDX-FileCopyrightText: 2023 The PwrCtrl Developers

SPDX-License-Identifier: Apache-2.0
-->

# PwrCtrl

PwrCtrl (previously known as `WinLinTDPControl`) is a kernel driver (for
Windows, Linux) and associated IPC protocol for controlling TDP/TGP on AMD and
Intel CPUs.

My aim is for a secure, minimal attack surface, and extendable signed kernel driver to be used by software like 'Handheld Companion', or just on its own.

## Checklist

- [ ] Populate `IntelCpuFamilyKind` and `AmdCpuFamilyKind` enums with variants.
- [ ] Wait for `windows-drivers-rs` to be stable for production.
- [ ] Research linking Rust code into pre-6.0 Linux kernels without native Rust modules. It should be out-of-tree.
- [ ] Add initial support for Intel TDP adjustment.
- [ ] Add initial support for AMD TDP adjustment.
- [ ] Design a secure, with minimal attack surface, IPC protocol for both Windows (UAC-authenticated?) and Linux (Polkit-authenticated?).

Licensed under Apache-2.0.
