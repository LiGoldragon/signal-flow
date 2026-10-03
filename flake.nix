{
  description = "signal-flow ordinary wire contract and generated ethos checks";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { nixpkgs, flake-utils, rust-build, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromToolchainFile pkgs {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
        };
        contractFilter = path: type:
          type == "regular" && (
            pkgs.lib.hasSuffix ".ethos" path ||
            pkgs.lib.hasSuffix "/build.rs" path ||
            builtins.match ".*/src/generated(/.*)?$" path != null
          );
        src = rust.cleanSource { root = ./.; extraFilters = [ contractFilter ]; };
        commonArgs = { inherit src; strictDeps = true; };
        cargoArtifacts = rust.craneLib.buildDepsOnly commonArgs;
      in {
        packages.default = rust.craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          test = rust.craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          test-datom = rust.craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features";
          });
          fmt = rust.craneLib.cargoFmt { inherit src; };
        };
      });
}
