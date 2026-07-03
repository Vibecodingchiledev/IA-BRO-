//! UI rendering for IA,BRO! TUI

use crate::app::{App, FilterType, InputMode};
use crate::model::Status;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    draw_header(f, app, chunks[0]);

    // Task list
    draw_task_list(f, app, chunks[1]);

    // Footer
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = Paragraph::new(
        Line::from(vec![
            Span::styled("IA,BRO! ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("TODO Manager"),
        ])
    )
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray)))
    .alignment(Alignment::Left);

    f.render_widget(title, area);
}

fn draw_task_list(f: &mut Frame, app: &App, area: Rect) {
    let tasks = app.filtered_tasks();

    let items: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let content = if task.status == Status::Completed {
                format!("{} {} ", task.status.symbol(), task.title)
            } else {
                let overdue = if task.is_overdue() { " ⚠" } else { "" };
                format!("{} {} [{}]{}", task.status.symbol(), task.priority.symbol(), task.priority.display(), overdue)
            };

            let style = if idx == app.selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if task.status == Status::Completed {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let task_list = List::new(items)
        .block(
            Block::default()
                .title(format!(
                    " Tasks ({}) - Filter: {:?} ",
                    tasks.len(),
                    match app.filter {
                        FilterType::All => "All",
                        FilterType::Pending => "Pending",
                        FilterType::InProgress => "In Progress",
                        FilterType::Completed => "Completed",
                        FilterType::HighPriority => "High Priority",
                    }
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(task_list, area);

    // Draw task details if selected
    if let Some(task) = app.selected_task() {
        let task_info = vec![
            format!("ID: {}", task.id),
            format!("Status: {}", task.status.display()),
            format!("Priority: {}", task.priority.display()),
            if let Some(desc) = &task.description {
                format!("Description: {}", desc)
            } else {
                "Description: -".to_string()
            },
        ];

        let details = Paragraph::new(task_info.join("\n"))
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().title(" Details ").borders(Borders::LEFT));

        let detail_area = Rect {
            x: area.x + area.width.saturating_sub(30),
            y: area.y,
            width: 30,
            height: 6,
        };

        f.render_widget(details, detail_area);
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let instructions = match app.input_mode {
        InputMode::Normal => vec![
            Span::raw("n: New | "),
            Span::raw("Enter: Toggle | "),
            Span::raw("d: Delete | "),
            Span::raw("f: Filter | "),
            Span::raw("c: Clear | "),
            Span::raw("q: Quit"),
        ],
        InputMode::Creating | InputMode::Editing => vec![
            Span::styled("ESC", Style::default().fg(Color::Red)),
            Span::raw(": Cancel | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": Confirm"),
        ],
    };

    let footer = Paragraph::new(Line::from(instructions))
        .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::DarkGray)))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}
