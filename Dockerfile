FROM messense/rust-musl-cross:x86_64-musl as build

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release --locked & rm ./src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release --locked

# our final base
FROM scratch

# copy the build artifact from the build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rwr-profile-server .
COPY ./config_example.json ./config.json

EXPOSE 80

# set the startup command to run your binary
CMD ["./rwr-profile-server"]
