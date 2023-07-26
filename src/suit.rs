use crate::error::GenericError;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Spade = 0,
    Heart = 1,
    Club = 2,
    Diamond = 3,
}

impl Suit {
    pub fn from_u8(suit: u8) -> Self{
        unsafe { std::mem::transmute(suit) }
    }
}

impl std::str::FromStr for Suit {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str()
        {
            "s" => Ok(Suit::Spade),
            "h" => Ok(Suit::Heart),
            "d" => Ok(Suit::Diamond),
            "c" => Ok(Suit::Club),
            _ => Err(GenericError{message: format!("Could not parse: {}", s)}),
        }
    }
}