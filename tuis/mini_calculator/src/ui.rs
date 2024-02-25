use crate::{App, TabKind, HEIGHTS};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default().style(Style::default());
    f.render_widget(block, size);

    // layouts
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(60), Constraint::Length(20)])
        .split(size);
    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEIGHTS.tab),
            Constraint::Length(size.height - HEIGHTS.tab),
        ])
        .split(main_chunks[0]);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            if app.tab_idx == TabKind::Basic as usize {
                vec![
                    Constraint::Length(sub_chunks[1].height - HEIGHTS.expression),
                    Constraint::Length(HEIGHTS.expression),
                ]
            } else {
                // programming
                vec![
                    Constraint::Length(
                        sub_chunks[1].height - (HEIGHTS.expression + HEIGHTS.arithmetic),
                    ),
                    Constraint::Length(HEIGHTS.expression),
                    Constraint::Length(HEIGHTS.arithmetic),
                ]
            }
        )
        .split(sub_chunks[1]);

    // setting for Spans to render
    //     for Tabs window
    let mut titles = vec![];
    for title in &app.tabs {
        titles.push(Spans::from(*title));
    }

    //     for Explanation window
    let explain_text = vec![
        Spans::from( "[Number]"),
        Spans::from( "--------"),
        Spans::from( " hex: 0x[0-9a-zA-Z]+"),
        Spans::from( " dec: [0-9]+"),
        Spans::from( ""),
        Spans::from( "[Priority]"),
        Spans::from( "----------"),
        Spans::from( " 1. unary +, unary -, unary !(not)"),
        Spans::from(r" 2. ×, /, %, //"),
        Spans::from( " 3. binary +, binary -"),
        Spans::from( " 4. <<, >> (bit shifting)"),
        Spans::from( " 5. &(and)"),
        Spans::from( " 6. ^(xor)"),
        Spans::from( " 7. |(or)"),
        Spans::from( ""),
        Spans::from( "[Keys]"),
        Spans::from( "------"),
        Spans::from( " 1. ENTER       : evaluate expression"),
        Spans::from( " 2. TAB/BACKTAB : change mode"),
        Spans::from( " 3. CTRL+c      : quit this program"),
        Spans::from( " 4. CTRL+l      : clear expression"),
        Spans::from( " 5. CTRL+f      : get the latest expression"),
        Spans::from( " 6. CTRL+q      : clear all results"),
        Spans::from( " 7. CTRL+p      : pop the latest result"),
        Spans::from( " 8. CTRL+d      : delete the latest result"),
    ];

    //     for Expression window
    let mut expression_text = vec![Spans::from(vec![
        Span::from(app.expression.as_ref()),
        Span::styled("_", Style::default().bg(Color::Gray).fg(Color::Gray)),
    ])];
    if app.error {
        expression_text.push(Spans::from("Wrong expression"));
    }

    let arithmetic_text: Vec<tui::text::Spans> = if app.tab_idx == TabKind::Programming as usize {
        vec![
            Spans::from(vec![
                Span::from("HEX: "),
                Span::from(app.last_result.hex.as_str()),
            ]),
            Spans::from(vec![
                Span::from("DEC: "),
                Span::from(app.last_result.dec.as_str()),
            ]),
            Spans::from(vec![
                Span::from("OCT: "),
                Span::from(app.last_result.oct.as_str()),
            ]),
            Spans::from(vec![
                Span::from("BIN: "),
                Span::from(app.last_result.bin.as_str()),
            ]),
        ]
    } else {
        vec![]
    };

    //     for Results window
    let mut results_text = vec![];
    let start_idx = if app.results.len() > (chunks[0].height - 2) as usize {
        app.results.len() - (chunks[0].height - 2) as usize
    } else {
        for _ in 0..(chunks[0].height - 2) as usize - app.results.len() {
            results_text.push(Spans::from("."));
        }
        0
    };
    let res_cnt = app.results.len();
    for i in start_idx..res_cnt {
        let result = &app.results[i];
        let s = format!("{} = {}", result.0, result.1);
        results_text.push(Spans::from(s));
    }

    // border setting
    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    // render widgets
    let paragraph = Paragraph::new(explain_text.clone())
        .style(Style::default())
        .block(create_block("Explanation"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, main_chunks[1]);

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tab_idx)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Gray),
        );
    f.render_widget(tabs, sub_chunks[0]);

    let paragraph = Paragraph::new(results_text.clone())
        .style(Style::default())
        .block(create_block("Results"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(expression_text.clone())
        .style(Style::default())
        .block(create_block("Expression"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    if app.tab_idx == TabKind::Programming as usize {
        let paragraph = Paragraph::new(arithmetic_text.clone())
            .style(Style::default())
            .block(create_block(""))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[2]);
    }
}
