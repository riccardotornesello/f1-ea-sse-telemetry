mod types;

use crate::types::{CarTelemetryPacket, PacketHeader};
use bytebuffer::{ByteBuffer, Endian};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        // let connection = sqlite::open(":memory:").unwrap();

        let socket = UdpSocket::bind("0.0.0.0:34254")?;
        let mut buf = [0; 4096];

        loop {
            let (amt, _src) = socket.recv_from(&mut buf)?;
            let mut buffer = ByteBuffer::from_bytes(&mut buf[..amt]);
            buffer.set_endian(Endian::LittleEndian);

            let header = PacketHeader::new(&mut buffer);

            match header.packet_id {
                6 => {
                    let packet = CarTelemetryPacket::new(&mut buffer);
                    println!("{:?}", packet.tyres_surface_temperature);
                }
                _ => {}
            }
        }
    }
}
