use multicaster::Multicaster;

#[tokio::main]
async fn main() {
    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, true).await;
    multi.send(String::from("Loopback").as_bytes()).await;

    let mut buf: [u8; 256] = [0x00; 256];
    
    let (len, src) = multi.rec(&mut buf).await;
    println!("Received {} bytes from {}: {}", len, src, String::from_utf8_lossy(&buf));
}