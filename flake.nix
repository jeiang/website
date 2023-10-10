{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    devenv.url = "github:cachix/devenv";
    nix2container = {
      url = "github:nlewo/nix2container";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    nixpkgs,
    devenv,
    flake-parts,
    treefmt-nix,
    rust-overlay,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        devenv.flakeModule
        treefmt-nix.flakeModule
      ];

      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        inherit (cargoToml.package) name;
        inherit (cargoToml.package) edition;
        inherit (cargoToml.package) version;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };

        build-deps = with pkgs; [
          perseus-cli
          wasm-bindgen-cli
          binaryen
          pkg-config
        ];
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            (final: _prev: {
              #
              perseus-cli = final.rustPlatform.buildRustPackage rec {
                pname = "perseus-cli";
                version = "0.4.2";

                src = final.fetchCrate {
                  inherit pname version;
                  sha256 = "sha256-Qq+DQOJP11A5WMG+l3F1Uh+VjK7A9fKej2UOmFIIYXs=";
                };

                cargoSha256 = "sha256-W1colldWSJ9/M3C8lWxgHCVGgksS9grzFZsMLd4ZFAo=";

                nativeBuildInputs = with final; [makeWrapper pkg-config];
                buildInputs = with pkgs; [openssl];

                postInstall = ''
                  wrapProgram $out/bin/perseus \
                    --prefix PATH : "${final.lib.makeBinPath [final.wasm-pack]}"
                '';

                # Disable tests for https://github.com/framesurge/perseus/issues/305
                doCheck = false;

                meta = with final.lib; {
                  homepage = "https://framesurge.sh/perseus/en-US";
                  description = "A high-level web development framework for Rust with full support for server-side rendering and static generation";
                  maintainers = with maintainers; [max-niederman];
                  license = with licenses; [mit];
                  mainProgram = "perseus";
                };
              };
            })
          ];
        };

        packages.default = config.packages.${name};
        packages.${name} = rustPlatform.buildRustPackage {
          pname = name;
          inherit version;

          src = ./.;
          nativeBuildInputs = build-deps;

          cargoLock = {lockFile = ./Cargo.lock;};

          buildPhase = ''
            # TODO: build
          '';
          installPhase = ''
            cp -r dist $out
          '';

          # Needs to be set to an existing folder
          # TODO: Set up build cache ahead of time
          XDG_CACHE_HOME = "/tmp/build-cache";
        };

        apps.${name} = {
          type = "app";
          program = "${config.packages.${name}}/bin/${name}";
        };

        devenv.shells.default = {
          packages = with pkgs;
            [
              bacon
              lldb
              commitizen
              config.treefmt.build.wrapper
            ]
            ++ build-deps;

          languages.nix.enable = true;
          languages.rust.enable = true;
          languages.rust.toolchain = {
            cargo = toolchain;
            clippy = toolchain;
            rust-analyzer = toolchain;
            rustc = toolchain;
            rustfmt = toolchain;
          };

          pre-commit.hooks.commitizen.enable = true;
          pre-commit.hooks.clippy.enable = true;
          pre-commit.hooks.convco.enable = true;
          pre-commit.hooks.treefmt.enable = true;

          pre-commit.settings.treefmt.package = config.treefmt.build.wrapper;

          difftastic.enable = true;
        };

        treefmt = {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true;
            deadnix.enable = true;
            rustfmt.enable = true;
            rustfmt.package = toolchain;
            rustfmt.edition = edition;
          };
        };
      };
    };
}
