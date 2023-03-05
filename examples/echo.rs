use multicaster::Multicaster;

#[tokio::main]
async fn main() {
    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, false).await;
    println!("Echoing multicast UDP packets destined for 239.0.0.20:5007");
    loop {
        let mut buf: [u8; 256] = [0x00; 256];
        let (len, src) = multi.rec(&mut buf).await;
        multi.send(&mut buf).await;

        println!("Echoed {} bytes from {}: {}", len, src, String::from_utf8_lossy(&buf));
    }
}