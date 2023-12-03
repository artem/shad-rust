include ../build/MakefileBase

PLAYER_TWO= aggressive.wasm
PLAYER_THREE= coward.wasm
PLAYER_FOUR= fool.wasm

all: base check_deps test_games

test_games: check_deps build_wasm_launcher build_bot_release
	@for i in 1 2 3; do \
		echo "Running game $${i}"; \
		OUTPUT=$$(python3 ./tools/runner/localrunner.py --no-gui \
			-p1 ../target/release/paperio-bot \
			-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
			-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
			-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"); \
		if [ "$$(echo $$OUTPUT | grep "winner_id:player1")" ]; then \
			echo "Game $${i} is OK"; \
		else \
			echo "Game $${i} is lost :("; \
			exit 1; \
		fi; \
	done
	@echo "OK!"

check_deps:
	@python3 -c 1 || (echo "ERROR: python3 is not installed."; exit 1)
	@python3 -c 'import pyglet' 2> /dev/null || (echo "ERROR: pyglet python library is not installed. On Ubuntu 23.04 you can install it via 'sudo apt install python3-pyglet'."; exit 1)
	@python3 -c 'import networkx' 2> /dev/null || (echo "ERROR: networkx python library is not installed. On Ubuntu 23.04 you can install it via 'sudo apt install python3-networkx'."; exit 1)

build_bot:
	cargo build --bin paperio-bot

build_bot_release:
	cargo build --bin paperio-bot --release

build_wasm_launcher:
	cargo build --release --manifest-path ./tools/wasm-launcher/Cargo.toml

play_keyboard: check_deps build_wasm_launcher
	python3 ./tools/runner/localrunner.py \
		-p1 keyboard \
		-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
		-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
		-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"

play: check_deps build_wasm_launcher build_bot_release
	python3 ./tools/runner/localrunner.py \
		-p1 ../target/release/paperio-bot \
		-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
		-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
		-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"

play_debug: check_deps build_wasm_launcher build_bot
	MAX_EXECUTION_TIME=10000 REQUEST_MAX_TIME=10000 python3 ./tools/runner/localrunner.py \
		-p1 tcp \
		-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
		-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
		-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"

play_nogui: check_deps build_wasm_launcher build_bot_release
	python3 ./tools/runner/localrunner.py --no-gui \
		-p1 ../target/release/paperio-bot \
		-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
		-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
		-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"

play_wasm: check_deps build_wasm_launcher
	@if [ ! "${PLAYER_ONE}" ]; then \
		echo "Error: PLAYER_ONE is undefined"; \
		exit 1; \
	fi
	python3 ./tools/runner/localrunner.py \
		-p1 "../target/release/wasm-launcher bots/${PLAYER_ONE}" \
		-p2 "../target/release/wasm-launcher bots/${PLAYER_TWO}" \
		-p3 "../target/release/wasm-launcher bots/${PLAYER_THREE}" \
		-p4 "../target/release/wasm-launcher bots/${PLAYER_FOUR}"
