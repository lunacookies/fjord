use {anyhow::Result, std::io};

const PROMPT: &'static str = "→ ";

fn main() -> Result<()> {
    use std::process::Command;

    loop {
        display_prompt()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut words = input.split_ascii_whitespace();

        // We have already ensured that the input is not empty, so there must be at least a first
        // item in this iterator;
        let command = words.next().unwrap();

        let status = Command::new(command)
            .args(words.collect::<Vec<_>>())
            .status()?;

        println!("{} exited with code {}", command, status);
    }
}

fn display_prompt() -> Result<()> {
    use std::io::Write;

    let mut stdout = io::stdout();

    stdout.write_all(PROMPT.as_bytes())?;
    stdout.flush()?;

    Ok(())
}
