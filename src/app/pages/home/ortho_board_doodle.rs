use leptos::prelude::*;
use rand::Rng;

const ROW_COUNT: usize = 4;
const COLUMN_COUNT: usize = 16;

// struct Grid {
//     grid: Vec<Vec<u32>>
// }

#[component]
pub fn OrthoBoardDoodle() -> impl IntoView {
    let initial_grid = create_initial_grid();
    let (rows, set_rows) = signal(initial_grid);

    let handle_cell_click = move |row_idx: usize, col_idx: usize| {
        move |_| {
            set_rows.update(|current_rows| {
                for (r_idx, row) in current_rows.iter_mut().enumerate() {
                    for (c_idx, cell) in row.iter_mut().enumerate() {
                        if r_idx == row_idx || c_idx == col_idx {
                            *cell += 1;
                        }
                    }
                }
            });
        }
    };

    view! {
        <div class="w-full h-64 sm:h-48 aspect-[4/16] sm:aspect-[16/4]">
            <div class="h-full flex flex-row sm:flex-col">
                {move || {
                    rows.get()
                        .into_iter()
                        .enumerate()
                        .map(|(row_idx, row)| {
                            view! {
                                <div class="h-full flex flex-col flex-1 sm:flex-row">
                                    {row.into_iter()
                                        .enumerate()
                                        .map(|(col_idx, cell_value)| {
                                            let color_class = determine_color(cell_value);
                                            view! {
                                                <div
                                                    class=format!("cursor-pointer flex-1 {}", color_class)
                                                    on:click=handle_cell_click(row_idx, col_idx)
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

fn create_initial_grid() -> Vec<Vec<u32>> {
    let mut rng = rand::rng();
    (0..ROW_COUNT)
        .map(|_| {
            (0..COLUMN_COUNT)
                .map(|_| rng.random_range(0..=100))
                .collect()
        })
        .collect()
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
