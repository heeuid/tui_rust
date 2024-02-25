use crate::{App, AppEvent, TabKind};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use math_parse::MathParse;
//////////////////////////////////////////////////////
pub fn do_event(app: &mut App, key_event: &KeyEvent) -> AppEvent {
    let app_event = match key_event.modifiers {
        KeyModifiers::CONTROL => do_event_key_code_with_control(app, key_event.code),
        KeyModifiers::SHIFT => do_event_key_code_with_shift(app, key_event.code),
        KeyModifiers::NONE => do_event_key_code(app, key_event.code),
        _ => AppEvent::Nothing,
    };

    if app.error && app_event != AppEvent::Nothing && app_event != AppEvent::Error {
        app.error = false;
    }

    app_event
}

fn do_event_key_code_with_shift(app: &mut App, key_code: KeyCode) -> AppEvent {
    do_event_key_code(app, key_code)
}

fn do_event_key_code_with_control(app: &mut App, key_code: KeyCode) -> AppEvent {
    match key_code {
        KeyCode::Char(code) => match code {
            'c' => AppEvent::Quit,
            'l' => {
                app.expression.clear();
                AppEvent::Expression
            }
            'b' => {
                if let Some((s, _)) = app.results.back() {
                    app.expression = s.clone();
                    AppEvent::Expression
                } else {
                    AppEvent::Nothing
                }
            }
            'q' => {
                app.results.clear();
                AppEvent::Results
            }
            'd' => {
                app.results.pop_back();
                AppEvent::Results
            }
            'p' => {
                if let Some((expr, _)) = app.results.pop_back() {
                    app.expression = expr;
                }
                AppEvent::ResultsExpression
            }
            'f' => {
                if let Some((expr, _)) = app.results.back() {
                    app.expression = expr.clone();
                }
                AppEvent::ResultsExpression
            }
            _ => AppEvent::Nothing,
        },
        _ => AppEvent::Nothing,
    }
}

fn do_event_key_code(app: &mut App, key_code: KeyCode) -> AppEvent {
    match key_code {
        KeyCode::Char(code) => {
            if app.tab_idx == TabKind::Programming as usize {
                if code != '.' {
                    app.expression.push(code);
                } else {
                    return AppEvent::Error;
                }
            } else {
                app.expression.push(code);
            }
            AppEvent::Expression
        }
        KeyCode::Enter => {
            // evaluate expression and save the result to app.results
            if !app.expression.is_empty() {
                match MathParse::parse(app.expression.as_str()) {
                    Ok(expr) => {
                        let tab_idx = app.tab_idx;
                        if app.tabs[tab_idx] == "basic" {
                            match expr.solve_float(None) {
                                Ok(value) => {
                                    app.results.push_back((app.expression.clone(), value));
                                    app.expression.clear();
                                    AppEvent::Enter
                                }
                                Err(_) => {
                                    app.error = true;
                                    AppEvent::Error
                                }
                            }
                        } else if app.tabs[tab_idx] == "program" {
                            match expr.solve_int(None) {
                                Ok(value) => {
                                    app.results.push_back((app.expression.clone(), value as f64));
                                    app.expression.clear();
                                    AppEvent::Enter
                                }
                                Err(_) => {
                                    app.error = true;
                                    AppEvent::Error
                                }
                            }
                        } else {
                            app.error = true;
                            AppEvent::Error
                        }
                    } 
                    Err(_) => {
                        app.error = true;
                        AppEvent::Error
                    }
                }
            } else {
                AppEvent::Nothing
            }
        }
        KeyCode::Backspace => {
            app.expression.pop();
            AppEvent::Expression
        }
        KeyCode::Tab => {
            let tab_cnt = app.tabs.len();
            let tab_idx = app.tab_idx;
            app.tab_idx = (tab_idx + 1) % tab_cnt;
            AppEvent::Tab
        }
        KeyCode::BackTab => {
            let tab_cnt = app.tabs.len();
            let tab_idx = app.tab_idx;
            app.tab_idx = (tab_idx + tab_cnt - 1) % tab_cnt;
            AppEvent::Tab
        }
        KeyCode::Up => {
            let result_len = app.results.len();
            if app.expr_idx < result_len {
                if app.expr_idx == 0 {
                    app.save_expression.clear();
                    app.save_expression = std::mem::take(&mut app.expression);
                }
                app.expr_idx += 1;
                app.expression.clear();
                app.expression
                    .push_str(app.results[result_len - app.expr_idx].0.as_str());
            }
            AppEvent::Expression
        }
        KeyCode::Down => {
            let result_len = app.results.len();
            if app.expr_idx > 0 {
                app.expr_idx -= 1;
                app.expression.clear();

                if app.expr_idx == 0 {
                    app.expression.push_str(app.save_expression.as_str());
                } else {
                    app.expression
                        .push_str(app.results[result_len - app.expr_idx].0.as_str());
                };
            }
            AppEvent::Expression
        }
        _ => AppEvent::Nothing,
    }
}
