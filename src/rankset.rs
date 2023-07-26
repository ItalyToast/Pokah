use crate::rank::Rank;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct RankSet {
    pub(crate) mask: u16
}

impl RankSet {
    pub fn contains(&self, rank: Rank) -> bool {
        (self.mask & 1 << rank as u16) != 0
    }

    pub fn insert(&self, rank: Rank) -> Self {
        Self { mask: self.mask | (1 << rank as u64) }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self{ mask: self.mask & other.mask }
    }

    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    pub fn is_disjoint(&self, other: Self) -> bool {
        self.mask & other.mask == 0
    }

    pub fn is_subset(&self, other: Self) -> bool {
        self.mask & other.mask == self.mask
    }

    pub fn is_superset(&self, other: Self) -> bool {
        self.mask & other.mask == other.mask
    }

    pub fn len(&self) -> u32 {
        self.mask.count_ones()
    }

    pub fn remove(&mut self, rank: Rank) {
        self.mask = !(self.mask & 1 << rank as u16);
    }

    pub fn symmetric_difference(&self, other: Self) -> Self{
        let common = self.mask & other.mask;
        Self {mask : (self.mask & !common) | (other.mask & !common)}
    }

    pub fn union(&self, other: Self) -> Self {
        Self { mask: self.mask | other.mask }
    }
}

impl FromIterator<Rank> for RankSet {
    fn from_iter<T: IntoIterator<Item = Rank>>(iter: T) -> Self {
        let mut mask = 0u16;
        for item in iter {
            mask |= item as u16;
        }
        Self { mask }
    }
}

pub struct RankSetIter {
    mask: u16,
    index: u8,
}

impl Iterator for RankSetIter{
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 13 {
            if self.mask & 1 == 1 {
                return Some(Rank::from_u8(self.index));
            }
            self.index += 1;
            self.mask = self.mask >> 1;
        }
        None
    }
}

impl IntoIterator for RankSet {
    type Item = Rank;

    type IntoIter = RankSetIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{mask: self.mask, index: 0}
    }
}

impl std::fmt::Debug for RankSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RankSet").field("mask", &self.mask).finish()
    }
}