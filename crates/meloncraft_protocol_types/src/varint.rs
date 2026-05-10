use crate::ProtocolType;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolBuffer;

    // Example cases taken from https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong
    #[test]
    fn test_serialize_varint_0() {
        assert_eq!(VarInt(0).net_serialize(), vec![0x00]);
    }
    #[test]
    fn test_serialize_varint_1() {
        assert_eq!(VarInt(1).net_serialize(), vec![0x01]);
    }
    #[test]
    fn test_serialize_varint_127() {
        assert_eq!(VarInt(127).net_serialize(), vec![0x7f]);
    }
    #[test]
    fn test_serialize_varint_128() {
        assert_eq!(VarInt(128).net_serialize(), vec![0x80, 0x01]);
    }
    #[test]
    fn test_serialize_varint_255() {
        assert_eq!(VarInt(255).net_serialize(), vec![0xff, 0x01]);
    }
    #[test]
    fn test_serialize_varint_25565() {
        assert_eq!(VarInt(25565).net_serialize(), vec![0xdd, 0xc7, 0x01]);
    }
    #[test]
    fn test_serialize_varint_2097151() {
        assert_eq!(VarInt(2097151).net_serialize(), vec![0xff, 0xff, 0x7f]);
    }
    #[test]
    fn test_serialize_varint_2147483647() {
        assert_eq!(VarInt(2147483647).net_serialize(), vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    }
    #[test]
    fn test_serialize_varint_neg1() {
        assert_eq!(VarInt(-1).net_serialize(), vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
    }
    #[test]
    fn test_serialize_varint_neg2147483648() {
        assert_eq!(VarInt(-2147483648).net_serialize(), vec![0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn test_deserialize_varint_0() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0x00]).unwrap().0, 0);
    }
    #[test]
    fn test_deserialize_varint_1() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0x01]).unwrap().0, 1);
    }
    #[test]
    fn test_deserialize_varint_127() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0x7f]).unwrap().0, 127);
    }
    #[test]
    fn test_deserialize_varint_128() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0x80, 0x01]).unwrap().0, 128);
    }
    #[test]
    fn test_deserialize_varint_255() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0xff, 0x01]).unwrap().0, 255);
    }
    #[test]
    fn test_deserialize_varint_25565() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0xdd, 0xc7, 0x01]).unwrap().0, 25565);
    }
    #[test]
    fn test_deserialize_varint_2097151() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0xff, 0xff, 0x7f]).unwrap().0, 2097151);
    }
    #[test]
    fn test_deserialize_varint_2147483647() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0xff, 0xff, 0xff, 0xff, 0x07]).unwrap().0, 2147483647);
    }
    #[test]
    fn test_deserialize_varint_neg1() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0xff, 0xff, 0xff, 0xff, 0x0f]).unwrap().0, -1);
    }
    #[test]
    fn test_deserialize_varint_neg2147483648() {
        assert_eq!(VarInt::net_deserialize(&mut vec![0x80, 0x80, 0x80, 0x80, 0x08]).unwrap().0, -2147483648);
    }
}
