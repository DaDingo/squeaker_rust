FROM rust:1.65 

WORKDIR /usr/src/squeaker

COPY . .

RUN cargo build