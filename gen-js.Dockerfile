FROM ubuntu:latest

RUN apt update
RUN apt install -y protobuf-compiler

COPY ./proto/protoc-gen-grpc-web-1.2.1-linux-x86_64 /usr/local/bin/protoc-gen-grpc-web
RUN chmod +x /usr/local/bin/protoc-gen-grpc-web

CMD protoc -I=$DIR $PROTO_FILE \
               --js_out=import_style=commonjs:$OUT_DIR \
               --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$OUT_DIR