use parity_scale_codec::{
    Encode,
    EncodeAsRef,
    HasCompact,
};

#[derive(Clone)]
pub struct Encoded(pub Vec<u8>);

impl Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

pub fn compact<T: HasCompact>(t: T) -> Encoded {
    let encodable: <<T as HasCompact>::Type as EncodeAsRef<'_, T>>::RefType =
        From::from(&t);
    Encoded(encodable.encode())
}
