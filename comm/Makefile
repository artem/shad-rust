include ../build/MakefileBase

all: build base_no_miri_no_clippy ok

build:
	cargo build
	cargo build --release

bench: ../target/debug/comm_cpp ../target/release/comm_cpp
	cargo criterion

../target/debug/comm_cpp:
	clang++ src/main.cpp -o ../target/debug/comm_cpp -O0 -g

../target/release/comm_cpp:
	clang++ src/main.cpp -o ../target/release/comm_cpp -O3