# Ferricyanide

![Integration](https://github.com/frodofine/ferricyanide/workflows/Integration/badge.svg)

Molecular viewer targeting WebAssembly (WASM).

**Ferricyanide** is available under the MIT/Apache-2.0 licenses. It is based off of source code published by [Chinedu Francis Nwafili](https://github.com/chinedufn/webgl-water-tutorial). His project is used as an extendable base (most notably the canvas and shader systems).

## Goals

This project has no real goals other than as an experiment in WASM and WebGL development. For this experiment, the following sub-goals should be followed.

### Extendability

### Code size

Since WASM is the target, compiled code size is a consideration. Currently, the release mode *.wasm* file clocks in under 100Kb without compression, and ideally it should not grow beyond 500Kb. Therefore, features which add significant code size should be made optional using build configurations. Additionally, libraries which increase code-size significantly should be avoided.

## How to build

First, install [rustup](https://rustup.rs/) for your respective operating system and make sure `cargo` is in your `PATH`. If you are on Linux or macOS, clone the project and built it using the following commands. On Windows, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and clone the directory.

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
git clone https://github.com/frodofine/ferricyanide.git
cd ferricyanide
```

To build with `cargo-make`, use the following commands in the *ferricyanide* directory. Note that the use of `cargo-make` is optional, but this tool is recommended for development as `cargo-make` can host a local webserver and watch for changes as to rebuild the project automatically.

```bash
cargo install cargo-make
cargo make build
# Watch for changes in the source code and auto rebuild
cargo make watch
```

Alternatively, you can use `wasm-pack` directly. The `--target web` option causes *ferricyanide* to be built as an **ES6** module without the need to use *webpack* to deploy the application in a browser. Other options are available and should be supported without modification to the **Rust** code. See [the official documentation](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html) for details.

```bash
wasm-pack build --target web --out-name package
```

This will populate the `pkg` directory with files starting with *package*. These files are required to deploy the viewer on a server. You can host a local server with the following command:

```bash
cargo make start
```

If done in the default *ferricyanide* directory, then it will host a simple demo application at [localhost:8000](http://localhost:8000).

## Why the name

Ferricyanide is a compound used in photo development to add a sepia tone. It is a **Rust** colored *compound*, so it's a perfect name for a package that visualizes molecules.
