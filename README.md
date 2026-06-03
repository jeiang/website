# Website

This is my website, made using Astro and Tailwind. A Nix Flake is provided for developing and running the website.

## Commands

All commands are run from the root of the project, from a terminal:

| Command               | Action                                           |
| :-------------------- | :----------------------------------------------- |
| `bun install`         | Installs dependencies                            |
| `bun dev`             | Starts local dev server at `localhost:4321`      |
| `bun build`           | Build your production site to `./dist/`          |
| `bun preview`         | Preview your build locally, before deploying     |
| `bun astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `bun astro -- --help` | Get help using the Astro CLI                     |

## Docker

The production image builds the Astro site with Bun, then serves `dist/` with nginx:

```sh
docker build -t website .
docker run --rm -p 8080:80 website
```

Pushes to `main` and version tags matching `v*.*.*` publish the image to GitHub Container Registry as `ghcr.io/<owner>/<repo>`.

## Served From

- [jeiang.dev](https://jeiang.dev)
- [pinard.co.tt](https://pinard.co.tt)
- [aidanpinard.co](https://aidanpinard.co)
