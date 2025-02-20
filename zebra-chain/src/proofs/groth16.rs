use std::{fmt, io};

use crate::serialization::{SerializationError, ZcashDeserialize, ZcashSerialize};

/// An encoding of a Groth16 proof, as used in Zcash.
pub struct Groth16Proof(pub [u8; 192]);

impl fmt::Debug for Groth16Proof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Groth16Proof")
            .field(&hex::encode(&self.0[..]))
            .finish()
    }
}

// These impls all only exist because of array length restrictions.

impl Copy for Groth16Proof {}

impl Clone for Groth16Proof {
    fn clone(&self) -> Self {
        let mut bytes = [0; 192];
        bytes[..].copy_from_slice(&self.0[..]);
        Self(bytes)
    }
}

impl PartialEq for Groth16Proof {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Eq for Groth16Proof {}

impl ZcashSerialize for Groth16Proof {
    fn zcash_serialize<W: io::Write>(&self, mut writer: W) -> Result<(), SerializationError> {
        writer.write_all(&self.0[..])?;
        Ok(())
    }
}

impl ZcashDeserialize for Groth16Proof {
    fn zcash_deserialize<R: io::Read>(mut reader: R) -> Result<Self, SerializationError> {
        let mut bytes = [0; 192];
        reader.read_exact(&mut bytes[..])?;
        Ok(Self(bytes))
    }
}
