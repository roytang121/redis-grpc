FROM amd64/ubuntu:latest

WORKDIR /app

RUN apt update
RUN apt install -y unzip wget

# Protoc binary
RUN wget -O protoc-3.18.1-linux-x86_64.zip https://github.com/protocolbuffers/protobuf/releases/download/v3.18.1/protoc-3.18.1-linux-x86_64.zip
RUN unzip protoc-3.18.1-linux-x86_64.zip -d ./protoc-bin

# Plugin: protoc-gen-grpc-web
RUN wget -O /usr/local/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.2.1/protoc-gen-grpc-web-1.2.1-linux-x86_64

# Plugin: protoc-gen-doc
RUN wget -O ./protoc-gen-doc-1.5.0.linux-amd64.go1.16.6.tar.gz https://github.com/pseudomuto/protoc-gen-doc/releases/download/v1.5.0/protoc-gen-doc-1.5.0.linux-amd64.go1.16.6.tar.gz
RUN tar xvf ./protoc-gen-doc-1.5.0.linux-amd64.go1.16.6.tar.gz
RUN mv ./protoc-gen-doc-1.5.0.linux-amd64.go1.16.6/protoc-gen-doc /usr/local/bin/protoc-gen-doc

RUN chmod +x /usr/local/bin/protoc-gen-grpc-web
RUN chmod +x /usr/local/bin/protoc-gen-doc

CMD ./protoc-bin/bin/protoc -I=$DIR $PROTO_FILE \
               --js_out=import_style=commonjs:$OUT_DIR \
               --doc_out=$DIR \
               --doc_opt=markdown,README.md \
               --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$OUT_DIR