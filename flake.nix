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
        devShells.default = pkgs.mkShell {
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
            inherit version;

            # Only the inputs bun install reads, so site edits do not
            # invalidate the node_modules build.
            src = pkgs.lib.cleanSourceWith {
              src = ./.;
              filter = path: _type:
                builtins.elem (baseNameOf path) ["package.json" "bun.lock"];
            };

            nativeBuildInputs = with pkgs; [bun];

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

            outputHash = {
              x86_64-linux = "sha256-KViHtrD62rbR8ibEY66K0ETUrG16U0tvJTeQHyTsDyQ=";
              aarch64-darwin = "sha256-155XRRaKxMfZQgQdTcR21vak7ZH+pRHAJ4E5X1tKk3Y=";
            }
            .${system} or (throw "no node_modules FOD hash for ${system}");
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
