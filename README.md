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

## Commands
Supports `get` `set` `keys` and arbitrary command (response casted to string)


## Usage in Browser
### Installation
`yarn install redis-grpc`
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

## License

This repository is licensed under the "MIT" license. See [LICENSE](LICENSE).