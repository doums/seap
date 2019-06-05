use seap::cli_parser::{self, Flag};
use std::env;
use std::io;
use std::process;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

const OPTIONS: [Flag; 2] = [
    Flag("help", 'h', "help", false),
    Flag("verbose", 'v', "verbose", false),
];

fn main() -> Result<(), io::Error> {
    let tokens = cli_parser::tokenize(env::args(), &OPTIONS).unwrap_or_else(|err| {
        eprintln!("parser error: {}", err);
        process::exit(1);
    });
    dbg!(&tokens);
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // terminal.draw(|mut f| {
    // let size = f.size();
    // Block::default()
    // .title("Block")
    // .borders(Borders::ALL)
    // .render(&mut f, size);
    // })
    Ok(())
}
