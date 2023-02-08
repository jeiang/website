# aidanpinard.co Website

My website, made using [Yew] and built with [Trunk].

## Building & Running

To build & run:
```bash
trunk serve
```

To build only:
```bash
trunk build
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release Build

```bash
trunk build --release
# or 
trunk serve --release
```

This builds the app in release mode similar to `cargo build --release`.

Unless overwritten, the output will be located in the `dist` directory.

## Acknowledgements Etc

[This flake for the yew template](https://github.com/OliverEvans96/yew-trunk-minimal-template).

[trunk]: https://github.com/thedodd/trunk
[yew]: https://github.com/yewstack/yew
