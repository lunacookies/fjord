#![warn(missing_debug_implementations, rust_2018_idioms)]

mod themes;

use std::{
    convert::TryInto,
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
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

    buffer.initialize_terminal(&mut stdout)?;
    buffer.redraw(&mut stdout)?;

    loop {
        if let event::Event::Key(k) = event::read()? {
            match (k.code, k.modifiers) {
                (c, KeyModifiers::NONE) => match c {
                    KeyCode::Up => buffer.move_cursor(Direction::Up),
                    KeyCode::Down => buffer.move_cursor(Direction::Down),
                    KeyCode::Left => buffer.move_cursor(Direction::Left),
                    KeyCode::Right => buffer.move_cursor(Direction::Right),
                    KeyCode::Backspace => buffer.backspace(),
                    KeyCode::Enter => buffer.insert_newline(),
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
    rows: Vec<String>,
    top_line: usize,
    left_col: usize,
    line_nr: usize,
    col_nr: usize,
    window_lines: usize,
    window_cols: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Buffer {
    // Hardcode the theme to default to Gruvbox.
    const THEME: themes::Gruvbox = themes::Gruvbox;

    fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        use crossterm::terminal;

        let (cols, lines) = terminal::size()?;

        // Open new files at the top, with the cursor on the first column of the first line.
        Ok(Self {
            rows: std::fs::read_to_string(path)?
                .lines()
                .map(ToString::to_string)
                .collect(),
            top_line: 0,
            left_col: 0,
            line_nr: 0,
            col_nr: 0,
            window_lines: lines.try_into()?,
            window_cols: cols.try_into()?,
        })
    }

    fn current_line_len(&self) -> usize {
        self.rows[self.line_nr].len()
    }

    fn is_on_first_line(&self) -> bool {
        self.line_nr == 0
    }

    fn is_on_last_line(&self) -> bool {
        self.line_nr == self.rows.len() - 1
    }

    fn is_on_first_col(&self) -> bool {
        self.col_nr == 0
    }

    fn is_on_last_col(&self) -> bool {
        self.col_nr == self.current_line_len()
    }

    fn scroll_lines(&mut self) {
        if self.line_nr < self.top_line {
            self.top_line = self.line_nr;
        } else if self.line_nr >= self.top_line + self.window_lines {
            self.top_line = self.line_nr - self.window_lines + 1;
        }
    }

    fn scroll_cols(&mut self) {
        if self.col_nr < self.left_col {
            self.left_col = self.col_nr;
        } else if self.col_nr >= self.left_col + self.window_cols {
            self.left_col = self.col_nr - self.window_cols + 1;
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if !self.is_on_first_line() {
                    self.line_nr -= 1;
                    self.snap_cursor_to_eol();
                }
            }

            Direction::Down => {
                if !self.is_on_last_line() {
                    self.line_nr += 1;
                    self.snap_cursor_to_eol();
                }
            }

            Direction::Left => {
                if !self.is_on_first_col() {
                    self.col_nr -= 1;
                } else if !self.is_on_first_line() {
                    self.line_nr -= 1;
                    self.col_nr = self.current_line_len();
                }
            }

            Direction::Right => {
                if !self.is_on_last_col() {
                    self.col_nr += 1;
                } else if !self.is_on_last_line() {
                    self.line_nr += 1;
                    self.col_nr = 0;
                }
            }
        }

        self.scroll_lines();
        self.scroll_cols();
    }

    fn snap_cursor_to_eol(&mut self) {
        let current_line_len = self.current_line_len();

        if self.col_nr >= current_line_len {
            self.col_nr = current_line_len;
        }
    }

    fn insert_char(&mut self, c: char) {
        self.rows[self.line_nr].insert(self.col_nr, c);
        self.move_cursor(Direction::Right);
    }

    fn insert_newline(&mut self) {
        // If we’re on the first or last column then we can simply add a new line. Otherwise, we
        // split the current line at the cursor’s position.
        if self.is_on_first_col() {
            self.rows.insert(self.line_nr, String::new());
        } else if self.is_on_last_col() {
            self.rows.insert(self.line_nr + 1, String::new());
        } else {
            let ending_segment = self.rows[self.line_nr].split_off(self.col_nr);
            self.rows.insert(self.line_nr + 1, ending_segment);
        }

        // Once we insert the new line we snap to the first column, and go down one line so that the
        // cursor is positioned on the new line.
        self.col_nr = 0;
        self.move_cursor(Direction::Down);
    }

    fn backspace(&mut self) {
        let was_on_first_col = self.is_on_first_col();

        // TODO: it would be cleaner if this was moved to the end of this function.
        //
        // Moving left now places us either before the character we want to delete (this corrects
        // for zero indexing) if we’re not on the first column, or brings us to the end of the line
        // before the one we’re on if we’re on the first column.
        self.move_cursor(Direction::Left);

        // Join the line we’re on and the line below it if we were on the first column before
        // moving, or just delete the character the cursor is on otherwise.
        if was_on_first_col {
            let joined_line = self.rows[self.line_nr + 1].clone();
            self.rows[self.line_nr].push_str(&joined_line);

            self.rows.remove(self.line_nr + 1);
        } else {
            self.rows[self.line_nr].remove(self.col_nr);
        }
    }

    fn update_window_dimens(&mut self) -> anyhow::Result<()> {
        use crossterm::terminal;

        let (cols, lines) = terminal::size()?;

        self.window_lines = lines.try_into()?;
        self.window_cols = cols.try_into()?;

        Ok(())
    }

    // All this function does is cover the the terminal in the colors of the theme.
    fn initialize_terminal(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        use {
            crossterm::{cursor, queue, terminal},
            syntax::Theme,
        };

        let default_style: ansi_term::Style = Self::THEME.default_style().into();

        // Move the cursor to the top-left, change the colours to be that of the theme, and finally
        // clear each line of the terminal.
        queue!(stdout, cursor::MoveTo(0, 0))?;
        stdout.write_all(default_style.prefix().to_string().as_bytes())?;

        for _ in 0..self.window_lines {
            queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
            stdout.write_all(b"\r\n")?;
        }

        queue!(stdout, cursor::MoveTo(0, 0))?;

        Ok(())
    }

    fn redraw(&mut self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        use {
            crossterm::{cursor, execute, queue, terminal},
            itertools::Itertools,
        };

        execute!(
            stdout,
            cursor::Hide, // Hiding the cursor makes redrawing less distracting.
            cursor::MoveTo(0, 0),
        )?;

        // Update window dimensions and syntax highlighting each redraw.
        self.update_window_dimens()?;
        let rendered = render(&self.rows.join("\n"), Self::THEME);

        let displayed_portion = rendered
            .lines()
            .skip(self.top_line) // Start drawing the file at the line at the top of the screen.
            .take(self.window_lines) // Only draw enough rows to fill the terminal.
            .map(|line| {
                let line = {
                    // Start drawing lines at the leftmost column number that is currently being
                    // displayed. If this were to run past the end of the line, just use an empty
                    // one.
                    if line.len() > self.left_col {
                        let line = &line[self.left_col..];

                        // Truncate lines if they don’t fit on the screen.
                        if line.len() > self.window_cols {
                            &line[..self.window_cols]
                        } else {
                            &line
                        }
                    } else {
                        ""
                    }
                };

                // Clear each line before displaying it.
                format!(
                    "{}{}",
                    terminal::Clear(terminal::ClearType::UntilNewLine),
                    line
                )
                .into_bytes()
            })
            // We intersperse line endings here to avoid an empty line at the bototm of the window.
            .intersperse(b"\r\n".to_vec())
            .flatten()
            .collect::<Vec<_>>();

        stdout.write_all(&displayed_portion)?;

        // Move the cursor to its position, and show it again so the user knows where it is.
        queue!(
            stdout,
            cursor::MoveTo(
                (self.col_nr - self.left_col).try_into().unwrap(),
                (self.line_nr - self.top_line).try_into().unwrap()
            ),
            cursor::Show
        )?;
        stdout.flush()?;

        Ok(())
    }
}

// This function highlights the input, renders this with a given theme, adds escape sequences to
// switch between the theme’s colors, and finally collects this into a string.
fn render<T: syntax::Theme>(input: &str, theme: T) -> String {
    let default_style: ansi_term::Style = theme.default_style().into();

    // Hardcode Rust syntax highlighting for now.
    syntax::render(input, syntax_rust::RustHighlighter, theme)
        .into_iter()
        .map(|syntax::StyledSpan { text, style }| {
            let style: ansi_term::Style = style.into();

            format!(
                "{}{}{}",
                // Transition from the theme’s default style to the style of the current span.
                default_style.infix(style),
                // Include the span’s text.
                text,
                // Transition back to the default style.
                style.infix(default_style)
            )
        })
        .collect()
}
