pub trait ProtocolType: Sized {
    fn net_serialize(&self) -> Vec<u8>;
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()>;
}
