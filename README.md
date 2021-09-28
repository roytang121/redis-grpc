# redis-web
```
redis gRPC proxy

USAGE:
    redis-web [OPTIONS]

FLAGS:
        --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -h, --host <HOST>    Target redis host to proxy from
    -p, --port <PORT>    Listen on port
```
### Example
```
redis-web -h redis://localhost:10400 -p 50051
```

## Commands
Supports `keys` `get` `set` `subscribe` `channel` and arbitrary command (response casted to string)


## Usage in Browser
### Installation
```
yarn install redis-grpc
```
> or npm install redis-grpc
### Basic Example
```typescript
/// JavaScript / TypeScript
import { RedisGrpcPromiseClient } from 'redis-web/gen-js/redis_grpc_grpc_web_pb';
import { KeysRequest, SetRequest } from 'redis-web/gen-js/redis_grpc_pb';

const perform_set = () => {
  const client = new RedisGrpcPromiseClient("http://localhost:50051");

  /// support TypeScript
  let req = new SetRequest();

  req.setKey("key")
  req.setValue("value");

  client.set(req)
    .then(resp => console.log(resp.getResult()))
    .catch(console.error)
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
})

/// Publish channel
const pub_request = new PublishRequest();
pub_request.setChannel("channel:1")
await client.publish(pub_request);
```

## License

This repository is licensed under the "MIT" license. See [LICENSE](LICENSE).