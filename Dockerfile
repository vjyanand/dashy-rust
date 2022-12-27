FROM alpine:latest

RUN apk add --update --no-cache --repository http://dl-3.alpinelinux.org/alpine/edge/community --repository http://dl-3.alpinelinux.org/alpine/edge/main rust cargo libpq-dev

WORKDIR /opt/dashy

COPY ./Cargo.toml ./Cargo.toml

ADD . ./

ENV DATABASE_URL=postgres://dashy_api:K9xtuYfIQWXFgeq@top2.nearest.of.dashy-api-db.internal:5432/dashy_api?sslmode=disable

#postgres://postgres:eEFAQfADEpqlnh4@dashy-db.internal:5432

#DATABASE_URL=postgres://dashy_web:hI4lc5mWrme8KXr@top2.nearest.of.dashy-db.internal:5432/dashy_web

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/dashy"]