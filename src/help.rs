const HELP: &str = "COMMANDS:
-----
roll -- Randomizes the dice that are not held.
hold (1..5)... -- Hold or unhold the dice.
score (category) -- Score a category.
cats -- List the categories.
-----";

pub fn help(args: &[&str]) -> &'static str {
    match args {
        _ => HELP,
    }
}
