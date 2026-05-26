use crate::{PrefixedArray, ProtocolType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitSet {
    pub bits: Vec<i64>,
}

impl BitSet {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        return self.bits.is_empty();
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        return BitSet {
            bits: vec![0; capacity.div_ceil(64)],
        };
    }

    #[must_use]
    pub const fn capacity(&self) -> usize {
        return self.bits.len() * 64;
    }

    #[must_use]
    pub fn get(&mut self, pos: usize) -> bool {
        let (index, bit_pos) = self.get_location(pos);
        return (self.bits.get(index).unwrap() & (1 << bit_pos)) != 0;
    }

    #[must_use]
    pub fn get_location(&mut self, pos: usize) -> (usize, usize) {
        let index = pos / 64;
        let bit_pos = pos % 64;
        if index >= self.bits.len() {
            self.bits.resize(index + 1, 0);
        }
        return (index, bit_pos);
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "Much simpler to mutate value than .get_mut(index).unwrap()"
    )]
    pub fn set(&mut self, pos: usize) {
        let (index, bit_pos) = self.get_location(pos);
        self.bits[index] |= 1 << bit_pos;
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "Much simpler to mutate value than .get_mut(index).unwrap()"
    )]
    pub fn unset(&mut self, pos: usize) {
        let (index, bit_pos) = self.get_location(pos);
        self.bits[index] &= !(1 << bit_pos);
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "Much simpler to mutate value than .get_mut(index).unwrap()"
    )]
    pub fn toggle(&mut self, pos: usize) {
        let (index, bit_pos) = self.get_location(pos);
        self.bits[index] ^= 1 << bit_pos;
    }

    pub fn clear(&mut self) {
        self.bits.clear();
    }
}

impl ProtocolType for BitSet {
    fn net_serialize(&self) -> Vec<u8> {
        let mut serial = Vec::new();
        for long_bits in &self.bits {
            serial.push(*long_bits);
        }
        return PrefixedArray(serial).net_serialize();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let mut bits = Vec::new();
        let longs: Vec<i64> = PrefixedArray::net_deserialize(data)?.0;
        for long in &longs {
            bits.push(*long);
        }
        return Ok(BitSet { bits });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolBuffer;

    #[test]
    fn bitset_create() {
        let mut bitset = BitSet::default();
        assert!(bitset.is_empty());
        assert_eq!(bitset.capacity(), 0);
        assert!(!bitset.get(0));
    }

    #[test]
    fn bitset_set() {
        let mut bitset = BitSet::default();
        bitset.set(3);
        assert_eq!(bitset.capacity(), 64);
        assert_eq!(bitset.bits[0], 8);
        assert_eq!(bitset.bits.get(1), None);
    }

    #[test]
    fn bitset_unset() {
        let mut bitset = BitSet::default();
        bitset.set(2);
        assert_eq!(bitset.bits[0], 4);
        bitset.unset(2);
        assert_eq!(bitset.bits[0], 0);
    }

    #[test]
    fn bitset_get() {
        let mut bitset = BitSet::default();
        bitset.set(1);
        assert!(bitset.get(1));
        bitset.set(2);
        assert!(!bitset.get(0));
    }

    #[test]
    fn bitset_toggle() {
        let mut bitset = BitSet::default();
        bitset.toggle(2);
        assert!(bitset.get(2));
        bitset.toggle(2);
        assert!(!bitset.get(2));
    }

    #[test]
    fn bitset_clear() {
        let mut bitset = BitSet::default();
        bitset.set(1);
        bitset.set(68);
        assert!(bitset.get(1));
        assert!(bitset.get(68));
        assert_eq!(bitset.capacity(), 128);
        bitset.clear();
        assert_eq!(bitset.capacity(), 0);
        assert_eq!(bitset.get(1), false);
        assert_eq!(bitset.get(68), false);
    }

    #[test]
    fn bitset_serialize() {
        let mut bitset = BitSet::default();
        bitset.set(1);
        bitset.set(64);
        let mut serialized = bitset.net_serialize();
        let serialized_longs: PrefixedArray<u64> = serialized.net_deserialize().unwrap();
        assert_eq!(serialized_longs.0.len(), 2);
        assert_eq!(serialized_longs.0[0], 2);
        assert_eq!(serialized_longs.0[1], 1);
    }

    #[test]
    fn bitset_serde() {
        let mut bitset = BitSet::default();
        bitset.set(1);
        bitset.set(65);
        let serialized = bitset.net_serialize();
        let deserialized = BitSet::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(bitset.bits, deserialized.bits);
    }
}
