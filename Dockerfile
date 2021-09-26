FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
RUN apt-get update -qq \
  && apt-get install -y \
  apt-transport-https \
  ca-certificates \
  libpq-dev \
  && apt-get update -qq \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/vproxy /usr/local/bin/vproxy
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["vproxy"]
