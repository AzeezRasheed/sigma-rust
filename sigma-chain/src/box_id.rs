use sigma_ser::serializer::SerializationError;
use sigma_ser::serializer::SigmaSerializable;
use sigma_ser::vlq_encode;
use std::io;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, collection::vec, prelude::*};

pub const BOX_ID_SIZE: usize = crate::constants::DIGEST32_SIZE;

#[derive(PartialEq, Debug)]
pub struct BoxId(pub [u8; BOX_ID_SIZE]);

impl SigmaSerializable for BoxId {
    fn sigma_serialize<W: vlq_encode::WriteSigmaVlqExt>(&self, mut w: W) -> Result<(), io::Error> {
        w.write_all(&self.0)?;
        Ok(())
    }
    fn sigma_parse<R: vlq_encode::ReadSigmaVlqExt>(mut r: R) -> Result<Self, SerializationError> {
        let mut bytes = [0; BOX_ID_SIZE];
        r.read_exact(&mut bytes)?;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
impl Arbitrary for BoxId {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (vec(any::<u8>(), BOX_ID_SIZE))
            .prop_map(|v| {
                let mut bytes = [0; BOX_ID_SIZE];
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
        fn box_id_roundtrip(box_id in any::<BoxId>()) {
            let mut data = Vec::new();
            box_id.sigma_serialize(&mut data)?;
            let box_id2 = BoxId::sigma_parse(&data[..])?;
            prop_assert_eq![box_id, box_id2];
        }
    }
}