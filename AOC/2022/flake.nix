{
  inputs = {
    nixpkgs.url = "github:jordanisaacs/nixpkgs/aoc-cli-init";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crate2nix,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [
      rust-overlay.overlays.default
      (self: super: let
        rust = super.rust-bin.stable.latest.default;
      in {
        rustc = rust;
        cargo = rust;
      })
    ];

    pkgs = import nixpkgs {
      inherit system overlays;
    };
    inherit
      (import "${crate2nix}/tools.nix" {inherit pkgs;})
      generateCargoNix
      ;

    name = "advent-of-code";
    pkg =
      (
        import
        (generateCargoNix {
          inherit name;
          src = ./.;
        })
        {inherit pkgs;}
      )
      .workspaceMembers
      .client
      .build;

    nativeBuildInputs = with pkgs; [
      rustc
      cargo
      cargo-edit
      cargo-audit
      cargo-tarpaulin
      clippy
    ];

    alias = "aoc -s /home/merrinx/Projects/leetcode/AOC/2022/.adventofcode.session";
  in
    with pkgs; {
      packages.${system} = {
        "${name}" = pkg;
        default = pkg;
      };

      devShells.${system}.default = mkShell {
        nativeBuildInputs = nativeBuildInputs ++ [pkgs.aoc-cli];
        shellHook = ''
          if [ -z ''${IS_DIRENV} ];
          then
            alias aoc="${alias}";
          else
            export_alias aoc '${alias} $@'
          fi
          echo HI
        '';
      };
    };
}
