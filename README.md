# barnes-hut

The goal of this project is to implement the barnes-hut algorithm to simulate gravitational dynamics.

This algorithm is built using the quad-tree data structure.

A nannou crate is used as the graphics framework.

### TODO [maybe!]

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
- [ ] wasm build
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
