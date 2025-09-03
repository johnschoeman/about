use leptos::prelude::*;
use rand::Rng;
use std::time::Duration;

const ROW_COUNT: usize = 5;
const COLUMN_COUNT: usize = 13;

const TICK_DURATION: u64 = 6;
// const TICK_DURATION: u64 = 1;

const ACTIVE_MODS: [u32; 2] = [2, 5];
const GREEN_MODS: [u32; 1] = [37];
const BLUE_MODS: [u32; 1] = [17];
const PURPLE_MODS: [u32; 1] = [49];

#[derive(Clone)]
pub struct Grid {
    data: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut rng = rand::rng();
        let data = (0..rows)
            .map(|_| (0..cols).map(|_| rng.random_range(0..=100)).collect())
            // .map(|_| (0..cols).map(|_| rng.random_range(0..=5)).collect())
            .collect();

        Self { data, rows, cols }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<u32> {
        self.data.get(row)?.get(col).copied()
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: u32) {
        if let Some(cell) = self.data.get_mut(row).and_then(|r| r.get_mut(col)) {
            *cell = value;
        }
    }

    pub fn increment_cell(&mut self, row_idx: usize, col_idx: usize) {
        if let Some(cell) = self.data.get_mut(row_idx).and_then(|r| r.get_mut(col_idx)) {
            *cell += 1;
        }
    }

    pub fn increment_row_and_column(&mut self, row_idx: usize, col_idx: usize) {
        for (r_idx, row) in self.data.iter_mut().enumerate() {
            for (c_idx, cell) in row.iter_mut().enumerate() {
                if r_idx == row_idx || c_idx == col_idx {
                    *cell += 1;
                }
            }
        }
    }

    pub fn increment_all_cells(&mut self) {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell += 1;
            }
        }
    }

    pub fn increment_random_cell(&mut self) {
        let mut rng = rand::rng();
        let row = rng.random_range(0..self.rows);
        let col = rng.random_range(0..self.cols);

        if let Some(cell) = self.data.get_mut(row).and_then(|r| r.get_mut(col)) {
            *cell += 1;
        }
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<u32> {
        let mut neighbors = Vec::new();
        let directions = [(-1, 0), (0, -1), (0, 1), (1, 0)];

        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0
                && new_row < self.rows as i32
                && new_col >= 0
                && new_col < self.cols as i32
            {
                if let Some(value) = self.get_cell(new_row as usize, new_col as usize) {
                    neighbors.push(value);
                }
            }
        }

        neighbors
    }

    pub fn update_cellular_automaton(&mut self) {
        let mut new_data = self.data.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(current_value) = self.get_cell(row, col) {
                    let neighbors = self.get_neighbors(row, col);
                    let active_neighbor_count = neighbors.iter().filter(|&&n| is_active(n)).count();
                    let inactive_neighbor_count =
                        neighbors.iter().filter(|&&n| !is_active(n)).count();

                    let is_cell_active = is_active(current_value);

                    let should_increment = if is_cell_active {
                        active_neighbor_count == 2
                            || inactive_neighbor_count == 4
                            || inactive_neighbor_count == 2
                    } else {
                        inactive_neighbor_count == 2
                    };

                    if should_increment {
                        new_data[row][col] = current_value + 1;
                    }
                }
            }
        }

        self.data = new_data;
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<u32>> {
        self.data.iter()
    }

    pub fn into_rows(self) -> Vec<Vec<u32>> {
        self.data
    }
}

#[component]
pub fn OrthoBoardDoodle() -> impl IntoView {
    let initial_grid = Grid::new(ROW_COUNT, COLUMN_COUNT);
    let (grid, set_grid) = signal(initial_grid);

    Effect::new(move |_| {
        set_interval(
            move || {
                set_grid.update(|current_grid| {
                    current_grid.update_cellular_automaton();
                });
            },
            Duration::from_secs(TICK_DURATION),
        );
    });

    let handle_cell_click = move |row_idx: usize, col_idx: usize| {
        move |_| {
            set_grid.update(|current_grid| {
                // current_grid.increment_row_and_column(row_idx, col_idx);
                current_grid.increment_cell(row_idx, col_idx);
            });
        }
    };

    // let container_style = format!("w-full grid grid-cols-{} gap-0", COLUMN_COUNT);

    view! {
        <div class="w-full grid grid-cols-13 gap-0">
            {move || {
                grid.get()
                    .into_rows()
                    .into_iter()
                    .enumerate()
                    .flat_map(|(row_idx, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(move |(col_idx, cell_value)| {
                                let color_class = determine_color(cell_value);
                                let text = format!("{}", cell_value);
                                view! {
                                    <div
                                        class=format!("cursor-pointer aspect-square {}", color_class)
                                        on:click=handle_cell_click(row_idx, col_idx)
                                    // >{text}</div>
                                    ></div>
                                }
                            })
                    })
                    .collect_view()
            }}
        </div>
    }
}

fn is_active(value: u32) -> bool {
    is_mod_of(value, &ACTIVE_MODS)
}

fn is_green(value: u32) -> bool {
    is_mod_of(value, &GREEN_MODS)
}

fn is_blue(value: u32) -> bool {
    is_mod_of(value, &BLUE_MODS)
}

fn is_purple(value: u32) -> bool {
    is_mod_of(value, &PURPLE_MODS)
}

fn is_mod_of(value: u32, moduli: &[u32]) -> bool {
    moduli.iter().any(|&m| value % m == 0)
}

fn determine_color(value: u32) -> &'static str {
    let white_style = "bg-white hover:bg-gray-100";
    let gray_style = "bg-gray-800 hover:brightness-150";
    let purple_style = "bg-purple-300 hover:brightness-95";
    let blue_style = "bg-blue-300 hover:brightness-95";
    let green_style = "bg-green-300 hover:brightness-95";

    if is_active(value) {
        white_style
    } else if is_green(value) {
        green_style
    } else if is_blue(value) {
        blue_style
    } else if is_purple(value) {
        purple_style
    } else {
        gray_style
    }
}
