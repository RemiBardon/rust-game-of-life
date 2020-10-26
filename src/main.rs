use std::cmp::{min, max};

fn main() {
    let width: usize = 8;
    let height: usize = 8;

    let mut grid: Vec<Vec<bool>> = empty_grid(width, height);
    grid[2][7] = true;

    print_grid(&grid, true);

    println!("{} cell(s) around ({},{})", neighbour_count(&grid, 7, 2), 7, 2);
    println!("{} cell(s) around ({},{})", neighbour_count(&grid, 7, 1), 7, 1);
    println!("{} cell(s) around ({},{})", neighbour_count(&grid, 5, 1), 5, 1);
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
    // `max` ensures we don't go under index `0`
    // `min` ensures we don't go out of bounds
    let line_range = max(y-1, 0)..=min(y+1, grid.len()-1);
    let column_range = |l: usize| max(x-1, 0)..=min(x+1, grid[l].len()-1);

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
