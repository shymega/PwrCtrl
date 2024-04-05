# SPDX-FileCopyrightText: 2023 The PwrCtrl Developers
#
# SPDX-License-Identifier: Apache-2.0

{ lib
, rustPlatform
, pkgs
}:
rustPlatform.buildRustPackage {
  name = "pwrctrl";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = [ pkgs.libcpuid ];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/shymega/PwrCtrl";
    license = licenses.mit;
  };
}
