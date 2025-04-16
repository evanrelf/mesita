use ratatui::{
    crossterm::{
        self,
        event::{self, Event},
    },
    text::Text,
};
use std::io;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
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
    ratatui::restore();
    Ok(())
}
