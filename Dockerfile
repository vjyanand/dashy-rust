FROM alpine:latest

RUN apk add --update --no-cache --repository http://dl-3.alpinelinux.org/alpine/edge/community --repository http://dl-3.alpinelinux.org/alpine/edge/main rust cargo libpq-dev

WORKDIR /opt/dashy

COPY ./Cargo.toml ./Cargo.toml

ADD . ./

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/dashy"]