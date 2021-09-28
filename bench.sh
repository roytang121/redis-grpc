ghz --insecure --proto ./proto/redis_grpc.proto -c 50 --connections=12 -z 5s --call redis_grpc.RedisGrpc.Get -d '{"key": "Hello"}'  localhost:50051
