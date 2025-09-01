use leptos::prelude::*;
use rand::Rng;
use std::time::Duration;

const ROW_COUNT: usize = 8;
const COLUMN_COUNT: usize = 24;

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
                    current_grid.increment_random_cell();
                });
            },
            Duration::from_secs(1),
        );
    });

    let handle_cell_click = move |row_idx: usize, col_idx: usize| {
        move |_| {
            set_grid.update(|current_grid| {
                current_grid.increment_row_and_column(row_idx, col_idx);
            });
        }
    };

    view! {
        <div class="w-full grid grid-cols-24 gap-0">
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
                                view! {
                                    <div
                                        class=format!("cursor-pointer aspect-square {}", color_class)
                                        on:click=handle_cell_click(row_idx, col_idx)
                                    />
                                }
                            })
                    })
                    .collect_view()
            }}
        </div>
    }
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

    if is_mod_of(value, &[2, 5]) {
        white_style
    } else if is_mod_of(value, &[23]) {
        green_style
    } else if is_mod_of(value, &[49]) {
        blue_style
    } else if is_mod_of(value, &[17]) {
        purple_style
    } else {
        gray_style
    }
}
