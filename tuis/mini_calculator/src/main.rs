mod events;
mod ui;

use crate::{events::do_event, ui::ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::VecDeque,
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use math_parse::MathParse;
//////////////////////////////////////////////////////
struct EvalRes {
    hex: String,
    dec: String,
    oct: String,
    bin: String,
}

pub struct App<'a> {
    tabs: Vec<&'a str>,
    tab_idx: usize,
    expr_idx: usize,
    results: VecDeque<(String, f64)>,
    save_expression: String,
    expression: String,
    error: bool,
    last_result: EvalRes,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            tabs: vec!["basic", "program"],
            tab_idx: 0,
            expr_idx: 0,
            results: VecDeque::new(),
            save_expression: String::new(),
            expression: String::new(),
            error: false,
            last_result: EvalRes {
                hex: String::new(),
                dec: String::new(),
                oct: String::new(),
                bin: String::new(),
            },
        }
    }
}

#[derive(PartialEq)]
pub enum AppEvent {
    Nothing,
    Expression,
    Results,
    ResultsExpression,
    Enter,
    Tab,
    Error,
    Quit,
}

pub struct Heights {
    tab: u16,
    expression: u16,
    arithmetic: u16,
}

pub static HEIGHTS: Heights = Heights {
    tab: 3,
    expression: 5,
    arithmetic: 6,
};

pub enum TabKind {
    Basic = 0,
    Programming = 1,
}
/////////////////////////////////////////////////////
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(100);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        if app.tab_idx == 0 {
            terminal.draw(|f| ui(f, &app))?;
        } else {
            terminal.draw(|f| ui(f, &app))?;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if do_event(&mut app, &key) == AppEvent::Quit {
                    return Ok(());
                }
            }

            if app.expression.is_empty() {
                app.last_result.hex.clear();
                app.last_result.dec.clear();
                app.last_result.oct.clear();
                app.last_result.bin.clear();
            } else {
                match MathParse::parse(app.expression.as_str()) {
                    Ok(expr) => {
                        let tab_idx = app.tab_idx;
                        if app.tabs[tab_idx] == "program" {
                            match expr.solve_int(None) {
                                Ok(val) => {
                                    app.last_result.hex = format!("0x{:X}", val as i64);
                                    app.last_result.dec = format!("{}", val);
                                    app.last_result.oct = format!("0o{:o}", val as i64);
                                    app.last_result.bin = format!("0b{:b}", val as i64);
                                }
                                Err(_) => {}
                            }
                        }
                    } 
                    Err(_) => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
