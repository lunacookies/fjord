use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut states = vec![libfjord::eval::State::new()];
    let mut user_inputs: Vec<String> = Vec::new();

    loop {
        let input = match prompt_input(&mut stdin, &mut stdout) {
            Ok(s) => s,
            Err(e) => {
                writeln!(stderr, "Error: {}", e)?;
                continue;
            }
        };
        user_inputs.push(input);

        let eval_result = libfjord::eval(&user_inputs.last().unwrap(), &states.last().unwrap());

        match eval_result {
            Ok((s, output)) => {
                states.push(s);
                dbg!(output);
            }
            Err(e) => {
                dbg!(e);
            }
        }

        dbg!(&states);
    }
}

fn prompt_input(stdin: &mut io::Stdin, stdout: &mut io::Stdout) -> io::Result<String> {
    write!(stdout, "→ ")?;
    stdout.flush()?;

    let mut s = String::new();
    stdin.read_line(&mut s)?;

    Ok(s)
}
