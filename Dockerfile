FROM rust:latest AS rs
WORKDIR /app

COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install -f wasm-bindgen-cli

RUN cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/isent_it.wasm

FROM rust:latest AS rss
WORKDIR /app

RUN git clone https://github.com/Loshido/rust_static_server /app
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM alpine:latest AS prod
WORKDIR /app

COPY --from=rss /app/target/aarch64-unknown-linux-musl/release/static_server /bin/static_server
COPY ./assets /app/assets
COPY ./web /app/
COPY --from=rs /app/out /app/js

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="80"
EXPOSE 80
# Rust static server binary
CMD [ "/bin/static_server" ]