use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;

#[derive(Serialize, Deserialize)]
pub struct LOUDS<L> {
    rank_select: RankSelect,
    labels: Vec<L>
}


impl<L> LOUDS<L> {
    pub fn from_bitvec(bits: BitVec<u8>, k: usize) -> Self {
        Self {
            rank_select: RankSelect::new(bits, k),
            labels: Vec::new()
        }
    }

    pub fn bits(&self) -> &BitVec<u8> {
        self.rank_select.bits()
    }

    pub fn labels(&self) -> &[L] {
        &self.labels
    }
}
