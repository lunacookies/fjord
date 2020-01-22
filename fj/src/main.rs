use anyhow::Result;

const PROMPT: &'static str = "→ ";

fn main() -> Result<()> {
    use {
        fj::LexItemsIter,
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

                // We collect into a vector and convert back into an iterator to pull each lex
                // item’s Result out and around the collection.
                let lex_items: std::result::Result<Vec<_>, _> = LexItemsIter::new(&i).collect();
                let lex_items = lex_items?;
                let mut lex_items = lex_items.iter();

                let command = lex_items.next().unwrap();

                let status = Command::new(command.as_ref())
                    .args(lex_items.map(|s| s.as_ref()))
                    .status()?;

                if !status.success() {
                    println!("{}: {}", command, status);
                }
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
