mod connected;
mod io;

pub use connected::Connected;

use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait Io: AsyncRead + AsyncWrite + Send + 'static {}
pub struct ClientIo(Pin<Box<dyn Io>>);
pub trait ConnectedIo: Io + Connected {}
pub struct ServerIo(Pin<Box<dyn ConnectedIo>>);

#[cfg(test)]
mod tests {
    use tokio::{
        io::{split, AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    };
    use tracing::info;

    use super::*;

    #[tokio::test]
    async fn test_client_and_server_io_should_work() {
        tracing_subscriber::fmt::init();

        let msg = b"Hello world\n";
        let mut buf = [0; 12];
        start_server("0.0.0.0:5000").await;
        start_client("127.0.0.1:5000", msg, &mut buf).await;

        assert_eq!(&buf, msg);
    }

    async fn start_server(addr: &str) {
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
    }

    async fn start_client(addr: &str, msg: &[u8], buf: &mut [u8]) {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        info!("client: conn established");

        stream.write_all(msg).await.unwrap();

        info!("client: send data");

        let (mut reader, _writer) = split(stream);

        reader.read_exact(buf).await.unwrap();

        info!("client: read echoed data");
    }
}
