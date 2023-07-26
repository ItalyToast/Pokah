use std::array::IntoIter;

use crate::card::Card;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct CardSet {
    mask: u64
}

impl CardSet {
    pub fn contains(&self, card: Card) -> bool {
        (self.mask & 1 << card as u64) != 0
    }

    pub fn insert(&self, card: Card) -> Self {
        Self { mask: self.mask | (1 << card as u64) }
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

    pub fn remove(&mut self, card: Card) {
        self.mask = !(self.mask & 1 << card as u64);
    }

    pub fn symmetric_difference(&self, other: Self) -> Self{
        let common = self.mask & other.mask;
        Self {mask : (self.mask & !common) | (other.mask & !common)}
    }

    pub fn union(&self, other: Self) -> Self {
        Self { mask: self.mask | other.mask }
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut mask = 0u64;
        for item in iter {
            mask |= item as u64;
        }
        Self { mask }
    }
}

pub struct CardSetIter {
    mask: u64,
    index: u8,
}

impl Iterator for CardSetIter{
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 52 {
            if self.mask & 1 == 1 {
                return Some(Card::from_u8(self.index));
            }
            self.index += 1;
            self.mask = self.mask >> 1;
        }
        None
    }
}

impl IntoIterator for CardSet {
    type Item = Card;

    type IntoIter = CardSetIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{mask: self.mask, index: 0}
    }
}

impl std::fmt::Debug for CardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardSet").field("mask", &self.mask).finish()
    }
}