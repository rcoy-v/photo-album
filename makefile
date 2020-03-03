MAKE_FLAGS+=--silent
.PHONY: build release test docker fmt

build:
	cargo build

release:
	cargo build --release

test:
	cargo clippy -- -D warnings
	cargo fmt -- --check
	cargo test --features mock

docker:
	docker build -t rcoy-v/photo-album .

fmt:
	cargo fmt
