use std::io::{
    self,
    BufRead,
    Write
};

use anyhow::Context;

pub enum AdvancedRunOption {
    RunAsIs,
    RunWithCustomOptions,
    AbortRun
}

pub fn advanced_opts() -> anyhow::Result<AdvancedRunOption> {
    println!("1            : proceed with the default run options (default)");
    println!("2            : customize the run options");
    println!("anything else: abort run");

    print!("$ ");

    let _ = io::stdout().flush();
    let input = readln()?;

    let retval = match &*input {
        "1" | "" => AdvancedRunOption::RunAsIs,
        "2" => AdvancedRunOption::RunWithCustomOptions,
        _ => AdvancedRunOption::AbortRun
    };

    println!();

    Ok(retval)
}

pub fn ask(question: &str, default: &str) -> anyhow::Result<String> {
    println!("{question} [default: {default}]");

    let _ = io::stdout().flush();
    let input = readln()?;

    println!();

    if input.is_empty() {
        return Ok(default.to_string())
    }

    Ok(input)
}

pub fn readln() -> anyhow::Result<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let lines = lines.next().transpose()?;

    match lines {
        Some(value) => Ok(value),
        None => Err(anyhow::anyhow!("no lines from standard input is detected"))
    }
        .context("unable to read from standard input")
}
