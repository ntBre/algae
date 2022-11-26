clippy:
	cargo clippy --tests

test:
	cargo test -- --nocapture

run:
	cargo run demo.ivy
