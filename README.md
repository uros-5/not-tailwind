<div align="center">
  <a href="https://github.com/uros-5/not-tailwind#gh-light-mode-only"><img src="assets/logo.svg#gh-light-mode-only"        width="300px" alt="not-tailwind logo"/></a>
  <a href="https://github.com/uros-5/not-tailwind#gh-dark-mode-only"><img src="assets/logo.darkmode.svg#gh-dark-mode-only" width="300px" alt="not-tailwind logo"/></a>
  <br>
  <a href="https://crates.io/crates/not-tailwind"><img alt="crates.io" src="https://img.shields.io/crates/v/not-tailwind.svg?style=for-the-badge&color=fdbb39&logo=rust" height="20"></a>
  <a href="https://github.com/uros-5/not-tailwind/actions?query=branch%3Amain"><img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uros-5/not-tailwind/build.yml?branch=main&style=for-the-badge&logo=github" height="20"></a>
</div>

<h1 align="center">
  not-tailwind
</h1>


<p align="center">
  Shorten those long TailwindCSS classes
</p>

This is your HTML in development:
```html
  <div class="text-red-600 px-2 bg-green-200 md:text-md lg:text-lg xl:text-xl custom-font dark:text-green-200 dark:bg-red-500">
    Hello World
  </div>
```

And here is in production:
```html
  <div class="c b a g h i custom-font f d">
    Hello World
  </div>
```

Usage: not-tailwind

```
Options:
  -r, --run <RUN>...          Build for production. Specify in which files to search
      --build-ts-map          Build TypeScript map
  -o, --output <OUTPUT>       Specify output directory(default directory is not-tailwind). Make sure it is in .gitignore
  -i, --ignored <IGNORED>...  CSS files to ignore
  -h, --help                  Print help
  -V, --version               Print version
```

```sh
not-tailwind --build-ts-map --run html ts j2 --ignored ./public/main.css
```

## Dependencies:

  - [lightningcss](https://crates.io/crates/lightningcss)

  - [lol-html](https://crates.io/crates/lol_html)

## Installation

```bash
cargo install --path=. --force
```
