{
  description = "my website";

  inputs = {
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-darwin" ];

      perSystem = { config, pkgs, ... }: {
        devenv.shells.default = {
          devenv.root =
            let
              devenvRootFileContent = builtins.readFile devenv-root.outPath;
            in
            pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

          name = "website-dev";

          # https://devenv.sh/reference/options/
          packages = with pkgs; [
            sqlite
            tailwindcss
            fish
            bacon
            cargo-expand
            (import ./config/perseus.nix {
              inherit (pkgs) lib stdenv rustPlatform fetchCrate makeWrapper wasm-pack;
              inherit (pkgs.darwin.apple_sdk.frameworks) CoreServices;
            })
          ];

          languages = {
            rust = {
              enable = true;
              channel = "nightly";
              targets = [
                "x86_64-unknown-linux-gnu"
                "aarch64-apple-darwin"
              ];
            };
            nix.enable = true;
          };
          process.implementation = "overmind";
          processes.tailwind.exec = ''
            ROOT=$(git rev-parse --show-toplevel)
            tailwindcss --config "$ROOT/config/tailwind.config.js" -i "$ROOT/src/index.css" -o "$ROOT/src/static/index.css" --watch
          '';

          pre-commit.hooks = {
            editorconfig-checker.enable = true;
            markdownlint = {
              enable = true;
              settings.configuration = {
                "MD013" = {
                  "line_length" = 120;
                };
              };
            };
            nil.enable = true;
            statix.enable = true;
            treefmt = {
              enable = true;
              package = config.treefmt.build.wrapper;
            };
          };
        };
        treefmt = {
          projectRootFile = "flake.nix";
          programs = {
            deadnix.enable = true;
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
            taplo.enable = true;
          };
        };
      };
    };
}
