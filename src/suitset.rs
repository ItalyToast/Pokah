use crate::suit::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct SuitSet {
    mask: u8
}

impl SuitSet {
    pub fn contains(&self, suit: Suit) -> bool {
        (self.mask & 1 << suit as u8) != 0
    }

    pub fn insert(&self, suit: Suit) -> Self {
        Self { mask: self.mask | (1 << suit as u64) }
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

    pub fn remove(&mut self, suit: Suit) {
        self.mask = !(self.mask & 1 << suit as u8);
    }

    pub fn symmetric_difference(&self, other: Self) -> Self{
        let common = self.mask & other.mask;
        Self {mask : (self.mask & !common) | (other.mask & !common)}
    }

    pub fn union(&self, other: Self) -> Self {
        Self { mask: self.mask | other.mask }
    }
}

impl FromIterator<Suit> for SuitSet {
    fn from_iter<T: IntoIterator<Item = Suit>>(iter: T) -> Self {
        let mut mask = 0u8;
        for item in iter {
            mask |= item as u8;
        }
        Self { mask }
    }
}

pub struct SuitSetIter {
    mask: u8,
    index: u8,
}

impl Iterator for SuitSetIter{
    type Item = Suit;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 4 {
            if self.mask & 1 == 1 {
                return Some(Suit::from_u8(self.index));
            }
            self.index += 1;
            self.mask = self.mask >> 1;
        }
        None
        
    }
}

impl IntoIterator for SuitSet {
    type Item = Suit;

    type IntoIter = SuitSetIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{mask: self.mask, index: 0}
    }
}

impl std::fmt::Debug for SuitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SuitSet").field("mask", &self.mask).finish()
    }
}