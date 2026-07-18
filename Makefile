.PHONY: flamegraph

flamegraph:
	RUSTFLAGS='-C force-frame-pointers=yes' \
    cargo flamegraph --profile profiling -p cli -F 999

.PHONY: clean-flamegraph

clean-flamegraph:
	rm -f flamegraph.svg perf.data perf.data.old
