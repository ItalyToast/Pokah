use crate::{rank::Rank, rankset::RankSet};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Straight {
    RANK_A_LOW = 0,
    RANK_2 = 1,
    RANK_3 = 2,
    RANK_4 = 3,
    RANK_5 = 4,
    RANK_6 = 5,
    RANK_7 = 6,
    RANK_8 = 7,
    RANK_9 = 8,
    RANK_T = 9,
    RANK_J = 10,
    RANK_Q = 11,
    RANK_K = 12,
    RANK_A_HIGH = 13,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct StraightSet {
    mask: u16
}

impl StraightSet {
    pub fn from_rankset(set: RankSet) -> Self{
        let ace = set.contains(Rank::RANK_A);
        Self { mask: set.mask << 1 | (if ace {1} else {0})}
    }

    pub fn len(&self) -> u32 {
        u16::count_ones(self.mask >> 1)
    }
}