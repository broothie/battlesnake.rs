FROM rust:1.60 AS builder

WORKDIR /usr/src/battlesnake
COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/battlesnake/target/release/battlesnake-rs battlesnake-rs

CMD ./battlesnake-rs -p $PORT --food-coefficient ${FOOD_COEFFICIENT:-1.5}
