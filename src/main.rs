use std::path::PathBuf;

use clap::{Arg, ArgAction, Command, arg, command, value_parser};

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
                    arg!(-d --duration "Duration of the focus phase")
                        .required(false)
                )
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
        Some(("start", sub_matches)) => println!(
            "'FocusCLI start' was used, the task is {:?}",
            sub_matches.get_one::<String>("task")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
    }
}