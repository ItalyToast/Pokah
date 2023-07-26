use crate::{rankset::RankSet, suitset::SuitSet, card::Card, suit::Suit};

struct CardExpander {
    ranks: RankSet,
    suits: SuitSet,
}

impl CardExpander {
    pub fn new(ranks: RankSet, suits: SuitSet) -> CardExpander {
        Self{ranks, suits}
    }

    pub fn expand(&self) -> Vec<Card> {
        let mut result = Vec::<Card>::new();
        for s in self.suits {
            for r in self.ranks {
                result.push(Card::from_u8(s as u8 * 13 + r as u8))
            }
        }
        result
    }

    pub fn from_card(card: Card) -> CardExpander {
        let ranks = RankSet::default().insert(card.rank());
        let suits = SuitSet::default().insert(card.suit());
        
        CardExpander { ranks, suits }
    }
}