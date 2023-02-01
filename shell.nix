let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };
in pkgs.mkShell {
  name = "rust-dev-shell";
  buildInputs = with pkgs; [ 
    rust-analyzer 
    rust
    bacon
    trunk
    wasm-bindgen-cli
    binaryen # wasm-pack may need wasm-opt from here
  ];
}
