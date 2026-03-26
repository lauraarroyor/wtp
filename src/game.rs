use colored::{Color, ColoredString, Colorize};

use crate::{poke::PokeInfo, util::capitalized};

/// A game state.
#[derive(Clone, Debug)]
pub struct Game {
    /// The Pokémon info.
    info: PokeInfo,

    /// Is `true` when the game is over, `false` if not.
    is_finished: bool,
}

impl Game {
    /// Returns a fresh game state with the given Pokémon info.
    pub const fn new(info: PokeInfo) -> Self {
        Self {
            info,
            is_finished: false,
        }
    }

    /// Runs the game until it's over.
    pub fn run(&mut self) {
        while !self.is_finished {
            let input = Self::get_input();
            self.process(&input);
        }
    }

    /// Prompts the user for input and returns it.
    fn get_input() -> String {
        use std::io::stdin;

        println!(
            "{}",
            "Enter guess or info request:".color(Color::BrightBlack)
        );

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    /// Processes the given user input.
    fn process(&mut self, input: &str) {
        let output = if input.ends_with('?') {
            let len = input.len() - '?'.len_utf8();
            self.process_guess(&input[..len])
        } else {
            self.process_command(&input)
        };

        println!("{}", output);
    }

    /// Processes the given guess and returns the output.
    fn process_guess(&mut self, input: &str) -> ColoredString {
        if input == "?" || input == self.info.name() {
            self.is_finished = true;
            format!("The Pokémon was {}", capitalized(&self.info.name())).color(Color::Blue)
        } else {
            format!("The guess \"{input}\" is incorrect").color(Color::Yellow)
        }
    }

    /// Processes the given command/info request and returns the output.
    fn process_command(&mut self, input: &str) -> ColoredString {
        match input {
            "gen" | "region" => self.info.region().to_string(),
            "type" | "types" => self.info.types().to_string(),
            "stat" | "stats" => self.info.stats().to_string(),
            "ab" | "abs" | "ability" | "abilities" => self.info.abilities().to_string(),

            _ => format!("Unknown input: \"{input}\""),
        }
        .color(Color::Yellow)
    }
}
