use {anyhow::Result, fj::LexItem, std::path::PathBuf};

const PROMPT_CHAR: &'static str = "→ ";

fn main() -> Result<()> {
    use {
        fj::LexItemsIter,
        rustyline::{error::ReadlineError, Editor},
        std::{env, process::Command},
    };

    let mut rl = Editor::<()>::new();
    let mut pwd = env::current_dir()?;

    loop {
        let prompt = format!("{} {}", pwd.display(), PROMPT_CHAR);
        let input = rl.readline(&prompt);

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

                match command.as_ref() {
                    "cd" => cd(lex_items, &mut pwd),
                    command => {
                        let status = Command::new(command)
                            .args(lex_items.map(|s| s.as_ref()))
                            .current_dir(&pwd)
                            .status()?;

                        if !status.success() {
                            println!("{}: {}", command, status);
                        }
                    }
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

fn cd<'a>(mut args: impl Iterator<Item = &'a LexItem<'a>>, pwd: &mut PathBuf) {
    let target_path =
        args.next()
            .map(|l| PathBuf::from(l.as_ref()))
            .unwrap_or(match dirs::home_dir() {
                Some(d) => d,
                None => {
                    eprintln!("cd: Home directory could not be determined.");
                    return;
                }
            });

    if target_path.exists() {
        *pwd = target_path;
    } else {
        eprintln!(
            "cd: Target path ‘{}’ does not exist.",
            target_path.display()
        );
    }
}
