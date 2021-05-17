use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicU32, Ordering};





#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    //let mut packet_cnt = AtomicU32::new(0);
    let mut local_time = tokio::time::Instant::now();
    let mut signal = false;
    //let (stop_signal_sender, stop_signal_receiver) = tokio::sync::oneshot::channel::<()>();
    if let Ok((mut stream, _)) = listener.accept().await {
        let mut buf = vec![0u8; 10];
        for _ in 0..10000 {
            if let Ok(nbytes) = stream.read(&mut buf[..]).await {
                //packet_cnt.fetch_add(1, Ordering::SeqCst);
                if !signal {
                    local_time = tokio::time::Instant::now();
                    signal = true;
                }
                //assert_eq!(nbytes, 8);
            }
        }
    }
    let duration = tokio::time::Instant::now().duration_since(local_time).as_millis() as f64;
    println!("pps: {}", (10000 as f64)/duration * 1000 as f64);
    println!("Duration: {} ms", duration);
    Ok(())
}