use anyhow::Result;

const PROMPT: &'static str = "→ ";

fn main() -> Result<()> {
    use {
        rustyline::{error::ReadlineError, Editor},
        std::process::Command,
    };

    let mut rl = Editor::<()>::new();

    loop {
        let input = rl.readline(PROMPT);

        match input {
            Ok(i) => {
                if i.is_empty() {
                    continue;
                }

                let mut words = i.split_ascii_whitespace();

                // We have already ensured that the input is not empty, so there must be at least a
                // first item in this iterator;
                let command = words.next().unwrap();

                let status = Command::new(command)
                    .args(words.collect::<Vec<_>>())
                    .status()?;

                println!("{} exited with code {}", command, status);
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }

    Ok(())
}
