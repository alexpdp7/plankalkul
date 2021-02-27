# Build PWA

The script `build-pwa` builds the PWA to `target/pwa/`.

It requires Rust, wasm-pack, rollup.

To build using a pre-built Docker image with build dependencies:

```
$ docker run -it --rm -v $(pwd):/app -w /app alexpdp7/rust-wasm-pack-rollup ./build-pwa
```

# Test PWA

This will not work as a PWA, just as a plain PWA, because PWAs must be served with https.

```
$ python3 -m http.server --directory target/pwa/
```
