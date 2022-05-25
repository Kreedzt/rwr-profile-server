# build
FROM rust:1.61.0-slim-buster as build

RUN USER=root cargo new --bin rwr-profile-server
WORKDIR /rwr-profile-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/rwr_profile_server*
RUN cargo build --release

# run
FROM rust:1.61.0-slim-buster

COPY --from=build /rwr-profile-server/target/release/rwr-profile-server .
COPY ./config.json ./config.json

EXPOSE 8080

CMD ["./rwr-profile-server"]
