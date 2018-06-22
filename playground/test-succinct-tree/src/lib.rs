#[macro_use]
extern crate serde_derive; // 1.0.66
extern crate serde; // 1.0.66
extern crate bio;
extern crate bv;

use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;

#[derive(Serialize, Deserialize)]
pub struct LOUDS<L> {
    #[serde(with = "LOUDSBits")]
    rank_select: RankSelect,
    label: Vec<L>
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
