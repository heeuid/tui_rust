use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Movement};

pub fn update(app: &mut App, key_event: KeyEvent) {
    if app.menu {
        update_menu(app, key_event);
        return;
    }

    update_game(app, key_event);
}

pub fn update_menu(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Up | KeyCode::Char('k') => app.menu_move(Movement::Up),
        KeyCode::Down | KeyCode::Char('j') => app.menu_move(Movement::Down),
        KeyCode::Left | KeyCode::Char('h') => app.menu_move(Movement::Left),
        KeyCode::Right | KeyCode::Char('l') => app.menu_move(Movement::Right),
        KeyCode::Enter | KeyCode::Char('c') => {
            let map_size = app.menu_map_size.map_size();
            let bomb_cnt = app.menu_game_level.bomb_cnt(app.menu_map_size);
            app.init_mine_map(map_size, bomb_cnt);
        }
        _ => {}
    }
}

pub fn update_game(app: &mut App, key_event: KeyEvent) {
    if app.over {
        update_over(app, key_event);
        return;
    }

    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            } else if key_event.modifiers == KeyModifiers::NONE {
                app.uncover_tile();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => app.game_move(Movement::Up),
        KeyCode::Down | KeyCode::Char('j') => app.game_move(Movement::Down),
        KeyCode::Left | KeyCode::Char('h') => app.game_move(Movement::Left),
        KeyCode::Right | KeyCode::Char('l') => app.game_move(Movement::Right),
        KeyCode::Enter => app.uncover_tile(),
        KeyCode::Char(' ') | KeyCode::Char('f') => app.change_cover(),
        _ => {}
    };
}

pub fn update_over(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => app.reset(),
        _ => {}
    }
}
