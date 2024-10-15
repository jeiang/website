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
    treefmt-nix.url = "github:numtide/treefmt-nix";
    gradle2nix.url = "github:tadfisher/gradle2nix/v2";
    gradle2nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ flake-parts, devenv-root, gradle2nix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-darwin" ];

      perSystem = { config, pkgs, system, ... }:
        let
          pname = "website";
          version = "1.0";
        in
        rec {
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
              ktfmt.enable = true;
            };
          };

          packages.default = gradle2nix.builders.${system}.buildGradlePackage {
            inherit pname version;
            src = ./.;
            lockFile = ./gradle.lock;
            gradleBuildFlags = [ "installDist" "--no-daemon" ];
            nativeBuildInputs = [ pkgs.tailwindcss ];
            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin $out/lib
              mv build/install/${pname}/bin/${pname} $out/bin
              mv build/install/${pname}/lib/* $out/lib
              runHook postInstall
            '';
          };

          apps.default = {
            type = "app";
            program = "${packages.default}/bin/${pname}";
          };
        };
    };
}
