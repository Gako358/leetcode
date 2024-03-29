{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , naersk
    ,
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      naersk-lib = pkgs.callPackage naersk { };
      # Package set for this system, add packages here
    in
    {
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          python39Packages.numpy
          python39Packages.pandas
          python39Packages.matplotlib
        ];
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy
        ];
      };
    });
}
