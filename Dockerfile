FROM alpine:latest

RUN apk add --no-cache cargo pkgconf openssl-dev

RUN cargo build --release \
    && apk del cargo pkgconf openssl-dev \
    && apk add --no-cache libssl3 libcrypto3 libgcc

COPY target/release/mailer /usr/local/bin/

CMD [ "mailer" ]
