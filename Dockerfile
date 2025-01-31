FROM rust:1.84-alpine AS builder

WORKDIR /home

RUN apk add --no-cache musl-dev

COPY . ./
RUN cargo build --release

RUN ls /home/target/release
RUN pwd

FROM alpine:3.21 AS run

RUN mkdir -p /var/lib/kindlyrss \
    && mkdir -p /usr/share/kindlyrss

EXPOSE 3000/tcp

COPY --from=builder /home/target/release/kindle-rss-reader /usr/local/bin/kindlyrss
COPY --from=builder /home/templates/ /usr/share/kindlyrss/templates/
COPY --from=builder /home/migrations/ /usr/share/kindlyrss/migrations/
COPY --from=builder /home/static/ /usr/share/kindlyrss/static/
COPY --from=builder /home/config/config.json /var/lib/kindlyrss/config.json

RUN ls /usr/local/bin

ENV RUST_LOG=info
ENV MAX_ARTICLES_QTY_TO_DOWNLOAD=0

CMD ["kindlyrss"]
