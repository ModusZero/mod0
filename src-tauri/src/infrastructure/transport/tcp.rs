use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

pub struct TcpExternal {
    addr: String,
}

impl TcpExternal {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub async fn send_and_receive(&self, payload: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut stream = TcpStream::connect(&self.addr).await?;
        
        // Enviar
        stream.write_all(&payload).await?;

        // Leer respuesta (buffer simple, idealmente usar Frames)
        let mut buffer = vec![0; 4096];
        let n = stream.read(&mut buffer).await?;
        buffer.truncate(n);
        
        Ok(buffer)
    }
}