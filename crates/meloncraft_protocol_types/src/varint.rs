use crate::ProtocolType;

pub struct VarInt(i32);

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

impl ProtocolType for VarInt {
    fn net_deserialize(bytes: &mut Vec<u8>) -> Result<Self, ()> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut current_byte: u8;
        bytes.reverse();

        loop {
            current_byte = bytes.pop().unwrap_or(0);
            value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(());
            }
        }

        bytes.reverse();

        Ok(VarInt(value))
    }

    fn net_serialize(&self) -> Vec<u8> {
        let mut value = self.0;
        let mut output: Vec<u8> = Vec::new();

        loop {
            if value as u8 & !SEGMENT_BITS == 0 {
                output.push(value as u8);
                break;
            }

            output.push((value as u8 & SEGMENT_BITS) | CONTINUE_BIT);

            value >>= 7;

            if output.len() >= 5 {
                break;
            }
        }

        output
    }
}
