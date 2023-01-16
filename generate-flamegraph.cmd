if not exist myDirName ( mkdir myDirName )
cargo flamegraph --output flamegraphs/graph-1.svg -- objects/hourglass.obj
start flamegraphs/graph-1.svg