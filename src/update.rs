use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

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
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Left | KeyCode::Char('h') => app.move_left(),
        KeyCode::Right | KeyCode::Char('l') => app.move_right(),
        KeyCode::Enter => app.uncover_tile(),
        KeyCode::Char(' ') | KeyCode::Char('f') => app.change_cover(),
        _ => {}
    };
}

pub fn update_over(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Enter => app.reset(),
        _ => {}
    }
}
