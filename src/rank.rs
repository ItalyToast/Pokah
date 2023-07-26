use crate::error::GenericError;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rank
{
    RANK_2 = 0,
    RANK_3 = 1,
    RANK_4 = 2,
    RANK_5 = 3,
    RANK_6 = 4,
    RANK_7 = 5,
    RANK_8 = 6,
    RANK_9 = 7,
    RANK_T = 8,
    RANK_J = 9,
    RANK_Q = 10,
    RANK_K = 11,
    RANK_A = 12,
}

impl Rank {
    pub fn from_u8(rank: u8) -> Self{
        unsafe { std::mem::transmute(rank) }
    }
}

impl std::str::FromStr for Rank {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str()
        {
            "2" => Ok(Rank::RANK_2),
            "3" => Ok(Rank::RANK_3),
            "4" => Ok(Rank::RANK_4),
            "5" => Ok(Rank::RANK_5),
            "6" => Ok(Rank::RANK_6),
            "7" => Ok(Rank::RANK_7),
            "8" => Ok(Rank::RANK_8),
            "9" => Ok(Rank::RANK_9),
            "T" => Ok(Rank::RANK_T),
            "J" => Ok(Rank::RANK_J),
            "Q" => Ok(Rank::RANK_Q),
            "K" => Ok(Rank::RANK_K),
            "A" => Ok(Rank::RANK_A),
            _ => Err(GenericError{message: format!("Could not parse: {}", s)}),
        }
    }
}