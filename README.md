# Using Spin with `wasi-cloud-core` components

This repo demonstrates how Spin can invoke Wasm components built only to the `wasi-cloud-core` draft spec,
and so how `wasi-cloud-core` can act as a portable API for building cloud native components.

The example is a QR generator which, given a URL and target size, generates a SVG code.  Because generating
a QR code can take a few moments, it caches generated codes using `wasi:keyvalue`.

## Structure

The repo contains three directories:

### wit

The `wit` directory contains the interface exposed by the QR generation component. Note this refers only
to QR generation.

### qr-generator-cached

This component implements the QR interface.  It uses `wasi:keyvalue` to cache generated codes. This
is a pure library component. It has no dependencies on Spin or any other WASI implementation.

### website

This is a Spin application which consumes the QR interface, and composes the `qr-generator-cached`
implementation via its `spin.toml`.

It's not a real web site.

## Build and run

1. Build the `qr-generator-cached` library component. `spin.toml` expects to find it in the release
   `wasm32-unknown-unknown` release configuration.  Of course it could equally well take dependencies
   on other WASI specs, e.g. for logging.

```console
cd qr-generator-cached
cargo component build --target wasm32-unknown-unknown --release
cd ..
```

2. Build and run the Spin application in the normal way.

```
spin up --build -f website
```

3. Test the Spin application.

```
# You need the quotes or bash treats &size=100 as some shell incantation. Ask me how I know
curl "localhost:3000/?url=mysite&size=100"
```

You can look at the KV store in `.spin` with a SQLite viewer to confirm that items have been cached.
