pub trait ProtocolType: Sized {
    fn net_serialize(&self) -> Vec<u8>;
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()>;
}

pub trait ProtocolBuffer<T: ProtocolType>: Sized {
    fn net_deserialize(self) -> Result<T, ()>;
}

impl<T: ProtocolType> ProtocolBuffer<T> for &mut Vec<u8> {
    fn net_deserialize(self) -> Result<T, ()> {
        T::net_deserialize(self)
    }
}
