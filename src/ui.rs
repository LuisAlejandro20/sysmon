use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, List, ListItem},
};
use sysinfo::System;

use crate::format::*;

pub fn ui(f: &mut ratatui::Frame, sys: &System) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40)
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

    let used_str = format_memory(sys.used_memory());
    let total_str = format_memory(sys.total_memory());

    let mem_label = format!("{} / {}", used_str, total_str);

    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(mem_percent)
        .label(mem_label);

    f.render_widget(mem_gauge, chunks[1]);

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    let mut process_items = Vec::new();

    for (i,p) in processes.iter().take(5).enumerate() {
        let memory_str = format_memory(p.memory());

        let name = p.name().to_string_lossy();
        let item_str = format!("{}- {} - {}", i  + 1, name, memory_str);
        process_items.push(ListItem::new(item_str));
    }

    let process_list = List::new(process_items)
        .block(Block::default().title("Top Process (Memory)").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(process_list, chunks[2])
}