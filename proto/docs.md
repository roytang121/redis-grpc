# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [redis_grpc.proto](#redis_grpc.proto)
    - [CommandRequest](#redis_grpc.CommandRequest)
    - [CommandResponse](#redis_grpc.CommandResponse)
    - [DelRequest](#redis_grpc.DelRequest)
    - [GetRequest](#redis_grpc.GetRequest)
    - [IntResponse](#redis_grpc.IntResponse)
    - [KeysRequest](#redis_grpc.KeysRequest)
    - [LPushRequest](#redis_grpc.LPushRequest)
    - [PublishRequest](#redis_grpc.PublishRequest)
    - [RPushRequest](#redis_grpc.RPushRequest)
    - [SetRequest](#redis_grpc.SetRequest)
    - [StringListResponse](#redis_grpc.StringListResponse)
    - [StringResponse](#redis_grpc.StringResponse)
    - [SubscribeRequest](#redis_grpc.SubscribeRequest)
    - [SubscribeResponse](#redis_grpc.SubscribeResponse)
  
    - [RedisGrpc](#redis_grpc.RedisGrpc)
  
- [Scalar Value Types](#scalar-value-types)



<a name="redis_grpc.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## redis_grpc.proto



<a name="redis_grpc.CommandRequest"></a>

### CommandRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| command | [string](#string) |  |  |






<a name="redis_grpc.CommandResponse"></a>

### CommandResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) | optional |  |






<a name="redis_grpc.DelRequest"></a>

### DelRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="redis_grpc.GetRequest"></a>

### GetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="redis_grpc.IntResponse"></a>

### IntResponse
Generic Integer Type Response


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [int64](#int64) |  | Integer result |






<a name="redis_grpc.KeysRequest"></a>

### KeysRequest
https://redis.io/commands/keys


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pattern | [string](#string) |  |  |






<a name="redis_grpc.LPushRequest"></a>

### LPushRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| element | [string](#string) |  |  |






<a name="redis_grpc.PublishRequest"></a>

### PublishRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| channel | [string](#string) |  |  |
| message | [string](#string) |  |  |






<a name="redis_grpc.RPushRequest"></a>

### RPushRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| element | [string](#string) |  |  |






<a name="redis_grpc.SetRequest"></a>

### SetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="redis_grpc.StringListResponse"></a>

### StringListResponse
Generic List&lt;String&gt; Type Response


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [string](#string) | repeated | List&lt;String&gt; result |






<a name="redis_grpc.StringResponse"></a>

### StringResponse
Generic String Type Response


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [string](#string) | optional | String result |






<a name="redis_grpc.SubscribeRequest"></a>

### SubscribeRequest
https://redis.io/commands/subscribe


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| channels | [string](#string) | repeated |  |






<a name="redis_grpc.SubscribeResponse"></a>

### SubscribeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| channel | [string](#string) |  |  |
| message | [string](#string) |  |  |





 

 

 


<a name="redis_grpc.RedisGrpc"></a>

### RedisGrpc
RedisGrpc Service
author: Roy.Tang &lt;me@roytang.me&gt;

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| command | [CommandRequest](#redis_grpc.CommandRequest) | [CommandResponse](#redis_grpc.CommandResponse) |  |
| subscribe | [SubscribeRequest](#redis_grpc.SubscribeRequest) | [SubscribeResponse](#redis_grpc.SubscribeResponse) stream |  |
| publish | [PublishRequest](#redis_grpc.PublishRequest) | [IntResponse](#redis_grpc.IntResponse) |  |
| keys | [KeysRequest](#redis_grpc.KeysRequest) | [StringListResponse](#redis_grpc.StringListResponse) |  |
| get | [GetRequest](#redis_grpc.GetRequest) | [StringResponse](#redis_grpc.StringResponse) |  |
| set | [SetRequest](#redis_grpc.SetRequest) | [StringResponse](#redis_grpc.StringResponse) |  |
| del | [DelRequest](#redis_grpc.DelRequest) | [IntResponse](#redis_grpc.IntResponse) |  |
| lpush | [LPushRequest](#redis_grpc.LPushRequest) | [IntResponse](#redis_grpc.IntResponse) |  |
| rpush | [RPushRequest](#redis_grpc.RPushRequest) | [IntResponse](#redis_grpc.IntResponse) |  |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

