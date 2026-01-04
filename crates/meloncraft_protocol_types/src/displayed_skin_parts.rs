use crate::ProtocolType;
use meloncraft_player::DisplayedSkinParts;

// This may be wrong. The endian-ness matters here, and may be incorrect.
impl ProtocolType for DisplayedSkinParts {
    fn net_serialize(&self) -> Vec<u8> {
        vec![
            self.cape as u8
                + self.jacket as u8 * 0x02
                + self.left_sleeve as u8 * 0x04
                + self.right_sleeve as u8 * 0x08
                + self.left_pants_leg as u8 * 0x10
                + self.right_pants_leg as u8 * 0x20
                + self.hat as u8 * 0x40,
        ]
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.is_empty() {
            return Err(());
        }
        let arg_data = data.drain(0..1);
        let byte = u8::from_be_bytes(arg_data.as_slice().try_into().map_err(|_| ())?);

        Ok(DisplayedSkinParts {
            cape: (byte & 0x01) != 0,
            jacket: (byte & 0x02) != 0,
            left_sleeve: (byte & 0x04) != 0,
            right_sleeve: (byte & 0x08) != 0,
            left_pants_leg: (byte & 0x10) != 0,
            right_pants_leg: (byte & 0x20) != 0,
            hat: (byte & 0x40) != 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ProtocolType;
    use meloncraft_player::DisplayedSkinParts;

    #[test]
    fn test_displayed_skin_parts_serialization() {
        let parts = DisplayedSkinParts {
            cape: true,
            jacket: false,
            left_sleeve: true,
            right_sleeve: false,
            left_pants_leg: true,
            right_pants_leg: false,
            hat: true,
        };

        let serialized = parts.net_serialize();
        assert_eq!(serialized, vec![0b01010101]);

        let mut data = serialized.clone();
        let deserialized = DisplayedSkinParts::net_deserialize(&mut data).unwrap();
        assert_eq!(deserialized, parts);
    }
}
