FROM rust:1.94-bookworm@sha256:6ae102bdbf528294bc79ad6e1fae682f6f7c2a6e6621506ba959f9685b308a55 AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p dnz-cli

FROM debian:bookworm-slim@sha256:734a72b3d6142a71993b4bc580ca01d8ca687f77a2c422d96a961d3376b179bc
COPY --from=build /src/target/release/dnz-cli /usr/local/bin/dnz
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/dnz"]
