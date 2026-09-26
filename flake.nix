{
  description = "The signal-flow Signal contract.";

  inputs = {
    nixpkgs.url = "github:LiGoldragon/nixpkgs?ref=main";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      crane,
    }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forSystems = function: nixpkgs.lib.genAttrs systems (system: function system);
      mkContext =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          toolchain = fenix.packages.${system}.complete.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
            "clippy"
            "rust-src"
          ];
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          # The build script reads the ethos source beside the Rust.
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type: (craneLib.filterCargoSources path type) || (pkgs.lib.hasSuffix ".ethos" path);
          };
          commonArgs = {
            inherit src;
            strictDeps = true;
          };
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { cargoExtraArgs = "--all-features"; });
        in
        {
          inherit craneLib commonArgs cargoArtifacts;
        };
    in
    {
      checks = forSystems (
        system:
        let
          context = mkContext system;
        in
        {
          # Every contract test runs with the datom feature: the round trips
          # are the falsifiable specification of each record kind.
          default = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--all-features";
            }
          );
        }
      );
    };
}
