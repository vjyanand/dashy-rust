FROM alpine:latest

RUN apk add --update --no-cache --repository http://dl-3.alpinelinux.org/alpine/edge/community --repository http://dl-3.alpinelinux.org/alpine/edge/main rust cargo libpq-dev

WORKDIR /opt/dashy

COPY ./Cargo.toml ./

# Dummy src for deps
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build deps with target (caches musl artifacts)
RUN cargo build --release && rm -rf src

ADD . ./

RUN touch src/main.rs && cargo build --release

EXPOSE 8080

ENV DATABASE_URL='postgresql://neondb_owner:npg_VtZWh36FuBzN@ep-damp-scene-ad3mamku-pooler.c-2.us-east-1.aws.neon.tech/neondb?sslmode=require'

ENV RUST_LOG='info'

CMD ["./target/release/dashy"]