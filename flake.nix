{
  description = "signal-flow contract checks";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = { url = "github:nix-community/fenix"; inputs.nixpkgs.follows = "nixpkgs"; };
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, flake-utils, fenix, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.complete.withComponents [ "cargo" "rustc" "rustfmt" ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = pkgs.lib.cleanSourceWith { src = ./.; filter = path: type:
          type == "directory" || (craneLib.filterCargoSources path type) || pkgs.lib.hasSuffix ".ethos" path; };
        commonArgs = { inherit src; strictDeps = true; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          default = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          generated = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; cargoTestExtraArgs = "--test contract"; });
          fmt = craneLib.cargoFmt { inherit src; };
        };
      });
}
