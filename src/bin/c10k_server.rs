use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicU32, Ordering};



/// 8 bytes [u32][u32]
async fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0; 10];
    if let Ok(nbytes) = stream.read(&mut buf).await {
        let num1 = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let num2 = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        stream.write(&(num1 + num2).to_le_bytes()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    //let mut packet_cnt = AtomicU32::new(0);
    let mut local_time = tokio::time::Instant::now();
    let mut signal = false;
    //let (stop_signal_sender, stop_signal_receiver) = tokio::sync::oneshot::channel::<()>();
    for _ in 0..10000 {
        if let Ok((stream, _)) = listener.accept().await {
            //packet_cnt.fetch_add(1, Ordering::SeqCst);
            if !signal {
                local_time = tokio::time::Instant::now();
                signal = true;
            }
            tokio::spawn(async move {
                handle_stream(stream).await;
            });
        }
    }
    let duration = tokio::time::Instant::now().duration_since(local_time).as_millis() as f64;
    println!("connections per millisecond: {}", (10000 as f64)/duration * 1000 as f64);
    println!("Duration: {} ms", duration);
    Ok(())
}