{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = inputs: let
    # The systems the node_modules FOD has a hash for are the systems
    # this flake supports; there is no fallback.
    nodeModulesHashes = {
      x86_64-linux = "sha256-HGEjFhGi7/FTqewVvbISTOIKQP6yfXL3YHRMbOrlts4=";
      aarch64-darwin = "sha256-ZhJOu9Rzl8AHL6W8M9y3zFiqDOheyu98oaDDCVaNSDE=";
    };
    systems = builtins.attrNames nodeModulesHashes;
    forAllSystems = f:
      inputs.nixpkgs.lib.genAttrs systems (
        system: f system (import inputs.nixpkgs {inherit system;})
      );
  in {
    devShells = forAllSystems (_system: pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          bun
          astro-language-server
          typescript-language-server
        ];
      };
    });

    packages = forAllSystems (
      system: pkgs: let
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

          outputHash = nodeModulesHashes.${system};
          outputHashAlgo = "sha256";
          outputHashMode = "recursive";
        };
      in {
        default = pkgs.stdenv.mkDerivation {
          inherit pname version src;

          nativeBuildInputs = with pkgs; [nodejs-slim_latest bun];

          configurePhase = ''
            runHook preConfigure
            cp -a ${node_modules}/node_modules ./node_modules
            chmod -R u+rw node_modules
            patchShebangs node_modules
            export HOME=$TMPDIR
            export ASTRO_TELEMETRY_DISABLED=1
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
      }
    );
  };
}
