FROM public.ecr.aws/docker/library/alpine:latest

COPY . /tmp

RUN cd /tmp \
    && apk add --no-cache cargo pkgconf openssl-dev \
    && cargo build --release \
    && apk del cargo pkgconf openssl-dev \
    && apk add --no-cache libssl3 libcrypto3 libgcc \
    && cp ./target/release/mailer /usr/local/bin/ \
    && rm -rf /tmp/* /root/.cargo

CMD [ "mailer" ]
