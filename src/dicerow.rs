use std::fmt::Display;

use crate::die::Die;

pub struct DiceRow {
    row: [Die; 5],
}
impl DiceRow {
    pub fn roll() -> Self {
        let roll = || Die::roll();
        let row = [roll(), roll(), roll(), roll(), roll()];
        Self { row }
    }
    pub fn reroll(&mut self, held: [bool; 5]) {
        self.row.iter_mut().zip(held).for_each(|(d, h)| {
            if !h {
                *d = Die::roll()
            }
        })
    }
    pub fn into_inner(self) -> [Die; 5] {
        self.row
    }

    pub fn as_inner(&self) -> &[Die; 5] {
        &self.row
    }

    pub fn to_u8(&self) -> [u8; 5] {
        self.into()
    }
}
impl From<&DiceRow> for [u8; 5] {
    fn from(value: &DiceRow) -> Self {
        value.as_inner().clone().map(|d| d.to_u8())
    }
}
impl Display for DiceRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {}", self.to_u8().map(|n| n.to_string()).join("    "))
    }
}

impl TryFrom<[u8; 5]> for DiceRow {
    type Error = ();

    fn try_from(value: [u8; 5]) -> Result<Self, Self::Error> {
        // turn to dice row
        let result: Result<[Die; 5], ()> = value
            .iter()
            .map(|&v| Die::try_from(v))
            .collect::<Result<Vec<Die>, ()>>()
            .map(|v| {
                let v: [Die; 5] = v.try_into().unwrap();
                v
            });
        // create new
        result.map(|row| Self { row })
    }
}
