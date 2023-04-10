# # # # # # #
# variables
#

rust_flags_release := RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables"

rust_log_debug := RUST_LOG=warn,bevy=debug,hen_rescue_hero=debug,hrh_game=debug,brp_game_base=debug

# # # # # # # # # # #
# initial commands
#

setup:
	rustup default stable
	cargo install --locked wasm-bindgen-cli # required by `trunk`
	cargo install --locked trunk # https://trunkrs.dev/

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_host_debug

web: run_web_debug

dist: dist_itch_io

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	trunk clean
	trunk --config ./Trunk.release.toml clean
	trunk --config ./Trunk.itch_io.toml clean
	rm -rf ./dist/
	cargo clean

test:
	cargo test --workspace
	cargo test --workspace --release

clippy:
	cargo clippy --workspace
	cargo clippy --workspace --release
	cargo clippy --workspace --target wasm32-unknown-unknown
	cargo clippy --workspace --target wasm32-unknown-unknown --release
	cargo clippy --workspace --profile test

visualize_schedule:
	# --quiet: required to have only schedule graph copied to the clipboard
	cargo run --package hrh_visualize_schedule --features bevy_dynamic_linking --quiet | pbcopy
	echo "Graph data is in your clipboard now. Visit https://edotor.net/ and paste it there 🙂"

# # # # # # # # # #
# build commands
#

build_host_release:
	$(rust_flags_release) cargo build --release
	cp -R ./assets/ ./target/release/assets/

# # # # # # # # #
# run commands
#

run_host_debug:
	$(rust_log_debug) cargo run --features hrh_game/bevy_dynamic_linking

run_host_release: build_host_release
	./target/release/hen_rescue_hero

run_web_debug:
	mkdir -p ./dist/
	$(rust_log_debug) trunk serve

run_web_release:
	mkdir -p ./dist/
	$(rust_flags_release) trunk --config ./Trunk.release.toml serve

# # # # # # # # #
# dist commands
#

dist_itch_io:
	mkdir -p ./dist/
	trunk --config ./Trunk.itch_io.toml clean
	$(rust_flags_release) trunk --config ./Trunk.itch_io.toml build
	rm -f ./dist/hen_rescue_hero__itch_io.zip
	rm -rf ./dist/hen_rescue_hero__itch_io/ # in case ZIP was extracted there
	cd ./dist/itch_io/ && zip -r ../hen_rescue_hero__itch_io.zip ./
	echo "✅ Dist package is ready: ./dist/hen_rescue_hero__itch_io.zip"
