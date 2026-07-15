FROM rust:1.88-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p dnz-cli

FROM debian:bookworm-slim
COPY --from=build /src/target/release/dnz-cli /usr/local/bin/dnz
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/dnz"]
