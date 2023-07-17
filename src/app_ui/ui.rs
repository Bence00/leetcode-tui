use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Padding, Paragraph},
    Frame,
};

use super::{app::App, helpers::question::Stats};

/// Renders the user interface widgets.
pub fn render<'a, B: Backend>(app: &'a mut App, f: &mut Frame<'_, B>) {
    // Create two chunks with equal horizontal screen space
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main block with round corners")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    // Iterate through all elements in the `items` app and append some debug text to it.
    for (i, w) in app.widgets.iter_mut().enumerate() {
        let is_widget_active = app.widget_switcher as usize == i;
        let mut border_style = Style::default();
        if is_widget_active {
            border_style = border_style.fg(Color::Cyan);
        }
        match w {
            super::app::Widget::TopicTagList(ttl) => {
                let items: Vec<ListItem> = ttl
                    .items
                    .iter()
                    .map(|tt_model| {
                        if let Some(name) = &tt_model.name {
                            let lines = vec![Line::from(name.as_str())];
                            ListItem::new(lines)
                        } else {
                            ListItem::new(vec![Line::from("")])
                        }
                    })
                    .collect();

                // Create a List from all list items and highlight the currently selected one
                let items = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Tags")
                            .border_style(border_style),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                // We can now render the item list
                f.render_stateful_widget(items, left_chunks[0], &mut ttl.state);
            }
            super::app::Widget::QuestionList(ql) => {
                let questions: Vec<ListItem> = ql
                    .items
                    .iter()
                    .map(|question| {
                        let mut lines = vec![];
                        if let Some(title) = &question.title {
                            lines.push(Line::from(format!(
                                "{:0>4}: {}",
                                question.frontend_question_id, title,
                            )));
                        }
                        ListItem::new(lines)
                    })
                    .collect();

                let items = List::new(questions)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Questions")
                            .border_style(border_style),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");
                f.render_stateful_widget(items, right_chunk[0], &mut ql.state);

                let create_block = |title| {
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Gray))
                        .title(Span::styled(
                            title,
                            Style::default().add_modifier(Modifier::BOLD),
                        ))
                };

                let block = create_block("Stats");
                let inner_area = block.inner(left_chunks[1]);

                f.render_widget(block, left_chunks[1]);

                let stats = Stats { qm: &ql.items };

                let guage = |title: &'a str, val: usize, total: usize| {
                    let block_title = format!("{}: {}/{}", title, val, total);
                    let label = Span::styled(
                        format!(
                            "{:.2}%",
                            if total != 0 {
                                (val as f32 / total as f32) * 100 as f32
                            } else {
                                0 as f32
                            }
                        ),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
                    );

                    Gauge::default()
                        .block(Block::default().title(block_title).borders(Borders::ALL))
                        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
                        .percent(val as u16)
                        .label(label)
                };

                let horizontal_partition = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(inner_area);

                let left_partition = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(horizontal_partition[0]);

                let right_partition = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ])
                    .split(horizontal_partition[1]);

                f.render_widget(
                    guage(
                        "Attempted",
                        stats.get_total_question() - stats.get_not_attempted(),
                        stats.get_total_question(),
                    ),
                    left_partition[0],
                );
                f.render_widget(
                    guage("Accepted", stats.get_accepted(), stats.get_total_question()),
                    left_partition[1],
                );

                f.render_widget(
                    guage("Easy", stats.get_easy_accepted(), stats.get_easy_count()),
                    right_partition[0],
                );

                f.render_widget(
                    guage(
                        "Medium",
                        stats.get_medium_accepted(),
                        stats.get_medium_count(),
                    ),
                    right_partition[1],
                );

                f.render_widget(
                    guage("Hard", stats.get_hard_accepted(), stats.get_hard_count()),
                    right_partition[2],
                );
            }
        }
    }
}
