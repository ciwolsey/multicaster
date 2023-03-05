use multicaster::Multicaster;

#[tokio::main]
async fn main() {
    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007).await;
    multi.send(&[0x41]).await;
    let mut buf: [u8; 10] = [0x00; 10];
    
    let (len, src) = multi.rec(&mut buf).await;
    println!("Received {} bytes from {}: {}", len, src, String::from_utf8_lossy(&buf));
}
