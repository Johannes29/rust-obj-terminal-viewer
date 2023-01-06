## Running
Run `cargo run`

## Performance testing
To generate and open a flamegraph:
- Open a terminal window as administrator
- Change working directory to the root of this repository
- Run 
`cargo flamegraph --output flamegraphs/flamegraph.svg -- objects/hourglass.obj && start flamegraphs/flamegraph.svg`