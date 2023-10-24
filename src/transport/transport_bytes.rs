// src/transport/transport_bytes.rs

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn send_data(addr: &str, data: Vec<u8>) -> io::Result<TcpStream> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(&data).await?;
    Ok(stream)
}

pub async fn receive_data(addr: &str, expected_length: usize) -> io::Result<Vec<u8>> {
    let listener = TcpListener::bind(addr).await?;
    let (mut socket, _) = listener.accept().await?;

    let mut data = Vec::with_capacity(expected_length);
    socket.read_to_end(&mut data).await?;

    Ok(data)
}
