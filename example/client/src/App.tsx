import React, { useEffect } from "react";
import logo from "./logo.svg";
import "./App.css";
import { RedisGrpcPromiseClient } from "redis-grpc/gen-js/redis_grpc_grpc_web_pb";
import {
  KeysRequest,
  SetRequest,
  SubscribeRequest,
  LPushRequest,
} from "redis-grpc/gen-js/redis_grpc_pb";

function App() {
  useEffect(() => {
    const client = new RedisGrpcPromiseClient("http://localhost:50051");
    let request = new KeysRequest();
    request.setPattern("*");
    client.keys(request).then((response) => {
      console.log(response.getResultList());
    });

    const perform_set = () => {
      let set_cmd = new SetRequest();
      set_cmd.setKey(Date.now().toString());
      set_cmd.setValue(123 as any);
      client
        .set(set_cmd)
        .then((resp) => console.log(resp.getResult()))
        .catch(console.error);
    };

    const perform_subscribe = () => {
      const sub_request = new SubscribeRequest();
      sub_request.setChannelsList(["LambdaParams:test", "LambdaParams:test2"]);
      const stream = client.subscribe(sub_request);
      stream.on("data", (resp) => {
        console.log(resp.toObject());
      });
    };

    const perform_lpush = () => {
      const lpush = new LPushRequest();
      lpush.setKey("Hello");
      lpush.setElement("diudiu");
      client
        .lpush(lpush)
        .then((resp) => console.log(resp.toObject()))
        .catch(console.error);
    };

    perform_set();
    perform_subscribe();
    perform_lpush();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
