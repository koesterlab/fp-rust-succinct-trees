#[macro_use]
extern crate serde_derive; // 1.0.66
extern crate serde; // 1.0.66
extern crate bio;
extern crate bv;

pub mod louds;

use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;

#[derive(Serialize, Deserialize)]
pub struct LOUDS<L> {
    // The type L is automatically constrained to implement Serialize/Deserialize
    // If we don't want to serialize the whole RankSelect, we can create a proxy
    // struct LOUDSBits that only contains the bitvector. This proxy struct is used
    // for (de)serialization, and automatically converted into RankSelect with the implementation
    // of the From-Trait below.
    // Also see https://serde.rs/attributes.html.
    #[serde(with = "LOUDSBits")]
    rank_select: RankSelect,
    labels: Vec<L>
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "RankSelect")]
struct LOUDSBits {
    #[serde(getter = "RankSelect::bits")]
    inner: BitVec<u8>
}


impl From<LOUDSBits> for RankSelect {
    fn from(bits: LOUDSBits) -> RankSelect {
        let k = (bits.inner.len() as f64).ln().powi(2) as usize / 32;
        RankSelect::new(bits.inner, k)
    }
}
