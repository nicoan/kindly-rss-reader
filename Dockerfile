FROM rust:1.84-alpine AS builder

WORKDIR /home

RUN apk add --no-cache musl-dev

COPY . ./
RUN cargo build --release

FROM alpine:3.21 AS run

RUN mkdir -p /home/kindlyrss/static_data \
    && mkdir -p /home/kindlyrss/data

EXPOSE 3000/tcp

COPY --from=builder /home/target/release/kindle-rss-reader /usr/local/bin/kindlyrss
COPY --from=builder /home/templates/ /home/kindlyrss/static_data/templates/
COPY --from=builder /home/migrations/ /home/kindlyrss/static_data/migrations/
COPY --from=builder /home/static/ /home/kindlyrss/static_data/static/
COPY --from=builder /home/config/config.json /home/kindlyrss/data/config.json

RUN ls /usr/local/bin

ENV RUST_LOG=info
ENV MAX_ARTICLES_QTY_TO_DOWNLOAD=0
ENV STATIC_DATA_PATH=/home/kindlyrss/static_data
ENV DATA_PATH=/home/kindlyrss/data

CMD ["kindlyrss"]
