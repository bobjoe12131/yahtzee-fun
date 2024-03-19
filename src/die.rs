use rand::Rng;
use std::fmt::Display;

use rand::distributions::{Distribution, Standard};

#[derive(PartialEq, Debug, Clone)]
pub enum Die {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
impl Die {
    pub const VARIANTS: [Die; 6] = {
        use Die::*;
        [One, Two, Three, Four, Five, Six]
    };
    pub fn roll() -> Self {
        rand::thread_rng().gen()
    }
    pub fn to_u8(&self) -> u8 {
        u8::from(self)
    }
}
impl From<&Die> for u8 {
    fn from(value: &Die) -> Self {
        match value {
            Die::One => 1,
            Die::Two => 2,
            Die::Three => 3,
            Die::Four => 4,
            Die::Five => 5,
            Die::Six => 6,
        }
    }
}
impl TryFrom<u8> for Die {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        use Die::*;
        Ok(match value {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            _ => return Err(()),
        })
    }
}
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Distribution<Die> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Die {
        match Die::try_from(rng.gen_range(1_u8..=6_u8)) {
            Ok(d) => d,
            Err(_) => unreachable!(),
        }
    }
}
