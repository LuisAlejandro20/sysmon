use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
};
use sysinfo::System;

pub fn ui(f: &mut ratatui::Frame, sys: &System) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(f.area());

    let cpu_usage = sys.global_cpu_usage();
    let cpu_text = format!("Total CPU usage: {:.2}%", cpu_usage);

    let cpu_block = Paragraph::new(cpu_text)
            .block(Block::default().title(" CPU Info ").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green));

    f.render_widget(cpu_block, chunks[0]);

    let used_mem = sys.used_memory() as f64;
    let total_mem = sys.total_memory() as f64;

    let mem_percent = if total_mem > 0.0 {
        (used_mem / total_mem * 100.00) as u16
    } else {
        0
    };

    let mem_label = format!("{} MB / {} MB", sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024);

    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(mem_percent)
        .label(mem_label);

    f.render_widget(mem_gauge, chunks[1]);
}