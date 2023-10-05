<p align="center">
  <picture>
  <img src="css-knife.png" widht="130" alt="Logo for css-knife"/>
</p>

<h1 align="center">
  css-knife
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

Configuration(*css-knife.toml*):

```toml
html_dir = ["web/src/templates"]
css_dir = ["web/src/css"]
js_dir = ["web/src/js"]
svg_dir = ["web/src/svg"]
output_dir = "prod"
```

## Dependencies:

  - [lightningcss](https://crates.io/crates/lightningcss)

  - [lol-html](https://crates.io/crates/lol_html)

## Installation

```bash
cargo install --path=. --force
```
