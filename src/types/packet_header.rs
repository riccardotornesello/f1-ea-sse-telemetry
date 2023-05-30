extern crate bytebuffer;

use bytebuffer::ByteBuffer;

pub struct PacketHeader {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

impl PacketHeader {
    pub fn new(reader: &mut ByteBuffer) -> PacketHeader {
        return PacketHeader {
            packet_format: reader.read_u16().unwrap(),
            game_major_version: reader.read_u8().unwrap(),
            game_minor_version: reader.read_u8().unwrap(),
            packet_version: reader.read_u8().unwrap(),
            packet_id: reader.read_u8().unwrap(),
            session_uid: reader.read_u64().unwrap(),
            session_time: reader.read_f32().unwrap(),
            frame_identifier: reader.read_u32().unwrap(),
            player_car_index: reader.read_u8().unwrap(),
            secondary_player_car_index: reader.read_u8().unwrap(),
        };
    }
}
