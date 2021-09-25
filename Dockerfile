FROM rust:1.55

WORKDIR /usr/src/cosyne
COPY . .

RUN cargo install --path .
