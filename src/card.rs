use crate::{error::GenericError, rank::Rank, suit::Suit};


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Card
{
    CARD_2S = 0,
    CARD_3S = 1,
    CARD_4S = 2,
    CARD_5S = 3,
    CARD_6S = 4,
    CARD_7S = 5,
    CARD_8S = 6,
    CARD_9S = 7,
    CARD_TS = 8,
    CARD_JS = 9,
    CARD_QS = 10,
    CARD_KS = 11,
    CARD_AS = 12,
    CARD_2H = 13,
    CARD_3H = 14,
    CARD_4H = 15,
    CARD_5H = 16,
    CARD_6H = 17,
    CARD_7H = 18,
    CARD_8H = 19,
    CARD_9H = 20,
    CARD_TH = 21,
    CARD_JH = 22,
    CARD_QH = 23,
    CARD_KH = 24,
    CARD_AH = 25,
    CARD_2C = 26,
    CARD_3C = 27,
    CARD_4C = 28,
    CARD_5C = 29,
    CARD_6C = 30,
    CARD_7C = 31,
    CARD_8C = 32,
    CARD_9C = 33,
    CARD_TC = 34,
    CARD_JC = 35,
    CARD_QC = 36,
    CARD_KC = 37,
    CARD_AC = 38,
    CARD_2D = 39,
    CARD_3D = 40,
    CARD_4D = 41,
    CARD_5D = 42,
    CARD_6D = 43,
    CARD_7D = 44,
    CARD_8D = 45,
    CARD_9D = 46,
    CARD_TD = 47,
    CARD_JD = 48,
    CARD_QD = 49,
    CARD_KD = 50,
    CARD_AD = 51
}

impl Card {
    pub fn from_u8(card: u8) -> Self{
        unsafe { std::mem::transmute(card) }
    }

    pub fn rank(self) -> Rank {
        Rank::from_u8(self as u8 % 13)
    }

    pub fn suit(self) -> Suit {
        Suit::from_u8(self as u8 / 13)
    }
}

impl std::str::FromStr for Card {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str()
        {
            "3S" => Ok(Card::CARD_3S),
            "4S" => Ok(Card::CARD_4S),
            "5S" => Ok(Card::CARD_5S),
            "6S" => Ok(Card::CARD_6S),
            "2S" => Ok(Card::CARD_2S),
            "7S" => Ok(Card::CARD_7S),
            "8S" => Ok(Card::CARD_8S),
            "9S" => Ok(Card::CARD_9S),
            "TS" => Ok(Card::CARD_TS),
            "JS" => Ok(Card::CARD_JS),
            "QS" => Ok(Card::CARD_QS),
            "KS" => Ok(Card::CARD_KS),
            "AS" => Ok(Card::CARD_AS),
            "2H" => Ok(Card::CARD_2H),
            "3H" => Ok(Card::CARD_3H),
            "4H" => Ok(Card::CARD_4H),
            "5H" => Ok(Card::CARD_5H),
            "6H" => Ok(Card::CARD_6H),
            "7H" => Ok(Card::CARD_7H),
            "8H" => Ok(Card::CARD_8H),
            "9H" => Ok(Card::CARD_9H),
            "TH" => Ok(Card::CARD_TH),
            "JH" => Ok(Card::CARD_JH),
            "QH" => Ok(Card::CARD_QH),
            "KH" => Ok(Card::CARD_KH),
            "AH" => Ok(Card::CARD_AH),
            "2C" => Ok(Card::CARD_2C),
            "3C" => Ok(Card::CARD_3C),
            "4C" => Ok(Card::CARD_4C),
            "5C" => Ok(Card::CARD_5C),
            "6C" => Ok(Card::CARD_6C),
            "7C" => Ok(Card::CARD_7C),
            "8C" => Ok(Card::CARD_8C),
            "9C" => Ok(Card::CARD_9C),
            "TC" => Ok(Card::CARD_TC),
            "JC" => Ok(Card::CARD_JC),
            "QC" => Ok(Card::CARD_QC),
            "KC" => Ok(Card::CARD_KC),
            "AC" => Ok(Card::CARD_AC),
            "2D" => Ok(Card::CARD_2D),
            "3D" => Ok(Card::CARD_3D),
            "4D" => Ok(Card::CARD_4D),
            "5D" => Ok(Card::CARD_5D),
            "6D" => Ok(Card::CARD_6D),
            "7D" => Ok(Card::CARD_7D),
            "8D" => Ok(Card::CARD_8D),
            "9D" => Ok(Card::CARD_9D),
            "TD" => Ok(Card::CARD_TD),
            "JD" => Ok(Card::CARD_JD),
            "QD" => Ok(Card::CARD_QD),
            "KD" => Ok(Card::CARD_KD),
            "AD" => Ok(Card::CARD_AD),
            _ => Err(GenericError{message: format!("Could not parse: {}", s)}),
        }
    }
}

