# Ferricyanide

Molecular viewer targeting WebAssembly (WASM).

**Ferricyanide** is available under the MIT/Apache-2.0 licenses. It is based off of source code published by [Chinedu Francis Nwafili](https://github.com/chinedufn/webgl-water-tutorial). His project is used as an extendable base (most notably the canvas and shader systems).

## Goals

This project has no real goals other than as an experiment in WASM and WebGL development. For this experiment, the following sub-goals should be followed.

### Extendability

### Code size

Since WASM is the target, compiled code size is a consideration. Currently, the release mode *.wasm* file clocks in under 100Kb, and ideally it should not grow beyond 500Kb. Therefore, features which add significant code size should be made option using build configurations. Additionally, libraries which increase code-size significantly should be avoided.

## How to build

First, install [rustup](https://rustup.rs/) for your respective operating system and make sure `cargo` is in your `PATH`. Then, clone the project and built it using the following commands:

```bash
cargo install cargo-make
git clone https://github.com/frodofine/ferricyanide.git
cd ferricyanide
cargo make build
```

This will populate the `pkg` directory with files starting with *ferricyanide*. These files are required to deploy the viewer on a server. You make host a local server with the following command:

```bash
cargo make serve
```

If done in the default *ferricyanide* directory, then it will host a simple demo application at [localhost:8000](http://localhost:8000).

## Why the name

Ferricyanide is a compound used in photo development to add a sepia tone. It is a **Rust** colored *compound*, so it's a perfect name for a package that visualizes molecules.
