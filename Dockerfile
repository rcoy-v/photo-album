FROM rust:1 as builder
WORKDIR /usr/src/photo-album
COPY . .

RUN useradd docker -N && chown -R docker .
USER docker

RUN rustup component add clippy-preview && cargo clippy -- -D warnings
RUN rustup component add rustfmt && cargo fmt -- --check
RUN cargo test --features mock
RUN cargo install --path .

ENTRYPOINT ["photo-album"]

