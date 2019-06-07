use seap::cli_parser::Parser;
use std::env;
use std::io;
use std::process;
use termion::clear;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let tokens = Parser::new(env::args())
        .help()
        .version()
        .tokenize()
        .unwrap_or_else(|err| {
            eprintln!("parser error: {}", err);
            process::exit(1);
        });
    dbg!(&tokens);
    println!("{}", env!("CARGO_PKG_VERSION"));

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let size = f.size();
        Block::default()
            .title("seap")
            .borders(Borders::ALL)
            .render(&mut f, size);
    });
    Ok(())
}
