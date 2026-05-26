pub trait ProtocolType: Sized {
    fn net_serialize(&self) -> Vec<u8>;
    #[expect(
        clippy::result_unit_err,
        reason = "More important things to do, currently, than nice deserialization error handling"
    )]
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()>;
}

pub trait ProtocolBuffer<T: ProtocolType>: Sized {
    #[expect(
        clippy::result_unit_err,
        reason = "More important things to do, currently, than nice deserialization error handling"
    )]
    fn net_deserialize(self) -> Result<T, ()>;
}

impl<T: ProtocolType> ProtocolBuffer<T> for &mut Vec<u8> {
    fn net_deserialize(self) -> Result<T, ()> {
        return T::net_deserialize(self);
    }
}
