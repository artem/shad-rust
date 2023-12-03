include ../build/MakefileBase

CARGO_NET_FLAGS= --no-default-features --features net

all: build base_no_miri ok

build:
	cargo build
	cargo build --release

net: build_net lint_net test_net ok

build_net:
	cargo build ${CARGO_NET_FLAGS}
	cargo build --release ${CARGO_NET_FLAGS}

lint_net:
	cargo clippy ${CARGO_NET_FLAGS}
	@if [ "$$(cargo clippy ${CARGO_NET_FLAGS} 2>&1 | grep -v Finished | grep -v Checking | grep -v Blocking)" ]; \
		then echo "ERROR: 'cargo clippy' found some issues (see above)"; exit 1; \
	fi

test_net:
	cargo test ${CARGO_NET_FLAGS}
	cargo test --release ${CARGO_NET_FLAGS}

submit_net:
	cargo run --manifest-path ../tools/submit/Cargo.toml -- --task-path . --subtask net
	@echo 'OK!'
