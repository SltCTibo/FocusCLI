mod timer;
mod graphic;
mod input;

use std::{path::PathBuf, sync::mpsc, time::{Duration, Instant}};

use clap::{Command, arg, command, value_parser};

use crate::timer::{Timer, TimerStatus};

fn main() {
    let matches = command!()
        .arg(
            arg!(-c --config <FILE> "Sets a custom file")
            .required(false)
            .value_parser(value_parser!(PathBuf))
        )
        .subcommand(
            Command::new("start")
                .about("Start the focus mode")
                .arg(
                    arg!(<task> "Task to start")
                        .required(true)
                )
                .arg(
                    arg!(-d --duration <DURATION> "Duration of the focus phase")
                        .required(false)
                        .value_parser(value_parser!(u8).range(1..))
                )
                .arg(
                    arg!(-b --break <BREAK> "Configure the break between 2 sessions")
                        .required(false)
                        .value_parser(value_parser!(u8).range(1..))
                )
        )
        .subcommand(
            Command::new("pause")
                .about("Pause the current task")
        )
        .subcommand(
            Command::new("stop")
                .about("Stop the current task")
        )
        .subcommand(
            Command::new("status")
                .about("Get the task status")
        )
        .subcommand(
            Command::new("stats")
                .about("Get the focus stats of the tasks")
                .arg(arg!([period] "Time period (today, week, month)"))
    ).get_matches();

    // You can check the value provided by positional arguments, or options arguments
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            println!("'FocusCLI start' was used, the task is {:?}", sub_matches.get_one::<String>("task"));
            let mut duration: u8 = 10;
            if let Some(d) = sub_matches.get_one::<u8>("duration") {
                duration = *d;
            }

            let mut dbreak: u8 = 10;
            if let Some(b) = sub_matches.get_one::<u8>("break") {
                dbreak = *b;
            }

            println!("With a focus duration of {duration} minutes");
            println!("And a break of {dbreak} minutes between focuses");

            let mut timer = Timer::new(duration);

            let (tx, rx) = mpsc::channel();
            let (status_tx, status_rx) = mpsc::channel();

            timer.start(tx, status_rx);

            while timer.status != TimerStatus::Finished {
                if let Ok(time_left) = rx.recv() {
                    print!("\rTime left: {}s ", time_left);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();
                    if time_left == 0 {
                        status_tx.send(TimerStatus::Finished).unwrap();
                        timer.set_status(TimerStatus::Finished);
                    }
                }
            }
        },
        Some(("pause", _)) => println!(
            "'FocusCLI pause' was used, the task is currently paused"
        ),
        Some(("stop", _)) => print!(
            "'FocusCLI stop' was used, the task {:?} stopped",
            "task"
        ),
        Some(("status", _)) => println!(
            "'FocusCLI status' was used, showing the focus status"
        ),
        Some(("stats", sub_matches)) => println!(
            "'FocusCLI stats' was used for the period: {:?}",
            sub_matches.get_one::<String>("period")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
    }
}

/*
    Start a session 3 boucles:
        - Graphic module
        - Timer module
        - User input module
*/