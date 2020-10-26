use std::cmp::min;

fn main() {
    let width: usize = 8;
    let height: usize = 8;

    let mut grid: Vec<Vec<bool>> = empty_grid(width, height);

    // Add a block
    grid[2][6] = true;
    grid[2][7] = true;
    grid[3][6] = true;
    grid[3][7] = true;

    // Add a blinker
    grid[6][3] = true;
    grid[6][4] = true;
    grid[6][5] = true;

    print_grid(&grid, true);

    loop {
        // Wait for a second
        std::thread::sleep(std::time::Duration::from_millis(500));

        play_turn(&mut grid);
        print_grid(&grid, true);
    }
}

fn play_turn(grid: &mut Vec<Vec<bool>>) {
    let starting_state: Vec<Vec<bool>> = grid.clone();
    for l in 0..starting_state.len() {
        for c in 0..starting_state[l].len() {
            play_cell_turn(&starting_state, grid, c, l);
        }
    }
}

/// **Rules:**
/// 
/// 1. Any live cell with two or three live neighbours survives.
/// 2. Any dead cell with three live neighbours becomes a live cell.
/// 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
fn play_cell_turn(starting_state: &Vec<Vec<bool>>, grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    match (starting_state[y][x], neighbour_count(starting_state, x, y)) {
        (true, 2..=3) => {}, // Nothing happens
        (true, _) => grid[y][x] = false,
        (false, 3) => grid[y][x] = true,
        (false, _) => {}, // Nothing happens
    }
}

/// Pretty-prints a 2D boolean grid with `◼︎` and `◻︎` characters.
/// 
/// * `grid` - 2D boolean grid, to be printed as `◼︎` and `◻︎`.
/// * `show_numbers` - Show numbers around grid or not.
/// 
///   * If `show_numbers` is set to `false`, function will print:
/// 
///     ```txt
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◼◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◻◼◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◼◻◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◼◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ◻◻◻◻◻◻◻◻◻◻◼◻
///     ◻◻◻◻◻◻◻◻◻◻◻◻
///     ```
/// 
///   * If `show_numbers` is set to `true`, function will print:
///     
///     ```txt
///                  11
///        012345678901
///       0◻◻◻◻◻◻◻◻◻◻◻◻
///       1◻◻◼◻◻◻◻◻◻◻◻◻
///       2◻◻◻◻◻◻◻◼◻◻◻◻
///       3◻◻◻◻◻◻◻◻◻◻◻◻
///       4◻◻◻◻◻◻◻◻◻◻◻◻
///       5◻◻◻◻◻◻◼◻◻◻◻◻
///       6◻◻◻◻◻◻◻◻◻◻◻◻
///       7◻◻◻◻◻◻◻◻◻◻◻◻
///       8◻◻◻◻◼◻◻◻◻◻◻◻
///       9◻◻◻◻◻◻◻◻◻◻◻◻
///      10◻◻◻◻◻◻◻◻◻◻◼◻
///      11◻◻◻◻◻◻◻◻◻◻◻◻
///     ```
/// 
/// - **Warning:** Might have unwanted behavior with non-square grids.
fn print_grid(grid: &Vec<Vec<bool>>, show_numbers: bool) {
    if show_numbers {
        let mut indices: Vec<Vec<char>> = vec![];
        for index in 0..grid[0].len() {
            indices.push(format!("{:>3}", index).chars().collect());
        }

        for l in 0..indices[0].len() {
            // Print leading spaces (grid is offset by row numbers)
            print!("{}", " ".repeat(3));

            // Get digit line
            let digit_line = indices.iter().map(|s| s[l]);

            // Print digit line
            for digit in digit_line {
                print!("{}", digit);
            }

            println!("");
        }
    }

    for (l, row) in grid.iter().enumerate() {
        if show_numbers {
            print!("{:>3}", l);
        }
        for cell in row {
            print!("{}", if *cell { "◼︎" } else { "◻︎" });
        }
        println!("");
    }
}

/// Iterate over neighbouring cells to find neighbour count.
/// 
/// - **Warning:** Will panic if `grid` is empty (size `0`).
fn neighbour_count(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count: u8 = 0;

    // Find lines and columns to go through
    // `min_line` ensures we don't go under index `0`
    let min_line = if y == 0 { 0 } else { y-1 };
    // `min()` ensures we don't go out of bounds
    let line_range = min_line..=min(y+1, grid.len()-1);
    let min_column = if x == 0 { 0 } else { x-1 };
    let column_range = |l: usize| min_column..=min(x+1, grid[l].len()-1);

    for l in line_range {
        for c in column_range(l) {
            if (c,l) != (x,y) {
                // If cell is not targetted cell
                // Increment neighbour count
                count += if grid[l][c] { 1 } else { 0 };
            }
        }
    }

    return count
}

/// Instantiates a new grid filled with `false`.
fn empty_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}
