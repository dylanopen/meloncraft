use crate::ProtocolType;

pub struct VarInt(pub i32);

const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

impl ProtocolType for VarInt {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        let mut uvalue = self.0 as u32;
        loop {
            let mut byte = (uvalue & SEGMENT_BITS) as u8;
            uvalue >>= 7;

            if uvalue != 0 {
                byte |= CONTINUE_BIT;
            }

            output.push(byte);

            if uvalue == 0 {
                break;
            }
        }

        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.is_empty() {
            return Err(());
        }

        let mut value: i32 = 0;
        let mut position = 0;
        let mut current_byte: u8;

        loop {
            current_byte = data.remove(0);
            value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(());
            }
        }
        Ok(Self(value))
    }
}
