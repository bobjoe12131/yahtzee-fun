use std::fmt::Display;

use crate::{
    categories::{Categories, ScoreResult},
    dicerow::DiceRow,
};

#[derive(PartialEq)]
pub enum Rolls {
    Three = 3,
    Two = 2,
    One = 1,
    Zero = 0,
}
impl Rolls {
    /// returns false and does not increment if self is Zero
    fn decrement(&mut self) -> bool {
        use Rolls as R;
        *self = match *self {
            R::Three => R::Two,
            R::Two => R::One,
            R::One => R::Zero,
            R::Zero => return false,
        };
        return true;
    }
    fn to_u8(&self) -> u8 {
        u8::from(self)
    }
}
impl From<&Rolls> for u8 {
    fn from(value: &Rolls) -> Self {
        match value {
            Rolls::Three => 3,
            Rolls::Two => 2,
            Rolls::One => 1,
            Rolls::Zero => 0,
        }
    }
}
impl Display for Rolls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u8())
    }
}

pub enum LastCommand {
    Roll(bool),
    Hold(Option<Vec<u8>>),
    Score(Option<ScoreResult>),
    Message(String),
    None,
}

pub struct Yahtzee {
    pub row: DiceRow,
    pub held: [bool; 5],
    pub rolls_left: Rolls,
    pub score: u16,
    pub categories: Categories,
    pub last_command: LastCommand,
    pub toggle_display_categories: bool,
}
impl Yahtzee {
    pub fn new() -> Self {
        Self {
            row: DiceRow::roll(),
            held: [false; 5],
            rolls_left: Rolls::Two,
            score: 0,
            categories: Categories::new(),
            last_command: LastCommand::None,
            toggle_display_categories: false,
        }
    }

    /// returns true if roll happens
    /// returns false if 0 rolls.
    pub fn roll(&mut self) {
        self.last_command = LastCommand::Roll(
            self.rolls_left
                .decrement()
                .then(|| self.row.reroll(self.held))
                .is_some(),
        );
    }
    pub fn hold(&mut self, hold: Option<Vec<u8>>) {
        let Some(hold) = hold else {
            self.last_command = LastCommand::Hold(None);
            return;
        };
        self.last_command = LastCommand::Hold(if hold.iter().all(|i| *i >= 1 && *i <= 5) {
            for i in hold.iter() {
                self.held[*i as usize - 1] = !self.held[*i as usize - 1];
            }
            Some(hold)
        } else {
            None
        });
    }
    pub fn score(&mut self, name: &str) {
        let res = self.categories.score(name, &self.row);
        if let Some(ScoreResult::Score(score_opt)) = res {
            if let Some(score) = score_opt {
                self.score += score
            }
            self.held = [false; 5];
            self.row.reroll(self.held);
            self.rolls_left = Rolls::Three
        }
        self.last_command = LastCommand::Score(res);
    }
    pub fn message(&mut self, message: String) {
        self.last_command = LastCommand::Message(message);
    }
}

impl Display for Yahtzee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = &self.row;
        let held = self.held.map(|h| if h { "held" } else { "    " }).join(" ");
        let score = self.score;
        let rolls_left = &self.rolls_left;
        let message = match &self.last_command {
            LastCommand::Roll(did) => match self.rolls_left {
                _ if !did => "Roll failed. Out of rolls.".to_string(),
                Rolls::Two => "Two rolls left.".to_string(),
                Rolls::One => "One roll left.".to_string(),
                Rolls::Zero => "Zero rolls left.".to_string(),
                _ => unreachable!(),
            },
            LastCommand::Hold(hold) => match hold {
                Some(list) => match &list[..] {
                    [] => "Nothing has been (un)held.".to_string(),
                    [one] => format!("Die {one} has been (un)held."),
                    [r @ .., last] => {
                        format!(
                            "Dice {} and {last} have been (un)held.",
                            r.iter()
                                .cloned()
                                .map(|n| n.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    }
                },
                None => "Invalid hold command. Use numbers between 1 and 5 seperated by spaces."
                    .to_string(),
            },
            LastCommand::Score(score) => match score {
                Some(score) => match score {
                    ScoreResult::Score(score) => {
                        (match score {
                            Some(score) => format!("Scored {score}."),
                            None => "Scored zero.".to_string(),
                        } + " New turn.")
                    }
                    ScoreResult::AlreadyScored => "This category was already scored.".to_string(),
                },
                None => "This category was not found.".to_string(),
            },
            LastCommand::Message(message) => message.to_string(),
            LastCommand::None => "Enter 'help' for commands.".to_string(),
        }
        .to_string();
        write!(
            f,
            "{row}\n{held}\nScore: {score}\nRolls left: {rolls_left}\n{message}"
        )
    }
}
