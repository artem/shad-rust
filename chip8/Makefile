include ../build/MakefileBase

define run-image =
@-rm -f log 2> /dev/null
@-../target/debug/console-runner $1 2> log
@cat log
@-rm -f log
endef

all: build base_no_miri ok

build:
	cargo build
	cargo build --release

build-runner:
	cargo build --manifest-path ./tools/console-runner/Cargo.toml

run_chip8_logo: build-runner
	$(call run-image,./images/tests/1-chip8-logo.ch8)

run_ibm_logo: build-runner
	$(call run-image,./images/tests/2-ibm-logo.ch8)

run_corax: build-runner
	$(call run-image,./images/tests/3-corax+.ch8)

run_flags: build-runner
	$(call run-image,./images/tests/4-flags.ch8)

run_quirks: build-runner
	$(call run-image,./images/tests/5-quirks.ch8)

run_keypad: build-runner
	$(call run-image,./images/tests/6-keypad.ch8)

run_breakout: build-runner
	$(call run-image,./images/games/breakout.ch8)
