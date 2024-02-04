use casper_types::{CLType, CLTyped, NamedArg};

use crate::{prelude::*, FromNamedArg, ToStrKey};

/// A token as identified by its NFT package and index
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TokenIdentifier {
    pub index: TokenIndex,
    pub package: ContractPackageHash 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum TokenIndex {
    Ordinal(u64),
    Hash(String)
}

impl TokenIdentifier {
    pub fn try_load_from_runtime_args() -> Option<Self> {
        try_get_named_arg::<TokenIdentifier>("token_id").ok()
    }

    pub fn to_named_arg(&self) -> NamedArg {
        let self_bytes = self
            .to_bytes()
            .unwrap();

        let serialized_self = CLValue::from_t(self_bytes).unwrap();

        NamedArg::new("token_id".into(), serialized_self)
    }
}

impl ToStrKey for TokenIdentifier {
    fn to_key(&self) -> String {
        self.to_bytes().unwrap().to_key()
    }
}

impl ToBytes for TokenIdentifier {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = bytesrepr::allocate_buffer(self)?;

        result.append(&mut self.index.to_bytes()?);
        result.append(&mut self.package.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.index.serialized_length() + self.package.serialized_length()
    }
}

impl FromBytes for TokenIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (index, bytes) = TokenIndex::from_bytes(bytes)?;
        let (package, bytes) = ContractPackageHash::from_bytes(bytes)?;
        let parsed = TokenIdentifier { index, package };
        Ok((parsed, bytes))
    }
}

impl ToBytes for TokenIndex {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        match self {
            TokenIndex::Ordinal(value) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(0);
                result.append(&mut value.to_bytes()?);
                Ok(result)
            }
            TokenIndex::Hash(value) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(1);
                result.append(&mut value.to_bytes()?);
                Ok(result)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        match self {
            TokenIndex::Ordinal(value) => 1 + value.serialized_length(),
            TokenIndex::Hash(value) => 1 + value.serialized_length(),
        }
    }
}

impl FromBytes for TokenIndex {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, rem) = u8::from_bytes(bytes)?;
        match tag {
            0 => {
                let (index, rem) = u64::from_bytes(rem)?;
                Ok((TokenIndex::Ordinal(index), rem))
            }
            1 => {
                let (hash, rem) = String::from_bytes(rem)?;
                Ok((TokenIndex::Hash(hash), rem))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

impl CLTyped for TokenIdentifier {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl FromNamedArg for TokenIdentifier {
    fn try_get(_: &str) -> Option<Self> {
        TokenIdentifier::try_load_from_runtime_args()
    }
}
