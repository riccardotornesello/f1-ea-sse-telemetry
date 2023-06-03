use crate::broadcaster::Broadcaster;
use crate::types::{CarTelemetryPacket, PacketHeader};
use bytebuffer::{ByteBuffer, Endian};
use std::net::UdpSocket;
use std::sync::Arc;

pub async fn listener_handler(listener_address: &str, broadcaster: Arc<Broadcaster>) {
    let socket = UdpSocket::bind(listener_address).expect("Couldn't bind to address");
    let mut buf = [0; 4096];

    println!("Listening telemetry on {}", socket.local_addr().unwrap());

    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let mut buffer = ByteBuffer::from_bytes(&mut buf[..amt]);
        buffer.set_endian(Endian::LittleEndian);

        let header = PacketHeader::new(&mut buffer);

        match header.packet_id {
            6 => {
                let packet: CarTelemetryPacket = CarTelemetryPacket::new(&mut buffer);
                broadcaster.broadcast("car", packet).await;
            }
            _ => {}
        }
    }
}
