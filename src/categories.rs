use std::fmt::Display;

use crate::dicerow::DiceRow;
use indexmap::{map::Slice, IndexMap};

macro_rules! insert_categories {
    {
        $hashmap:ident;
        $(
            $name:expr => {
                $check_fn:expr,
                $score_fn:expr,
            }
        )*
    } =>
        {
            $(
                $hashmap.insert(
                    $name.to_string(),
                    Category::new(
                        $name.to_string(),
                        $check_fn,
                        $score_fn,
                    )
                );
            )*
        };
}

macro_rules! upper {
    [$hashmap:ident; $($name:expr => $die:expr),* $(,)?] =>
    {
        insert_categories!{
            $hashmap;
            $(
            $name => {
                |row| check::upper(row, $die),
                |row| score::upper(row, $die),
            }
            )*
        };
    }

}

pub struct Categories(pub IndexMap<String, Category>);
impl Categories {
    pub fn new() -> Self {
        use crate::die::Die;
        let mut cats = IndexMap::new();
        upper! {
            cats;
            "1" => &Die::One,
            "2" => &Die::Two,
            "3" => &Die::Three,
            "4" => &Die::Four,
            "5" => &Die::Five,
            "6" => &Die::Six,
        }
        insert_categories! {
            cats;
            "3-kind" => {
                |row| check::n_of_a_kind(row, 3),
                |row| score::total(row),
            }
            "4-kind" => {
                |row| check::n_of_a_kind(row, 4),
                |row| score::total(row),
            }
            "house" => {
                |row| check::full_house(row),
                |_| 25,
            }
            "s-straight" => {
                |row| check::n_straight(row, 4),
                |_| 30,
            }
            "l-straight" => {
                |row| check::n_straight(row, 5),
                |_| 40,
            }
            "yahtzee" => {
                |row| check::n_of_a_kind(row, 5),
                |_| 50,
            }
            "chance" => {
                |_| true,
                |row| score::total(row),
            }
        }
        Self(cats)
    }
    pub fn score(&mut self, name: &str, row: &DiceRow) -> Option<ScoreResult> {
        self.0.get_mut(name).map(|c| c.score(row))
    }
}

impl Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_string = |sl: &Slice<String, Category>| {
            sl.values()
                .map(|c: &Category| c.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        };
        let upper = to_string(&self.0[..=5]);
        let lower = to_string(&self.0[6..]);

        write!(f, "Upper:\n{upper}\nLower:\n{lower}")
    }
}

pub enum ScoreResult {
    Score(Option<u16>),
    AlreadyScored,
}
pub enum ScoreStatus {
    Unscored,
    Scored(Option<u16>),
}
pub struct Category {
    pub name: String,
    pub status: ScoreStatus,
    check_fn: fn(&DiceRow) -> bool,
    score_fn: fn(&DiceRow) -> u16,
}
impl Category {
    fn new(name: String, check: fn(&DiceRow) -> bool, score: fn(&DiceRow) -> u16) -> Self {
        Category {
            name,
            status: ScoreStatus::Unscored,
            check_fn: check,
            score_fn: score,
        }
    }
    pub fn score(&mut self, row: &DiceRow) -> ScoreResult {
        match self.status {
            ScoreStatus::Unscored => {
                let score = (self.check_fn)(row).then(|| (self.score_fn)(row));
                self.status = ScoreStatus::Scored(score);
                ScoreResult::Score(score)
            }
            ScoreStatus::Scored(_) => ScoreResult::AlreadyScored,
        }
    }
}
impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.status {
            ScoreStatus::Unscored => "-".to_string(),
            ScoreStatus::Scored(score) => match score {
                Some(score) => score.to_string(),
                None => "Z".to_string(),
            },
        };
        write!(f, "{}: {}", self.name, status)
    }
}

pub mod check {
    use crate::{dicerow::DiceRow, die::Die};

    pub fn upper(row: &DiceRow, checking_die: &Die) -> bool {
        row.as_inner().contains(checking_die)
    }
    pub fn n_of_a_kind(row: &DiceRow, n: u8) -> bool {
        for checking_die in Die::VARIANTS {
            if row
                .as_inner()
                .iter()
                .filter(|&die| *die == checking_die)
                .count()
                >= n as usize
            {
                return true;
            }
        }
        false
    }
    pub fn full_house(row: &DiceRow) -> bool {
        let mut three = false;
        let mut two = false;

        for checking_die in Die::VARIANTS {
            let count = row
                .as_inner()
                .iter()
                .filter(|&die| *die == checking_die)
                .count();
            if count == 3 {
                three = true
            } else if count == 2 {
                two = true
            }
        }
        three && two
    }
    pub fn n_straight(row: &DiceRow, n: u8) -> bool {
        let mut row: Vec<_> = row.to_u8().to_vec();
        row.sort();
        row.dedup();

        let mut max_count = 0;
        let mut current_count = 1;

        for i in 1..row.len() {
            if row[i] == row[i - 1] + 1 {
                current_count += 1;
            } else {
                max_count = max_count.max(current_count);
                current_count = 1;
            }
        }

        max_count = max_count.max(current_count);

        max_count == n
    }
}

pub mod score {
    use crate::{dicerow::DiceRow, die::Die};

    pub fn upper(row: &DiceRow, scoring_die: &Die) -> u16 {
        let temp: u16 = row
            .as_inner()
            .iter()
            .filter(|&num| num == scoring_die)
            .count()
            .try_into()
            .unwrap();
        temp * scoring_die.to_u8() as u16
    }
    pub fn total(row: &DiceRow) -> u16 {
        row.as_inner()
            .iter()
            .fold(0, |score, die| score + die.to_u8() as u16)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_straight() {
        use crate::categories::check::n_straight;
        use crate::dicerow::DiceRow;
        let mut sorted = DiceRow::roll();

        assert_eq!(n_straight(&[1, 2, 3, 4, 1].try_into().unwrap(), 4), true);
        assert_eq!(n_straight(&[1, 2, 2, 3, 4].try_into().unwrap(), 4), true);
        assert_eq!(n_straight(&[1, 2, 4, 4, 5].try_into().unwrap(), 4), false);
        assert_eq!(n_straight(&[1, 2, 3, 4, 5].try_into().unwrap(), 4), false);
    }
}
