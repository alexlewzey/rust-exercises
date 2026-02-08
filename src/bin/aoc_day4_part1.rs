use std::{fs, path};

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let data_path = path::PathBuf::from(root).join("data").join("aoc_day_4.txt");
    let data = fs::read_to_string(data_path).unwrap();
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let h = grid.len();
    let w = grid[0].len();
    println!("height: {h} width: {w}");

    const DELTAS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut n_accessible_rolls = 0;
    for r_i in 0..h {
        for c_i in 0..w {
            let roll = grid[r_i][c_i];
            if roll == '.' {
                continue;
            }
            let mut paper_neighbours: i32 = 0;
            for (x, y) in &DELTAS {
                let row_idx_ = r_i as isize + x;
                let col_idx_ = c_i as isize + y;
                if row_idx_ >= 0 && col_idx_ >= 0 && row_idx_ < w as isize && col_idx_ < h as isize
                {
                    let neighbour_value = grid[row_idx_ as usize][col_idx_ as usize];
                    if neighbour_value == '@' {
                        paper_neighbours += 1
                    }
                }
            }
            if paper_neighbours < 4 {
                n_accessible_rolls += 1
            }
        }
    }

    println!("n_accessible_rolls: {}", n_accessible_rolls);

    // for row in grid {
    //     println!("{:?}", row);
    // }
}
