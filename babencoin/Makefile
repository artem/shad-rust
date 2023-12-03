include ../build/MakefileBase

all: build clear_artifacts base_no_miri ok

build:
	cargo build
	cargo build --release

clear_artifacts:
	mkdir -p test_artifacts
	-rm -rf test_artifacts/*
