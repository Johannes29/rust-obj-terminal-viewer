if not exist flamegraphs ( mkdir flamegraphs )
cargo flamegraph --output flamegraphs/graph-1.svg -- objects/hourglass.obj && start flamegraphs/graph-1.svg