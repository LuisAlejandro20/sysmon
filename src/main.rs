use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io, time::Duration};
use sysinfo::System;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sys = System::new_all();

    let res = run_app(&mut terminal, &mut sys);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app(
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

fn ui(f: &mut ratatui::Frame, sys: &System) {
    // Split the screen vertically into two chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(f.area());

    // Calculate Global CPU Usage
    let cpu_usage = sys.global_cpu_usage();
    let cpu_text = format!("Total CPU Usage: {:.2}%", cpu_usage);

    let cpu_block = Paragraph::new(cpu_text)
        .block(Block::default().title(" CPU Info ").borders(Borders::ALL));
    
    f.render_widget(cpu_block, chunks[0]);

    // Calculate Memory Usage
    let used_mem = sys.used_memory() / 1024 / 1024;
    let total_mem = sys.total_memory() / 1024 / 1024;
    let mem_text = format!("Memory Usage: {} MB / {} MB", used_mem, total_mem);

    let mem_block = Paragraph::new(mem_text)
        .block(Block::default().title(" Memory Info ").borders(Borders::ALL));
    
    f.render_widget(mem_block, chunks[1]);
}