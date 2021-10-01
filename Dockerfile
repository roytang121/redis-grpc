FROM alpine:latest

RUN apk update
RUN apk add curl
RUN apk add alpine-sdk
RUN apk add protoc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN source $HOME/.cargo/env

RUN ~/.cargo/bin/cargo install redis-grpc

ENTRYPOINT ~/.cargo/bin/redis-grpc
