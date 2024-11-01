# Introduction

This repository contains Plankalkul.
Plankalkul consists of:

* A Nom-based mathematical expression parser
* A mathematical expression evaluator using the `BigRational` type from the num crate.

The additional `plankalkul-pwa` contains a PWA implemented using Yew.

`plankalkul-pwa` has a public deployment at https://alexpdp7.github.io/plankalkul/

I find this application works better than many built-in calculator apps on phones with physical keyboards such as the Titan Pocket or BlackBerry KeyONE.

## Hacking the PWA

The implementation is based on https://github.com/fkohlgrueber/yew-pwa-minimal

### Building

The script `build-pwa` builds the PWA to `target/pwa/`.

It requires Rust, wasm-pack, rollup.

To build using a pre-built container image with build dependencies:

```
$ podman run --security-opt label=disable -it --rm -v $(pwd):/app -w /app docker.io/alexpdp7/rust-wasm-pack-rollup ./build-pwa
```

### Testing

This will not work as a PWA, just as a plain webapp, because PWAs must be served with https.

```
$ python3 -m http.server --directory target/pwa/
```
