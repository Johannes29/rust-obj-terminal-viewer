# Rust Obj Terminal Viewer

A CLI program to view 3D models directly in the terminal. Supports .obj files.

This program uses no graphics library.
Perspective transformations, rasterization, etc are coded by hand.
The 3D object is displayed in the terminal with ascii-characters.

## Getting started

- Clone this repository.
- Make sure Rust is installed. If not, [install it](https://www.rust-lang.org/tools/install).
- `cd` into the cloned repository.

## How to use

### Installing the program

Make sure to follow [Getting started](#getting-started) first.

Open a terminal and run `cargo install --path ./`.

### Using the program

Open a terminal and run `rust-obj-terminal-viewer FILE_PATH_HERE`,
but replace `FILE_PATH_HERE` with the path to the file you want to view.
Sample files can be found in [the objects folder](/objects/).

The object should now be displayed in the terminal.

To rotate the object, move the mouse with the left mouse button pressed.
To close the program, press `q`.

## Development

Make sure to follow [Getting started](#getting-started) first.

### Running

Run `cargo run -- PATH_TO_OBJ_FILE_HERE`. Replace `PATH_TO_OBJ_FILE_HERE` with
a path to a `.obj` file that you wish to view within the program.

Sample files can be found in [the objects folder](/objects/).
Alternatively, run `cargo rhg` to open the program with `hourglass.obj`.

### Performance testing

To run a benchmark, run `cargo bench --bench BENCHMARK_NAME_HERE`,
but replace `BENCHMARK_NAME_HERE` with the name of a benchmark.
The benchmarks are defined in [the benchmarks folder](/benchmarks/).
Each file in the folder represents a benchmark.
The benchmark name is the name of the file without the `.rs` file extension.

### Performance profiling

![An example of what the generated flamegraph can look like](flamegraph_example.png)
To generate a flamegraph:

- Make sure that flamegraph is installed. You can test this by
  running `cargo flamegraph --version`. To install flamegraph, follow
  [these instructions](https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#installation).
- If on windows, make sure that you have opened your terminal as administrator.
- Change working directory to the root of this repository
- Run `cargo flamegraph -- PATH_TO_OBJ`, where `PATH_TO_OBJ` is a path to
  the `.obj` file you want to render, for example `objects/hourglass.obj`
- Wait a few seconds and then press `q`

The generated flamegraph should now be in the root of this repository, named `flamegraph.svg`.

## Known issues and workarounds

- The program might look weird if you change the size of the terminal window while the program is runnning.

  Workaround: Restart the program.

- The cursor will remain hidden if the program crashes.

  Workaround: Restart the terminal.
