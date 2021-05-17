

use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::Rng;



/// 8 bytes [u32][u32]
async fn handle_stream(mut stream: TcpStream, num1: u32, num2: u32) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0u8; 10];
    let mut packet = num1.to_le_bytes().to_vec();
    packet.extend_from_slice(&num2.to_le_bytes());
    assert_eq!(packet.len(), 8);
    stream.write(&packet).await;
    if let Ok(nbytes) = stream.read(&mut buf).await {
        assert_eq!(nbytes, 4);
        let sum = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        assert_eq!(num2 + num1, sum);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let stream = TcpStream::connect("127.0.0.1:8080").await?;
        let num1 = (rng.gen::<u32>()/2 + 1)/2;
        let num2 = (rng.gen::<u32>()/2 + 1)/2;
        tokio::spawn(async move {
           handle_stream(stream, num1, num2).await;
        });
    }
    loop {}
    Ok(())
}