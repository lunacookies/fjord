use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut state = libfjord::eval::State::new_root(vec![(
        libfjord::IdentName::new_panicking("editor"),
        Box::new(fjordstd::EditorFunc::new()),
    )]);

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        let mut s = String::new();
        stdin.read_line(&mut s)?;

        let eval_result = libfjord::eval(s.trim(), &mut state);

        match eval_result {
            Ok(output) => writeln!(stdout, "{}", output)?,
            Err(e) => writeln!(stderr, "Error: {}", e)?,
        }
    }
}
