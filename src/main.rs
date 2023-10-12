use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

mod helpers;
use helpers::command_line::PrintCommand;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Time in minutes
    #[structopt(default_value = "120")]
    t: u64,

    /// Kill any existing `caffeinate` processes
    #[structopt(short, long)]
    kill: bool,

    #[structopt(short, long)]
    list: bool,
}

fn main() {
    let opt = Opt::from_args();
    let filepath = "/tmp/nosleep_timestamp";

    // If -k or --kill is provided
    if opt.kill {
        let _ = Command::new("pkill").arg("caffeinate").output();
        let _ = fs::remove_file(filepath); // Remove the timestamp file.
        PrintCommand::Issue.print_nosleep_message("Killed any existing `caffeinate` processes.");
        return;
    }

    // If -l or --list is provided
    if opt.list {
        if let Ok(contents) = fs::read_to_string(filepath) {
            let lines: Vec<&str> = contents.split("\n").collect();
            if lines.len() == 2 {
                if let (Ok(start_time), Ok(duration)) =
                    (lines[0].parse::<u64>(), lines[1].parse::<u64>())
                {
                    let elapsed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        - start_time;
                    if elapsed < duration {
                        let all_time_left_seconds = duration - elapsed;
                        let time_left_minutes = all_time_left_seconds / 60;
                        let mod_seconds_left = all_time_left_seconds % 60;
                        PrintCommand::print_time_left(&time_left_minutes, &mod_seconds_left);
                    } else {
                        println!("No active `nosleep` process.");
                    }
                }
            }
        } else {
            PrintCommand::Info.print_nosleep_message("No active `nosleep` process.");
        }
        return;
    }

    let time_seconds = opt.t * 60;

    // Store the current timestamp and duration in a file
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    fs::write(filepath, format!("{}\n{}", now, time_seconds)).expect("Unable to write to file");

    // Kill any existing `caffeinate` processes.
    let _ = Command::new("pkill").arg("caffeinate").output();

    // Build the `caffeinate` command.
    let mut command = Command::new("caffeinate");
    command.arg("-d").arg("-t").arg(time_seconds.to_string());

    // Spawn in the background.
    let child = command.spawn();

    match child {
        Ok(_) => {
            PrintCommand::print_time_start(&opt.t);
        }
        Err(e) => {
            eprintln!("Failed to execute `caffeinate` command: {}", e);
        }
    }
}
