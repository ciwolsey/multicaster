use std::net::SocketAddr;

use tokio::net::UdpSocket;

pub struct Multicaster {
    pub socket: UdpSocket,
    pub port: u16,
    pub multi_ip: String,
}

impl Multicaster {
    pub async fn new(local_ip: &str, multi_ip: &str, port: u16) -> Multicaster {
        let socket = UdpSocket::bind((local_ip, port)).await.unwrap();

        socket
            .join_multicast_v4(multi_ip.parse().unwrap(), local_ip.parse().unwrap())
            .unwrap();

        socket.set_multicast_loop_v4(false).unwrap();

        Multicaster {
            multi_ip: multi_ip.to_string(),
            port,
            socket,
        }
    }

    pub async fn send(&self, buf: &[u8]) {
        self.socket
            .send_to(buf, (self.multi_ip.as_str(), self.port))
            .await
            .unwrap();
    }

    pub async fn rec(&self, buf: &mut [u8]) -> (usize, SocketAddr) {
        self.socket.recv_from(buf).await.unwrap()
    }
}