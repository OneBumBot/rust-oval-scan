cli: 
    cargo run -p cli

flamegraph:
	RUSTFLAGS='-C force-frame-pointers=yes' cargo flamegraph --profile profiling -p cli -F 999

clean-flamegraph:
	rm -f flamegraph.svg perf.data perf.data.old

clean: 
    cargo clean

test-all:
    cargo test --workspace
