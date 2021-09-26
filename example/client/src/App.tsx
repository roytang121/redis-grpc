import React, { useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import { RedisGrpcPromiseClient } from 'redis-web/gen-js/redis_grpc_grpc_web_pb';
import { KeysRequest, SetRequest } from 'redis-web/gen-js/redis_grpc_pb';

function App() {

  useEffect(() => {
    const client = new RedisGrpcPromiseClient("http://localhost:50051");
    let request = new KeysRequest();
    request.setPattern("*");
    client.keys(request).then(response => {
      console.log(response.getResultList())
    });

    const perform_set = () => {
      let set_cmd = new SetRequest();
      set_cmd.setKey(Date.now().toString())
      set_cmd.setValue(123 as any);
      client.set(set_cmd).then(resp => console.log(resp.getResult())).catch(console.error)
    }

    perform_set()
  }, [])

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
