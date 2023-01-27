<div class="oranda-hide">
<h1>oranda</h1>
</div>

> 🎁 generate beautiful landing pages for your projects

[![crates.io](https://img.shields.io/crates/v/oranda.svg)](https://crates.io/crates/oranda)
[![CI](https://github.com/axodotdev/oranda/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/axodotdev/oranda/actions/workflows/ci.yml)
[![release](https://github.com/axodotdev/oranda/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/axodotdev/oranda/actions/workflows/release.yml)
[![web](https://github.com/axodotdev/oranda/actions/workflows/web.yml/badge.svg?branch=main)](https://github.com/axodotdev/oranda/actions/workflows/web.yml)

`oranda` is an opinionated static-site generator that is designed for developers
who are publishing projects and would like a website but don't want to build
one from scratch.

<div class="oranda-hide">

`oranda` uses `oranda` so you can checkout a live example [here][`oranda` website]!

## Installation

To install `oranda`, please visit the [`oranda` website]- which is generated by
`oranda`!

[`oranda` website]: https://axodotdev.github.io/oranda

</div>

## Quickstart

```sh
# build your site
> oranda build

# start a server to checkout a local version of your built site in a browser
> oranda serve
```

## Configuration

If you'd like to configure `oranda`, place an `oranda.json` file in the root of
your project and fill it with the configuration you'd like. Check out the [docs]
to learn more about your configuration options!

[docs]: TBD

## Installers: integrating with `cargo-dist`

`oranda` is built to work alongside [`cargo-dist`], which is a tool that builds
distributable artifacts for your Rust applications. To tell `oranda` you are
using `cargo-dist` you can add this to your `oranda.json`:

```json
"artifacts": {
    "cargo-dist": true
}
```

This will link `oranda` and `cargo-dist` such that `oranda` can display your
installers and downloadable artifacts on your page.

[`cargo-dist`]: https://github.com/axodotdev/cargo-dist
