use crossterm::{
    event::{self, Event, KeyCode},
};
use std::{
    error::Error, 
    io, time::Duration
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use sysinfo::System;

use crate::ui::ui;

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    sys: &mut System,
) -> Result <(), Box<dyn Error>> {
    loop {
        sys.refresh_all();

        terminal.draw(|f| ui(f, sys))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(())
                }
            }
        }
    }
}

