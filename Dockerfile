FROM rust:1.40 as builder
WORKDIR /usr/src/the_count
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/the_count /usr/local/bin/the_count
ENTRYPOINT ["the_count"]
