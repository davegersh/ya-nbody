# Yet Another N-Body Simulation

![Two colliding galaxies made up of 15,000 particles](/samples/galaxy_collision.gif)

An interactive n-body simulation created in Rust using Macroquad and egui.

Makes use of a (lightly) multithreaded implenentation of the [Barnes-Hut](https://en.wikipedia.org/wiki/Barnes%E2%80%93Hut_simulation) algorithm to effectively simulate tens of thousands of gravitational interactions between particles.

Makes use of TOML files to define initial conditions of the simulation, sample configs can be find in the `/samples` folder.
the gif above uses the config located at `samples/galaxy_collision.toml`.

### Installation
First, make sure to have rust and cargo installed. If not, instructions can be found [here](https://www.rust-lang.org/tools/install).

Next, simply clone this repo to a desired location, and run the following command:
```
cargo run --release [example.toml]
```
Replace `[example.toml]` with the path to your own file path as a config for the simulation.