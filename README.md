# bevy-chess-3d
A simple chess game created by using [Bevy](https://bevyengine.org/) following the tutorial at [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial/).

Since the tutorial is straight forward and I'm familiar with rust I decided to update the dependencies:
- Bevy 0.4 → Bevy 0.6
- [bevy_mod_picking 0.3.1](https://github.com/aevyrie/bevy_mod_picking/tree/v0.3.1) → [bevy_mod_picking 0.5.2](https://github.com/aevyrie/bevy_mod_picking/tree/v0.5.2)

Notes when writing this Bevy 0.6 just released so the dependencies were not quite there yet, so to get it running i had to get a local clone of bevy_mod_picking and used `Cargo.toml` to point it to my local clone: 
```toml
bevy_mod_picking = { path = "../path_to_local/clone/bevy_mod_picking" }
```

To stay on the latest edge I also used some nightly featues and had to add `#![feature(int_abs_diff)]` at the top of `main.rs` as well as a `rust-toolchain.toml`:

```toml
[toolchain]
channel = "nightly"
```
