use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout, Stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    Info,
    Issue,
}

impl PrintCommand {
    pub fn print_time_left(time_left_minutes: &u64, mod_seconds_left: &u64) {
        let mut stdout: Stdout = stdout();
        // Decide on the print colour
        let colour: Color = Color::Cyan;
        // Print the agent_statement in a specific colour
        stdout
            .execute(SetForegroundColor(colour))
            .unwrap();
        println!(
            "[NOSLEEP]: Time left: {} minutes and {} seconds",
            time_left_minutes, mod_seconds_left
        );

        // Reset the colour
        stdout.execute(ResetColor).unwrap();
    }

    pub fn print_time_start(time_minutes: &u64) {
        let mut stdout: Stdout = stdout();
        // Decide on the print colour
        let colour: Color = Color::DarkGreen;
        // Print the agent_statement in a specific colour
        stdout
            .execute(SetForegroundColor(colour))
            .unwrap();
        println!(
            "[NOSLEEP]: Preventing sleep for {} minutes.",
            time_minutes
        );

        // Reset the colour
        stdout.execute(ResetColor).unwrap();
    }

    pub fn print_nosleep_message(&self, statement: &str) {
        let mut stdout: Stdout = stdout();
        // Decide on the print colour
        let colour: Color = match self {
            PrintCommand::Info => Color::Cyan,
            PrintCommand::Issue => Color::Red,
        };

        // Print the agent_statement in a specific colour
        stdout
            .execute(SetForegroundColor(colour))
            .unwrap();
        println!("[NOSLEEP]: {}", statement);

        // Reset the colour
        stdout.execute(ResetColor).unwrap();
    }
}