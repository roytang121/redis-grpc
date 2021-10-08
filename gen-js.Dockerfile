FROM amd64/ubuntu:latest

WORKDIR /app

RUN apt update
RUN apt install -y unzip wget

RUN wget -O protoc-3.18.1-linux-x86_64.zip https://github.com/protocolbuffers/protobuf/releases/download/v3.18.1/protoc-3.18.1-linux-x86_64.zip
RUN unzip protoc-3.18.1-linux-x86_64.zip -d ./protoc-bin

RUN wget -O /usr/local/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.2.1/protoc-gen-grpc-web-1.2.1-linux-x86_64

RUN chmod +x /usr/local/bin/protoc-gen-grpc-web

CMD ./protoc-bin/bin/protoc -I=$DIR $PROTO_FILE \
               --js_out=import_style=commonjs:$OUT_DIR \
               --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$OUT_DIR