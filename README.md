# barnes-hut

The goal of this project is to implement the barnes-hut algorithm to simulate gravitational dynamics.

This algorithm is built using the quad-tree data structure.

A nannou crate is used as the graphics framework.

## Building & usage

### Prerequisites

- Rust toolchain (tested with rustc 1.68.2 (9eb3afe9e 2023-03-27))
- For webassembly build: [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Desktop

Build & run with:

```shell
cargo run --release
```

### Webassembly

Build with:

```shell
wasm-pack build --target web
```

Then serve [index.html](./index.html) and the [pkg](./pkg) directory with a webserver of your choice, e.g.

```shell
npx http-server -o
```

or the `Run wasm in chrome` CLion / IntelliJ run configuration.

## TODO [maybe!]

(In no particular order)

#### Physics simulation

- [ ] Implement a better integration scheme,
  e.g. https://en.wikipedia.org/wiki/Leapfrog_integration / https://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet
- [ ] Work out nice physics params for a stable system
- [ ] Relate in-sim dimensions (space & time) to real-world dimensions, use real G
- [ ] Collision detection & handling
- [ ] 3D implementation

#### UI / usability

- [ ] Draw tree bounding boxes again
    - [ ] Better: highlight bodies and bounding boxes contributing to g calculation at the mouse location
- [ ] Easy way to tune parameters at runtime, e.g. https://github.com/tversteeg/const-tweaker
    - [ ] Extract a `Config` struct - possibly separating physics and app / graphics config
- [X] wasm build
    - [ ] Fix: Logging in wasm - want to control log level per package as with env_logger
    - [ ] Fix: mouse wheel zoom
    - [ ] Fill browser frame
    - [ ] Build & deploy somewhere with GitHub actions
- [X] Pan with middle mouse button
    - [ ] Touch controls?
- [ ] Some interface for adding / removing bodies

#### Code structure / implementation details

- [ ] Tidy up physics code - `Space` trait & its `2D` and `3D` implementations could be a single source of truth for '
  laws of physics, e.g. moving general purpose gravity calcs out of barnes_hut.rs.
- [ ] Grow tree bounds dynamically
- [ ] Move away from nannou? E.g. to

#### Testing

- [ ] Think up some 'physics sim fidelity' tests
- [ ] Benchmark[s] to measure perf impacts of changes
