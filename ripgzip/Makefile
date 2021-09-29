include ../build/MakefileBase

all: build base_no_miri test_py ok

build:
	cargo build
	cargo build --release

test_py:
	cargo test
	./test.py