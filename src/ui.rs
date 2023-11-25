use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::{
    app::{App, MenuKind},
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
    if app.menu {
        render_menu(app, f);
    } else {
        render_game(app, f);
    }
}

const MENU_HARD: &str = "HARD";
const MENU_NORMAL: &str = "NORMAL";
const MENU_EASY: &str = "EASY";
const MENU_LARGE: &str = "LARGE";
const MENU_SMALL: &str = "SMALL";

fn render_menu(app: &mut App, f: &mut Frame) {
    let frame_size = f.size();
    let size = Rect {
        width: u16::min(30, frame_size.width),
        height: u16::min(12, frame_size.height),
        ..frame_size
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .padding(Padding::zero())
        .title("Menu");
    let content_size = block.inner(size);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_size);

    let map_size_rect = chunks[0];
    let game_level_rect = chunks[1];

    // border setting
    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let selected_style = Style::default().fg(Color::Black);
    let unselected_style = Style::default().fg(Color::Gray);

    let mut map_sizes = vec![
        Line::from(MENU_LARGE),
        Line::from(""),
        Line::from(MENU_NORMAL),
        Line::from(""),
        Line::from(MENU_SMALL),
    ];
    map_sizes[app.menu_map_size as usize * 2]
        .patch_style(Style::default().bg(Color::Black).fg(Color::White));
    let paragraph = Paragraph::new(map_sizes.clone())
        .style(Style::default().bg(Color::White))
        .block(
            create_block("Map Size").title_style(if let MenuKind::MapSize = app.menu_focus {
                selected_style
            } else {
                unselected_style
            }),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, map_size_rect);

    let mut game_levels = vec![
        Line::from(MENU_HARD),
        Line::from(""),
        Line::from(MENU_NORMAL),
        Line::from(""),
        Line::from(MENU_EASY),
    ];
    game_levels[app.menu_game_level as usize * 2]
        .patch_style(Style::default().bg(Color::Black).fg(Color::White));
    let paragraph = Paragraph::new(game_levels.clone())
        .style(Style::default().bg(Color::White))
        .block(create_block("Game Level").title_style(
            if let MenuKind::GameLevel = app.menu_focus {
                selected_style
            } else {
                unselected_style
            },
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, game_level_rect);
}

fn render_game(app: &mut App, f: &mut Frame) {
    let (map_width, map_height) = app.map_size;
    let frame_size = f.size();
    let size = Rect {
        width: u16::min((map_width * 2) + 2, frame_size.width),
        height: u16::min(map_height + 2, frame_size.height),
        ..frame_size
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .padding(Padding::zero())
        .title("Game");
    f.render_widget(block, size);

    let (map_ui_x, map_ui_y) = (size.x + 1, size.y + 1);
    let (curr_x, curr_y) = app.curr_pos;

    let buf = f.buffer_mut();
    let mine_map = &app.mine_map;

    for y in 0..map_height {
        for x in 0..map_width {
            let (symbol, mut style) = mine_map[y as usize][x as usize].symbol_n_style();

            if y == curr_y && x == curr_x {
                let temp = style.bg;
                style.bg = style.fg;
                style.fg = temp;
            }

            let (ui_x, ui_y) = (map_ui_x + x * 2, map_ui_y + y);
            if ui_x < size.x + size.width - 1 && ui_y < size.y + size.height - 1 {
                buf.get_mut(ui_x, ui_y).set_symbol(symbol).set_style(style);
            }
        }
    }

    if app.over {
        render_over(app, f);
        return;
    }
}

fn render_over(app: &mut App, f: &mut Frame) {
    let (message, fg_color, bg_color) = if app.empty_cnt == 0 {
        (" YOU WIN! ", Color::Yellow, Color::Black)
    } else {
        (" GAME OVER! ", Color::White, Color::Black)
    };
    let len_msg = message.len() as u16;
    let (map_w, map_h) = app.map_size;
    let (mid_x, mid_y) = (1 + map_w / 2, 1 + map_h / 2);
    let (over_x, over_y) = ((mid_x - len_msg / 2 + 1) * 2, mid_y - 1);
    let chunk = {
        let base_rect = Rect::default();
        Rect {
            x: over_x,
            y: over_y,
            width: len_msg + 2,
            height: 3,
            ..base_rect
        }
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(fg_color).bg(bg_color));
    f.render_widget(block, chunk);

    let buf = f.buffer_mut();
    for (i, ch) in message.chars().enumerate() {
        let (x, y) = (chunk.x + i as u16 + 1, chunk.y + 1);
        let s = ch.to_string();
        buf.get_mut(x, y)
            .set_symbol(s.as_str())
            .set_fg(fg_color)
            .set_bg(bg_color);
    }
    for y in (chunk.y)..(chunk.y + chunk.height) {
        let x = chunk.x - 1;
        buf.get_mut(x, y)
            .set_symbol(" ")
            .set_bg(bg_color)
            .set_fg(fg_color);
        let x = chunk.x + chunk.width;
        buf.get_mut(x, y)
            .set_symbol(" ")
            .set_bg(bg_color)
            .set_fg(fg_color);
    }
}
