FROM rust:1.67.0-buster as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Now copy it into our base image.
FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/src/app/target/release/rust-openai-demo /usr/local/bin/rust-openai-demo
CMD ["rust-openai-demo"]
