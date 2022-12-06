FROM rust:1.65-bullseye AS builder
COPY ./Cargo.lock ./Cargo.toml /
RUN echo "fn main() {}" >> /dummy.rs
RUN ["cargo", "build", "--release", "--bin", "dummy"]
ADD ./src /src
RUN ["cargo", "build", "--release", "--bin", "server"]

FROM debian:bullseye
COPY --from=builder /target/release/server /
EXPOSE 8080/tcp
CMD ["/server"]
