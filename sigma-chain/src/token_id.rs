use core::fmt;
use sigma_ser::serializer::SerializationError;
use sigma_ser::serializer::SigmaSerializable;
use sigma_ser::vlq_encode;
use std::io;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, collection::vec, prelude::*};

pub const TOKEN_ID_SIZE: usize = crate::constants::DIGEST32_SIZE;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct TokenId(pub [u8; TOKEN_ID_SIZE]);

impl fmt::Display for TokenId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = io::Cursor::new(Vec::new());
        let _ = self.sigma_serialize(&mut bytes);

        f.debug_tuple("TokenId")
            .field(&base16::encode_lower(bytes.get_ref()))
            .finish()
    }
}

impl SigmaSerializable for TokenId {
    fn sigma_serialize<W: vlq_encode::WriteSigmaVlqExt>(&self, mut w: W) -> Result<(), io::Error> {
        w.write_all(&self.0)?;
        Ok(())
    }
    fn sigma_parse<R: vlq_encode::ReadSigmaVlqExt>(mut r: R) -> Result<Self, SerializationError> {
        let mut bytes = [0; TOKEN_ID_SIZE];
        r.read_exact(&mut bytes)?;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
impl Arbitrary for TokenId {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (vec(any::<u8>(), TOKEN_ID_SIZE))
            .prop_map(|v| {
                let mut bytes = [0; TOKEN_ID_SIZE];
                bytes.copy_from_slice(v.as_slice());
                Self(bytes)
            })
            .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {

        #[test]
        fn token_id_roundtrip(id in any::<TokenId>()) {
            let mut data = Vec::new();
            id.sigma_serialize(&mut data)?;
            let id2 = TokenId::sigma_parse(&data[..])?;
            prop_assert_eq![id, id2];
        }
    }
}