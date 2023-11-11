use ratatui::{
    //layout::{Alignment, Constraint, Direction, Layout, Rect},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding},
};

use crate::{app::App, tui::Frame};

pub fn render(app: &mut App, f: &mut Frame) {
    if app.menu {
        render_menu(app, f);
    } else {
        render_game(app, f);
    }
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
        .padding(Padding::zero());
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

fn render_menu(_app: &mut App, _f: &mut Frame) {}

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
