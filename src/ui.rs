use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use sysinfo::System;

pub fn ui(f: &mut ratatui::Frame, sys: &System) {
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