# Yet Another N-Body Simulation

![Two colliding galaxies made up of 15,000 particles](/samples/galaxy_collision.gif)

An interactive n-body simulation created in Rust using Macroquad and egui.

Makes use of a (lightly) multithreaded implenentation of the [Barnes-Hut](https://en.wikipedia.org/wiki/Barnes%E2%80%93Hut_simulation) algorithm to effectively simulate tens of thousands of gravitational interactions between particles.

Initial conditions of the simulation are defined using TOML files, sample configs can be found in the [`/samples`](/samples) directory.
The gif above uses the config located [here](samples/galaxy_collision.toml).

### Building
First, make sure to have rust and cargo installed. If not, instructions can be found [here](https://www.rust-lang.org/tools/install).

Next, simply clone this repo to a desired location, and run the following command:
```
cargo run --release [example.toml]
```
Replace `[example.toml]` with the path to your own toml config file.

#### NixOS
Before using the command above, make use of the [`shell.nix`](shell.nix) file by running:
```
nix-shell
```
This starts a development environment where Ya-nbody can be built and run.