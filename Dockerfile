from docker.io/rust:alpine as builder
run apk add --no-cache musl-dev
workdir /usr/src/app
copy . .
run cargo install --path .

from alpine:latest
copy --from=builder /usr/local/cargo/bin/misskey-booru-rs /usr/local/bin/misskey-booru-rs

entrypoint /usr/local/bin/misskey-booru-rs
