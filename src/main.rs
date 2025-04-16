mod terminal;

use clap::Parser as _;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind};
use ratatui::{
    layout::Constraint,
    style::Stylize as _,
    widgets::{Cell, Row, Table, TableState},
};
use std::{
    cmp::max,
    collections::VecDeque,
    io::{self, Read},
    path::PathBuf,
};

#[derive(clap::Parser)]
struct Args {
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut builder = csv::ReaderBuilder::new();
    builder.has_headers(false);
    let (table_columns, mut rows) = if let Some(path) = args.path {
        csv_to_rows(builder.from_path(path)?)?
    } else {
        csv_to_rows(builder.from_reader(io::stdin()))?
    };
    let header = rows.pop_front().unwrap();
    let constraints = vec![Constraint::Fill(1); table_columns];
    let table = Table::default()
        .header(header)
        .rows(rows)
        .widths(constraints);
    let mut table_state = TableState::default();
    let mut terminal = terminal::enter()?;
    loop {
        terminal.draw(|frame| {
            frame.render_stateful_widget(&table, frame.area(), &mut table_state);
        })?;
        match event::read()? {
            Event::Key(key_event) => match (key_event.modifiers, key_event.code) {
                (m, KeyCode::Up) if m == KeyModifiers::NONE => {
                    *table_state.offset_mut() = table_state.offset().saturating_sub(1)
                }
                (m, KeyCode::Down) if m == KeyModifiers::NONE => *table_state.offset_mut() += 1,
                (m, KeyCode::Char('c')) if m == KeyModifiers::CONTROL => break,
                (m, KeyCode::Char('q')) if m == KeyModifiers::NONE => break,
                _ => {}
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => {
                    *table_state.offset_mut() = table_state.offset().saturating_sub(3)
                }
                MouseEventKind::ScrollDown => *table_state.offset_mut() += 3,
                _ => {}
            },
            _ => {}
        }
    }
    terminal::exit()?;
    Ok(())
}

fn csv_to_rows(
    mut reader: csv::Reader<impl Read>,
) -> anyhow::Result<(usize, VecDeque<Row<'static>>)> {
    let mut rows = VecDeque::new();
    let mut table_columns = 0;
    for (i, result) in reader.records().enumerate() {
        let record = result?;
        let mut cells = Vec::new();
        let mut row_columns = 0;
        for field in &record {
            let cell = Cell::from(String::from(field));
            cells.push(cell);
            row_columns += 1;
        }
        table_columns = max(table_columns, row_columns);
        let row = if i == 0 {
            Row::new(cells).reversed()
        } else if i % 2 == 0 {
            Row::new(cells).on_white()
        } else {
            Row::new(cells)
        };
        rows.push_back(row);
    }
    Ok((table_columns, rows))
}
