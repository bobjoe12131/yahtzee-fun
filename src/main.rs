use std::io::{self, stdin};

mod categories;
mod dicerow;
mod die;
mod help;
mod yahtzee;

fn main() {
    let mut yahtzee = yahtzee::Yahtzee::new();
    loop {
        println!("{yahtzee}");
        let Ok(input) = read_line() else {
            break;
        };
        let input = input.trim();

        match &input.to_lowercase().split(" ").collect::<Vec<&str>>()[..] {
            ["roll"] => _ = yahtzee.roll(),
            ["hold", hold @ ..] => {
                let parsed: Option<Vec<u8>> =
                    hold.into_iter().map(|&die| die.parse().ok()).collect();
                yahtzee.hold(parsed);
            }
            ["score", category] => yahtzee.score(category),
            ["cats"] => println!("{}", yahtzee.categories),
            ["help", args @ ..] => println!("{}", help::help(args)),
            [""] | [] => yahtzee.message("No command was put in.".to_string()),
            [command, ..] => yahtzee.message(format!("{command} is not a command.")),
        };
    }
}

fn read_line() -> io::Result<String> {
    let mut temp = String::new();
    stdin().read_line(&mut temp).map(|_| temp)
}
