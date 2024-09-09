# neophys
[![Crates.io Status](https://img.shields.io/crates/v/neophys)](https://img.shields.io/crates/v/neophys)

Neophys is a 2D physics simulator that is implemented with the force of gravity. The core of the program itself relies on the RK4 algorithm to run the simulation, which is coded from scratch. As well, the program is coded in Rust, through the Nannou graphics library.

## Implementations
- [x] Force of gravity
- [x] Collision detection with walls (partially)
- [ ] Collision detection with multiple bodies
- [ ] Proper impulse processing
- [ ] Moving bodies through the mouse

## To Build:
- Install cargo through rustup.
```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```
- Run `cargo run` inside the directory.
```bash 
cd neophys/
cargo run
```
