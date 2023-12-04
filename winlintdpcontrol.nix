{ lib
, rustPlatform
}:
rustPlatform.buildRustPackage {
  name = "winlintdpcontrol";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  meta = with lib; {
    description = "";
    homepage = "https://github.com/shymega/WinLinTDPControl";
    license = licenses.mit;
  };
}
