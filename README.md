## Running
Run `cargo run -- objects/hourglass.obj`

## Performance testing
To generate and open a flamegraph:
- Open a terminal window as administrator
- Change working directory to the root of this repository
- Run 
`cargo flamegraph --output flamegraphs/graph-1.svg -- objects/hourglass.obj && start flamegraphs/graph-1.svg`