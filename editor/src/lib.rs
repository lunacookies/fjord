use {
    foreignfjordfunc_derive::ForeignFjordFunc,
    std::{
        io::{self, Write},
        path::Path,
    },
};

#[derive(Debug, ForeignFjordFunc)]
struct Editor {
    // TODO: Add support for paths to Fjord.
    file: String,
}

impl Editor {
    fn run(self) -> libfjord::eval::OutputExpr {
        if let Err(e) = run(self.file) {
            eprintln!("Error: {}", e);
        }

        // The editor doesn’t return anything
        libfjord::eval::OutputExpr::Unit
    }
}

fn run(path: impl AsRef<Path>) -> anyhow::Result<()> {
    use crossterm::{queue, terminal};

    // Attempt to load the given file before doing anything else.
    let buffer = Buffer::new(path)?;

    let mut stdout = io::stdout();

    queue!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.flush()?;

    buffer.redraw(&mut stdout)?;

    // Sleep for a few seconds so I can check if everything is working as expected. This is only
    // for debugging purposes, and can be removed once proper handling of keystrokes is added.
    std::thread::sleep(std::time::Duration::from_secs(5));

    terminal::disable_raw_mode()?;
    queue!(stdout, terminal::LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}

#[derive(Debug)]
struct Buffer {
    file_contents: ropey::Rope,
    top_line: usize,
    line_nr: u16,
    col_nr: u16,
}

impl Buffer {
    fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        // Open new files at the top, with the cursor on the first column of the first line.
        Ok(Self {
            file_contents: ropey::Rope::from_reader(std::fs::File::open(path)?)?,
            top_line: 0,
            line_nr: 0,
            col_nr: 0,
        })
    }

    fn redraw(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        use {
            crossterm::{cursor, execute, queue, terminal},
            itertools::Itertools,
        };

        // Hiding the cursor makes redrawing less distracting.
        execute!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        let (cols, rows) = terminal::size()?;
        let (cols, rows) = (cols as usize, rows as usize);

        let displayed_portion = self
            .file_contents
            .lines()
            .skip(self.top_line) // Start drawing the file at the line at the top of the screen.
            .take(rows) // Only draw enough rows to fill the terminal.
            .map(|rope_slice| {
                // Truncate lines if they don’t fit on the screen.
                if rope_slice.len_chars() > cols {
                    rope_slice.slice(..cols).bytes().collect::<Vec<_>>()
                } else {
                    rope_slice.bytes().collect::<Vec<_>>()
                }
            })
            .intersperse(b"\r".to_vec())
            .flatten()
            .collect::<Vec<_>>();

        stdout.write_all(&displayed_portion)?;

        // Move the cursor to its position, and show it again so the user knows where it is.
        queue!(
            stdout,
            cursor::MoveTo(self.col_nr, self.line_nr),
            cursor::Show
        )?;
        stdout.flush()?;

        Ok(())
    }
}
