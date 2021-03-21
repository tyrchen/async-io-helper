# Async IO Helper

Boxed async IO for tokio client/server.

Usage:

Server:

```rust
let listener = TcpListener::bind(addr).await.unwrap();
tokio::spawn(async move {
    let (stream, _) = listener.accept().await.unwrap();
    let io = ServerIo::new(stream);
    info!("server: accepted: {:?}", io.remote_addr());
    let (mut reader, mut writer) = split(io);
    let mut buf = [0; 12];
    reader.read_exact(&mut buf).await.unwrap();
    info!("server: got data: {:?}", buf);
    writer.write_all(&buf).await.unwrap();
    info!("server: flush the data out");
});
```

Client:

```rust
let mut stream = TcpStream::connect(addr).await.unwrap();
info!("client: conn established");

stream.write_all(msg).await.unwrap();

info!("client: send data");

let (mut reader, _writer) = split(stream);

reader.read_exact(buf).await.unwrap();

info!("client: read echoed data");
```

## License

`async-io-helper` is distributed under the terms of MIT.

See [LICENSE](LICENSE.md) for details.

Copyright 2021 Tyr Chen
