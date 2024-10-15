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
            tailwindcss
          ];

          languages = {
            nix.enable = true;
            kotlin.enable = true;
          };

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
            taplo.enable = true;
          };
        };
      };
    };
}
