FROM rust:1.76-slim-bookworm AS builder

#Setting work directory

WORKDIR /app

COPY ./target/release/stridespot_email_system /usr/local/bin/stridespot_email_system

CMD ["./target/release/stridespot_email_system"]