use {
    foreignfjordfunc_derive::ForeignFjordFunc,
    std::{
        convert::{TryFrom, TryInto},
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

        // The editor doesn’t return anything.
        libfjord::eval::OutputExpr::Unit
    }
}

fn run(path: impl AsRef<Path>) -> anyhow::Result<()> {
    use crossterm::{
        event::{self, KeyCode, KeyModifiers},
        queue, terminal,
    };

    // Attempt to load the given file before doing anything else.
    let mut buffer = Buffer::new(path)?;

    let mut stdout = io::stdout();

    queue!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.flush()?;

    buffer.redraw(&mut stdout)?;

    loop {
        if let event::Event::Key(k) = event::read()? {
            match (k.code, k.modifiers) {
                (c, KeyModifiers::NONE) => match c {
                    KeyCode::Left => buffer.move_cursor(-1, 0),
                    KeyCode::Right => buffer.move_cursor(1, 0),
                    KeyCode::Up => buffer.move_cursor(0, -1),
                    KeyCode::Down => buffer.move_cursor(0, 1),
                    KeyCode::Backspace => buffer.backspace(),
                    KeyCode::Char(c) => buffer.insert_char(c),
                    _ => (),
                },
                // Quit on C-q
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                _ => (),
            }
        }

        buffer.redraw(&mut stdout)?;
    }

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
    idx: usize,
}

impl Buffer {
    fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        // Open new files at the top, with the cursor on the first column of the first line.
        Ok(Self {
            file_contents: ropey::Rope::from_reader(std::fs::File::open(path)?)?,
            top_line: 0,
            line_nr: 0,
            col_nr: 0,
            idx: 0,
        })
    }

    fn recalculate_idx(&mut self) {
        self.idx = self
            .file_contents
            .line_to_char(self.line_nr.try_into().unwrap())
            + usize::try_from(self.col_nr).unwrap();
    }

    fn move_cursor(&mut self, x: i32, y: i32) {
        let clamp = |x| if x < 0 { 0 } else { x };

        let col_nr = i32::from(self.col_nr) + x;
        let line_nr = i32::from(self.line_nr) + y;

        // We know the conversion cannot fail, as clamp prevents negative values.
        self.col_nr = clamp(col_nr).try_into().unwrap();
        self.line_nr = clamp(line_nr).try_into().unwrap();

        // We’ve potentially changed the column and line number, so we need to recalculate the
        // character index.
        self.recalculate_idx();
    }

    fn insert_char(&mut self, c: char) {
        let idx = self
            .file_contents
            .line_to_char(self.line_nr.try_into().unwrap())
            + usize::try_from(self.col_nr).unwrap();

        self.file_contents.insert_char(idx, c);

        self.move_cursor(1, 0);
    }

    fn backspace(&mut self) {
        // Remove the character the cursor is at.
        self.file_contents.remove(self.idx - 1..self.idx);

        self.move_cursor(-1, 0);
    }

    fn redraw(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        use {
            crossterm::{cursor, execute, queue, terminal},
            itertools::Itertools,
        };

        execute!(
            stdout,
            cursor::Hide, // Hiding the cursor makes redrawing less distracting.
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All),
        )?;

        let (cols, rows) = terminal::size()?;

        // TODO: Early-return a Result here instead of unwrapping.
        let (cols, rows): (usize, usize) = (cols.try_into().unwrap(), rows.try_into().unwrap());

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
