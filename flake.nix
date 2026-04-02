{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import (inputs.nixpkgs) {inherit system;};
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            bun
            astro-language-server
            typescript-language-server
          ];
        };

        packages = let
          src = ./.;
          packageJson = pkgs.lib.importJSON "${src}/package.json";
          version = packageJson.version;
          pname = packageJson.name;

          node_modules = pkgs.stdenv.mkDerivation {
            pname = "${pname}_node-modules";
            inherit src version;

            nativeBuildInputs = with pkgs; [bun];
            buildInputs = with pkgs; [nodejs-slim_latest];

            dontConfigure = true;
            dontFixup = true; # patchShebangs produces illegal path references in FODs

            buildPhase = ''
              runHook preBuild
              export HOME=$TMPDIR
              bun install --no-progress --frozen-lockfile
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              mkdir -p $out/node_modules
              mv node_modules $out/
              runHook postInstall
            '';

            outputHash = "sha256-Tt7kqID0Q6h1/KGOvZv5s1kdjr+XgZggA/A5qOiiM5U=";
            outputHashAlgo = "sha256";
            outputHashMode = "recursive";
          };
        in {
          default = pkgs.stdenv.mkDerivation {
            inherit pname version src;

            nativeBuildInputs = with pkgs; [
              node_modules
              nodejs-slim_latest
              bun
            ];

            configurePhase = ''
              runHook preConfigure
              cp -a ${node_modules}/node_modules ./node_modules
              chmod -R u+rw node_modules
              chmod -R u+x node_modules/.bin
              patchShebangs node_modules
              export HOME=$TMPDIR
              export PATH="$PWD/node_modules/.bin:$PATH"
              bun astro telemetry disable
              runHook postConfigure
            '';

            buildPhase = ''
              runHook preBuild
              bun run build
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              cp -pr --reflink=auto dist $out/
              runHook postInstall
            '';
          };
        };
      }
    );
}
