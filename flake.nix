{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustToolchain;
          cargo = rustToolchain;
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        inherit (cargoToml.package) name;
        inherit (cargoToml.package) version;
      in rec {
        packages = flake-utils.lib.flattenTree {
          ${cargoToml.package.name} = rustPlatform.buildRustPackage {
            pname = name;
            inherit version;
            src = ./.;

            # Required for serve app
            # TODO: define in serve app
            buildInputs = with pkgs; [
              trunk
            ];
          
            nativeBuildInputs = with pkgs; [
              binaryen
              nodePackages.sass
              pkgconfig
              rustToolchain
              trunk
              wasm-bindgen-cli
            ];

            cargoLock = { lockFile = ./Cargo.lock; };

            # avoid the double compile caused by trunk build & cargo check
            doCheck = false;
            buildPhase = "trunk build --release";
            installPhase = ''
              cp -r dist $out
            '';

            # Needs to be set to an existing folder
            # TODO: Set up build cache ahead of time
            XDG_CACHE_HOME = "/tmp/build-cache";
          };
        };

        defaultPackage = packages.${cargoToml.package.name};

        apps.serve = flake-utils.lib.mkApp (let 
          script = pkgs.writeScriptBin "${name}" '' 
            ${pkgs.ran}/bin/ran -r ${defaultPackage}
          '';
        in {
          drv = script;
        });

        defaultApp = apps.serve;

        devShell = pkgs.mkShell {
          name = "rust web-dev shell";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            bacon
            binaryen
            nodePackages.sass
            pkgconfig
            rustToolchain
            rust-analyzer
            trunk
            wasm-bindgen-cli
            zlib
          ];
          
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
            pkgs.lib.makeLibraryPath [ pkgs.zlib ]
          }"'';

          RUST_BACKTRACE = 1;
        };
      }
    );
}
