# redis-grpc
```
redis gRPC proxy

USAGE:
    redis-grpc [OPTIONS]

FLAGS:
        --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -h, --host <HOST>    Target redis host to proxy from
    -p, --port <PORT>    Listen on port
```

## Supported Redis Commands
```
Supports `keys` `get` `set` `subscribe` `publish` 
# and arbitrary command (response casted to string)
```

## Server Side Usage
### Usage with Binary
```shell
redis-grpc -h redis://localhost:6379 -p 50051
```
### Usage with Docker CLI
```shell
docker run --rm \
  --network=host \
  -e REDIS_GRPC_HOST=redis://localhost:6379 \
  -e REDIS_GRPC_PORT=50051 \
  -e RUST_LOG=INFO \
  roytang121/redis-grpc:latest
```
> more commonly, used with docker-compose
### Usage with Docker Compose
```docker
version: '3.1'

services:
  redis:
    image: redis:latest
    ports:
      - 6379:6379

  redis-grpc:
    image: roytang121/redis-grpc:latest
    depends_on:
      - redis
    environment:
      - RUST_LOG=INFO
      - REDIS_GRPC_PORT=50051
      - REDIS_GRPC_HOST=redis://redis:6379
    ports:
      - '50051:50051'
```

## Client Side Usage - Browser
Calling redis commands in browser with JavaScript / TypeScript.
### Installation
```
npm install redis-grpc
```
### Basic Example
```typescript
/// JavaScript / TypeScript
import { RedisGrpcPromiseClient } from 'redis-grpc/gen-js/redis_grpc_grpc_web_pb';
import { KeysRequest, SetRequest } from 'redis-grpc/gen-js/redis_grpc_pb';

const perform_set = () => {
  const client = new RedisGrpcPromiseClient("http://localhost:50051");

  /// support TypeScript
  let req = new SetRequest();

  req.setKey("key")
  req.setValue("value");

  client.set(req)
    .then(resp => console.log(resp.getResult()))
    .catch(console.error);
}
```

### PubSub
```typescript
/// Subscribe channels
const sub_request = new SubscribeRequest();
sub_request.setChannelsList(["channel:1", "channel:2"]);
const stream = client.subscribe(sub_request);
stream.on('data', (data: SubscribeResponse) => {
  console.log({
    message: data.getMessage(),
      channel: data.getChannel(),
  })
});

/// Publish channel
const pub_request = new PublishRequest();
pub_request.setChannel("channel:1")
await client.publish(pub_request);
```

## License
This repository is licensed under the "MIT" license. See [LICENSE](LICENSE).