# # # # # # #
# variables
#

rust_flags_release := RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables"

rust_log_debug := RUST_LOG=warn,bevy=debug,bevy_pixels_web_game_poc=debug

# # # # # # # # # # #
# initial commands
#

setup:
	rustup default stable

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_host_debug

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	cargo clean

test:
	cargo test --workspace
	cargo test --workspace --release

clippy:
	cargo clippy --workspace
	cargo clippy --workspace --release
	cargo clippy --workspace --profile test

# # # # # # # # # #
# build commands
#

build_host_release:
	$(rust_flags_release) cargo build --release

# # # # # # # # #
# run commands
#

run_host_debug:
	$(rust_log_debug) cargo run --features hrh_game/bevy_dynamic_linking

run_host_release: build_host_release
	./target/release/hen_rescue_hero
