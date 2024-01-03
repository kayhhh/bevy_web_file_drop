{ lib, pkgs, system, build_inputs, native_build_inputs, makeRustPlatform }:
let
  rustBin = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustBin;
    rustc = rustBin;
  };

  common = {
    version = "0.0.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    buildInputs = build_inputs;
    nativeBuildInputs = native_build_inputs;

    LD_LIBRARY_PATH = lib.makeLibraryPath build_inputs;
  };
in {
  example = rustPlatform.buildRustPackage (common // {
    pname = "example";
    buildPhase = "trunk build --release";
    installPhase = ''
      mkdir -p $out/web
      cp -r ./dist/* $out/web
    '';
  });
  lib =
    rustPlatform.buildRustPackage (common // { pname = "bevy_web_file_drop"; });
}
