if not exist flamegraphs ( mkdir flamegraphs )
cargo flamegraph --output flamegraphs/graph-1.svg -- objects/Tree1.obj && start flamegraphs/graph-1.svg