use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    // echo server. Waits for a client to connect then sends a message back to the client.
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
