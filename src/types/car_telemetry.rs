extern crate bytebuffer;

use bytebuffer::ByteBuffer;

pub struct CarTelemetryPacket {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: u8,
    pub rev_lights_percent: u8,
    pub rev_lights_bit_value: u16,
    pub brakes_temperature: [u16; 4],
    pub tyres_surface_temperature: [u8; 4],
    pub tyres_inner_temperature: [u8; 4],
    pub engine_temperature: u16,
    pub tyres_pressure: [f32; 4],
    pub surface_type: [u8; 4],
}

impl CarTelemetryPacket {
    pub fn new(reader: &mut ByteBuffer) -> CarTelemetryPacket {
        return CarTelemetryPacket {
            speed: reader.read_u16().unwrap(),
            throttle: reader.read_f32().unwrap(),
            steer: reader.read_f32().unwrap(),
            brake: reader.read_f32().unwrap(),
            clutch: reader.read_u8().unwrap(),
            gear: reader.read_i8().unwrap(),
            engine_rpm: reader.read_u16().unwrap(),
            drs: reader.read_u8().unwrap(),
            rev_lights_percent: reader.read_u8().unwrap(),
            rev_lights_bit_value: reader.read_u16().unwrap(),
            brakes_temperature: [
                reader.read_u16().unwrap(),
                reader.read_u16().unwrap(),
                reader.read_u16().unwrap(),
                reader.read_u16().unwrap(),
            ],
            tyres_surface_temperature: [
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
            ],
            tyres_inner_temperature: [
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
            ],
            engine_temperature: reader.read_u16().unwrap(),
            tyres_pressure: [
                reader.read_f32().unwrap(),
                reader.read_f32().unwrap(),
                reader.read_f32().unwrap(),
                reader.read_f32().unwrap(),
            ],
            surface_type: [
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
                reader.read_u8().unwrap(),
            ],
        };
    }
}
