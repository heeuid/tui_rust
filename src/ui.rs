use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{app::App, tui::Frame};

pub fn render(app: &mut App, f: &mut Frame) {
    if !app.menu {
        render_game(app, f);
    } else {
        render_game(app, f);
    }
}

fn render_game(app: &mut App, f: &mut Frame) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .padding(Padding::zero());
    f.render_widget(block, size);

    let (map_width, map_height) = app.map_size;
    let (map_ui_x, map_ui_y) = (size.x + 1, size.y + 1);

    let buf = f.buffer_mut();
    let mine_map = &app.mine_map;

    for y in 0..map_height {
        for x in 0..map_width {
            let (symbol, style) = mine_map[y as usize][x as usize].symbol_n_style();
            let (ui_x, ui_y) = (map_ui_x + x * 2, map_ui_y + y);
            if ui_x < size.x + size.width - 1 && ui_y < size.y + size.height - 1 {
                buf.get_mut(ui_x, ui_y).set_symbol(symbol).set_style(style);
            }
        }
    }
}

fn render_menu(app: &mut App, f: &mut Frame) {}
