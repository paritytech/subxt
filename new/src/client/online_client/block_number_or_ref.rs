use crate::config::{ Config, HashFor, Hasher };
use crate::backend::BlockRef;

/// This represents either a block number or a reference 
/// to a block, which is essentially a block hash.
pub enum BlockNumberOrRef<T: Config> {
    /// A block number.
    Number(u64),
    /// A block ref / hash.
    BlockRef(BlockRef<HashFor<T>>)
}

impl <T: Config> From<u32> for BlockNumberOrRef<T> {
    fn from(value: u32) -> Self {
        BlockNumberOrRef::Number(value.into())
    }
}

impl <T: Config> From<u64> for BlockNumberOrRef<T> {
    fn from(value: u64) -> Self {
        BlockNumberOrRef::Number(value)
    }
}

impl <T: Config> From<BlockRef<HashFor<T>>> for BlockNumberOrRef<T> {
    fn from(block_ref: BlockRef<HashFor<T>>) -> Self {
        BlockNumberOrRef::BlockRef(block_ref)
    }
}

// Ideally we'd have `impl From<HashFor<T>> for BlockNumberOrRef<T>` but since our config
// could set _any_ hash type, this boils down to `impl From<H> for ..` which is too general.
// Thus, we target our current concrete hash type.
impl <T: Config> From<crate::config::substrate::H256> for BlockNumberOrRef<T> 
where
    <T::Hasher as Hasher>::Hash: From<crate::config::substrate::H256>
{
    fn from(hash: crate::config::substrate::H256) -> Self {
        BlockNumberOrRef::BlockRef(BlockRef::from_hash(hash.into()))
    }
}
