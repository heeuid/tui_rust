use rand::prelude::*;
use ratatui::style::{Color, Style};

#[derive(Debug)]
pub enum TileContent {
    Empty(u8),
    Bomb,
}

#[derive(Debug)]
pub enum TileCover {
    Empty,
    QuestionMark,
    FlagMark,
}

impl TileCover {
    pub fn next_cover(&self) -> Self {
        match self {
            Self::Empty => Self::QuestionMark,
            Self::QuestionMark => Self::FlagMark,
            Self::FlagMark => Self::Empty,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    content: TileContent,
    cover: Option<TileCover>,
}

const EMPTY_NUM_COLORS: [Color; 9] = [
    Color::Gray,
    Color::LightBlue,
    Color::LightRed,
    Color::LightGreen,
    Color::LightMagenta,
    Color::LightCyan,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightRed,
];

impl Tile {
    pub fn symbol_n_style<'a>(&self) -> (&'a str, Style) {
        match &self.cover {
            Some(cover) => match cover {
                TileCover::Empty => ("  ", Style::default().bg(Color::DarkGray)),
                TileCover::QuestionMark => {
                    (" ?", Style::default().bg(Color::DarkGray).fg(Color::Yellow))
                }
                TileCover::FlagMark => (" P", Style::default().bg(Color::DarkGray).fg(Color::Red)),
            },
            None => match self.content {
                TileContent::Empty(num) => {
                    let s = match num {
                        1 => " 1",
                        2 => " 2",
                        3 => " 3",
                        4 => " 4",
                        5 => " 5",
                        6 => " 6",
                        7 => " 7",
                        8 => " 8",
                        _ => " .",
                    };
                    (s, Style::default().bg(EMPTY_NUM_COLORS[num as usize]))
                }
                TileContent::Bomb => (" *", Style::default().bg(Color::Gray).fg(Color::Red)),
            },
        }
    }
}

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    pub menu: bool,

    pub map_size: (u16, u16), //(w,h)
    pub bomb_cnt: u16,
    pub curr_pos: (u16, u16), //(x,y)
    pub mine_map: Vec<Vec<Tile>>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn conf_mine_map(mut self, map_size: (u16, u16), bomb_cnt: u16) -> Self {
        self.map_size = map_size;
        self.bomb_cnt = bomb_cnt;
        self.curr_pos = (map_size.0 / 2, map_size.1 / 2);
        self
    }

    pub fn init_mine_map(mut self) -> Self {
        let (width, height) = self.map_size;
        let bomb_cnt = self.bomb_cnt;
        let mut rng = rand::thread_rng();
        let mut positions = vec![];

        self.mine_map.clear();

        for y in 0..height {
            self.mine_map.push(vec![]);

            for x in 0..width {
                let size = self.mine_map.len();

                self.mine_map[size - 1].push(Tile {
                    content: TileContent::Empty(0),
                    cover: None, //Some(TileCover::Empty),
                });

                positions.push((x, y));
            }
        }

        positions.shuffle(&mut rng);

        for i in 0..bomb_cnt {
            let (x, y) = positions[i as usize];

            self.mine_map[y as usize][x as usize].content = TileContent::Bomb;
        }

        let dxdys = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];

        for i in 0..bomb_cnt {
            let (x, y) = positions[i as usize];
            let x = x as i32;
            let y = y as i32;

            for (dx, dy) in dxdys {
                let (new_x, new_y) = (x + dx, y + dy);

                if !(0 <= new_x && new_x < height as i32 && 0 <= new_y && new_y < height as i32) {
                    continue;
                }

                let tile = &mut self.mine_map[new_y as usize][new_x as usize];
                if let TileContent::Empty(num) = tile.content {
                    tile.content = TileContent::Empty(num + 1);
                }
            }
        }

        self
    }

    pub fn move_right(&mut self) {
        let (w, _) = self.map_size;
        let (x, _) = self.curr_pos;
        self.curr_pos.0 = (x + 1) % w;
    }

    pub fn move_left(&mut self) {
        let (w, _) = self.map_size;
        let (x, _) = self.curr_pos;
        self.curr_pos.0 = (x + w - 1) % w;
    }

    pub fn move_up(&mut self) {
        let (_, h) = self.map_size;
        let (_, y) = self.curr_pos;
        self.curr_pos.1 = (y + h - 1) % h;
    }

    pub fn move_down(&mut self) {
        let (_, h) = self.map_size;
        let (_, y) = self.curr_pos;
        self.curr_pos.1 = (y + 1) % h;
    }

    pub fn uncover_tile(&mut self) {
        let (x, y) = self.curr_pos;
        let tile = &mut self.mine_map[y as usize][x as usize];
        tile.cover = None;
    }

    pub fn change_cover(&mut self) {
        let (x, y) = self.curr_pos;
        let tile = &mut self.mine_map[y as usize][x as usize];
        match &tile.cover {
            Some(cover) => {
                tile.cover = Some(cover.next_cover());
            }
            None => {}
        }
    }
}
