use rand::prelude::*;
use tile::{Tile, TileContent, TileCover};

mod tile {
    use ratatui::style::{Color, Style};

    #[derive(Debug, Clone, Copy)]
    pub enum TileContent {
        Empty(u8),
        Bomb,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TileCover {
        Empty,
        QuestionMark,
        FlagMark,
    }

    impl TileCover {
        pub fn next_cover(&self) -> Self {
            match self {
                Self::Empty => Self::FlagMark,
                Self::FlagMark => Self::QuestionMark,
                Self::QuestionMark => Self::Empty,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Tile {
        pub content: TileContent,
        pub cover: Option<TileCover>,
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
                    TileCover::Empty => (
                        "ㅁ",
                        Style::default()
                            .bg(Color::Rgb(180, 180, 180))
                            .fg(Color::White),
                    ),
                    TileCover::QuestionMark => (
                        " ?",
                        Style::default()
                            .bg(Color::Rgb(200, 200, 180))
                            .fg(Color::Yellow),
                    ),
                    TileCover::FlagMark => (
                        " ⚑",
                        Style::default()
                            .bg(Color::Rgb(200, 180, 180))
                            .fg(Color::Red),
                    ),
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
                        (
                            s,
                            Style::default()
                                .bg(EMPTY_NUM_COLORS[num as usize])
                                .fg(Color::Black),
                        )
                    }
                    TileContent::Bomb => (
                        " *",
                        Style::default()
                            .bg(Color::Rgb(250, 200, 200))
                            .fg(Color::Red),
                    ),
                },
            }
        }
    }
}

const DXDY8: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

const DXDY4: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    pub menu: bool,
    pub over: bool,

    pub map_size: (u16, u16), //(w,h)
    pub bomb_cnt: u16,
    pub empty_cnt: u16,
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

    fn init_members(&mut self, map_size: (u16, u16), bomb_cnt: u16) {
        self.map_size = map_size;
        self.bomb_cnt = bomb_cnt;
        self.empty_cnt = map_size.0 * map_size.1 - bomb_cnt;
        self.curr_pos = (map_size.0 / 2 - 1, map_size.1 / 2 - 1);
        self.over = false;
    }

    pub fn conf_mine_map(mut self, map_size: (u16, u16), bomb_cnt: u16) -> Self {
        self.init_members(map_size, bomb_cnt);
        self
    }

    fn init_map(&mut self) {
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
                    // cover: Some(TileCover::FlagMark),
                    // cover: Some(TileCover::QuestionMark),
                    cover: Some(TileCover::Empty),
                    // cover: None,
                });

                positions.push((x, y));
            }
        }

        positions.shuffle(&mut rng);

        for i in 0..bomb_cnt {
            let (x, y) = positions[i as usize];
            self.mine_map[y as usize][x as usize].content = TileContent::Bomb;
        }

        for i in 0..bomb_cnt {
            let (x, y) = positions[i as usize];
            let x = x as i32;
            let y = y as i32;

            for (dx, dy) in DXDY8 {
                let (new_x, new_y) = (x + dx, y + dy);

                if !(0 <= new_x && new_x < width as i32 && 0 <= new_y && new_y < height as i32) {
                    continue;
                }

                let tile = &mut self.mine_map[new_y as usize][new_x as usize];
                if let TileContent::Empty(num) = tile.content {
                    tile.content = TileContent::Empty(num + 1);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.init_members(self.map_size, self.bomb_cnt);
        self.init_map();
    }

    pub fn init_mine_map(mut self) -> Self {
        self.init_map();
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

    fn game_over(&mut self) {
        for row in self.mine_map.iter_mut() {
            for cell in row {
                match cell.content {
                    TileContent::Bomb => {
                        cell.cover = None;
                    }
                    _ => {}
                }
            }
        }

        self.over = true;
    }

    fn uncover_dfs(&mut self, x: u16, y: u16, go: bool) -> u32 {
        let (width, height) = self.map_size;
        let mut cnt = 1;

        self.mine_map[y as usize][x as usize].cover = None;

        if !go {
            return cnt;
        }

        for (dx, dy) in DXDY4 {
            let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);

            if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                continue;
            }

            let tile = self.mine_map[new_y as usize][new_x as usize];

            if let None = tile.cover {
                continue;
            }

            if let TileContent::Empty(n) = tile.content {
                if n == 0 {
                    cnt += self.uncover_dfs(new_x as u16, new_y as u16, true);
                } else {
                    cnt += self.uncover_dfs(new_x as u16, new_y as u16, false);
                }
            }
        }

        cnt
    }

    pub fn uncover_tile(&mut self) {
        let (x, y) = self.curr_pos;
        let tile = self.mine_map[y as usize][x as usize];

        if let Some(TileCover::FlagMark) = &tile.cover {
            return;
        } else if let None = &tile.cover {
            return;
        }

        match &tile.content {
            TileContent::Empty(n) => {
                let cnt = if *n == 0 {
                    self.uncover_dfs(x, y, true)
                } else {
                    self.uncover_dfs(x, y, false)
                } as u16;

                self.empty_cnt = self.empty_cnt.saturating_sub(cnt);
                if self.empty_cnt == 0 {
                    self.over = true;
                }
            }
            TileContent::Bomb => {
                self.game_over();
                return;
            }
        }
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
