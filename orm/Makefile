include ../build/MakefileBase

all: base_no_miri test_lifetimes ok

test_lifetimes:
	@if [ -z "$$(cargo test --features test_lifetimes_create 2>&1 | grep 'error\[E0505\]')" ]; then \
		echo "ERROR: 'cargo test --features test_lifetimes_create' should fail"; \
		exit 1; \
	fi
	@if [ -z "$$(cargo test --features test_lifetimes_get 2>&1 | grep 'error\[E0505\]')" ]; then \
		echo "ERROR: 'cargo test --features test_lifetimes_get' should fail"; \
		exit 1; \
	fi