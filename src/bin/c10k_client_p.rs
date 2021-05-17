use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use rand::Rng;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let num1 = (rng.gen::<u32>()/2 + 1)/2;
        let num2 = (rng.gen::<u32>()/2 + 1)/2;
        let mut packet = num1.to_le_bytes().to_vec();
        packet.extend_from_slice(&num2.to_le_bytes());
        stream.write(&packet[..8]).await;
    }

    Ok(())
}