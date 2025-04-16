mod terminal;

use clap::Parser as _;
use ratatui::{
    crossterm::{
        self,
        event::{self, Event},
    },
    text::Text,
};
use std::io;

#[derive(clap::Parser)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    let mut terminal = terminal::enter()?;
    crossterm::execute!(io::stdout(), event::EnableMouseCapture)?;
    loop {
        terminal.draw(|frame| {
            let text = Text::raw("Hello World!");
            frame.render_widget(text, frame.area());
        })?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    terminal::exit()?;
    Ok(())
}
