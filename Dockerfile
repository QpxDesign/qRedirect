FROM rust:1.81.0

EXPOSE 3981

WORKDIR /app
COPY . .

RUN cargo build

CMD ["cargo", "run"]
