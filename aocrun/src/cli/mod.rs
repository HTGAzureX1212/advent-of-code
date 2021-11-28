use std::process;

pub mod common;

use common::AdvancedRunOption;

pub const WELCOME: &'static str =
r"Welcome to the Advent of Code Solution Runner.

This will build the `aocsol` binary with the corresponding year and day as configured below, as
well as running the solution and providing the output.
";

pub struct ExitCode(pub i32);

pub struct RunOptions {
    pub year: Option<u16>,
    pub day: Option<u8>,
    pub bench: Option<bool>
}

pub fn run(mut opts: RunOptions) -> anyhow::Result<ExitCode> {
    println!("{WELCOME}");

    loop {
        println!("{}", current_runopts(&opts));

        match common::advanced_opts()? {
            AdvancedRunOption::RunAsIs => {
                opts.year = Some(2015);
                opts.day = Some(1);
                opts.bench = Some(false);

                break;
            }
            AdvancedRunOption::RunWithCustomOptions => {
                opts = customize_runopts(opts)?;
            }
            AdvancedRunOption::AbortRun => {
                println!("aborting run");

                return Ok(ExitCode(-1));
            }
        }
    }

    println!("stage 1: compile `aocsol` binary");
    let mut command = process::Command::new("cargo");
    command
        .arg("build")
        .arg("--package")
        .arg("aocsol")
        .arg("--features")
        .arg(format!("aocsol/year{}-day{}", opts.year.unwrap(), opts.day.unwrap()))
        .arg("--no-default-features")
        .arg("--release");

    if !command
        .status()
        .expect(&format!("failed to execute command `{command:?}`"))
        .success() {
        eprintln!("could not compile `aocsol` binary.");

        return Ok(ExitCode(-1));
    }

    println!();
    println!("stage 2: run `aocsol` binary");
    let mut command = process::Command::new("./target/release/aocsol");

    if !command
        .status()
        .expect(&format!("failed to run command `{command:?}`"))
        .success() {
        eprintln!("could not run `aocsol` binary.");

        return Ok(ExitCode(-1));
    }

    return Ok(ExitCode(0));
}

fn current_runopts(opts: &RunOptions) -> String {
    format!(r"current run options:

- Advent of Code Event Year: {}
- Advent of Code Event Day : {}
- Benchmark Solution       : {}
",
            opts.year.unwrap_or(2015),
            opts.day.unwrap_or(1),
            opts.bench.unwrap_or(false)
    )
}

fn customize_runopts(mut opts: RunOptions) -> anyhow::Result<RunOptions> {
    println!("Questions will now be asked for the values of each run option.");
    println!("You may press the ENTER key to leave the value unchanged for a specific option.");

    println!();

    let year = common::ask("What is the Advent of Code event year? (2015..=2021)", "2021")?;
    let day = common::ask("What is the Advent of Code event day? (1..=25)", "1")?;
    let bench = common::ask("Do you want to benchmark the solution? (true / false)", "false")?;

    opts.year.replace(year.parse().unwrap());
    opts.day.replace(day.parse().unwrap());
    opts.bench.replace(match &*bench {
        "true" => true,
        "false" => false,
        _ => false
    });

    Ok(opts)
}
