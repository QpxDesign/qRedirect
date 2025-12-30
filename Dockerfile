# BUILD
FROM rust:1.89.0 AS builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# PROD
EXPOSE 3981
FROM gcr.io/distroless/cc
COPY --from=builder /app/assets /assets
COPY --from=builder /app/target/release/qRedirect /
CMD ["./main"]
