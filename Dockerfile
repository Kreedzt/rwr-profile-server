# build
FROM ekidd/rust-musl-builder:latest as build

WORKDIR /rwr-profile-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --locked --release
RUN mkdir -p build-out/
RUN cp target/x86_64-unknown-linux-musl/release/rwr-profile-server build-out/

# run
# FROM rust:1.61.0-slim-buster
FROM scratch
WORKDIR /app

COPY --from=build /rwr-profile-server/build-out/rwr-profile-server .
COPY ./config_example.json ./config.json

EXPOSE 8080

CMD ["./rwr-profile-server"]
