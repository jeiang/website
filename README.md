# aidanpinard.co Website

My website, made using [Yew] and built with [Trunk].

## Building & Serving

### The normal way

Using trunk (follow the instructions on [trunk's website](https://trunkrs.dev/#install)):
```bash
# Build output (files are located in dist folder)
trunk build

# Build watch, and serve the files
trunk serve

# Add --release to build/serve for "more perf"
trunk serve --release
```

After building, other static file servers can serve the files:
```bash
static-web-server -d dist/ -p 8080
ran -r dist/
```
If you can set up rewrite urls, you can redirect all links to this app.

### Nix

The nix flake will create a derivation with the contents of dist copied to the nix store, and a script to serve the 
files. Run `nix build` to build the website, and `nix run` to serve (using `ran`).

## Acknowledgements Etc

[This flake for the yew template](https://github.com/OliverEvans96/yew-trunk-minimal-template).

[trunk]: https://github.com/thedodd/trunk
[yew]: https://github.com/yewstack/yew
