use std::cmp::min;

use std::io::{self,Write};
use std::thread;
use std::time::Duration;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> io::Result<()> {
    let mut grid: Vec<Vec<bool>> = empty_grid(16, 16);

    // Allow user to resize grid
    resize_grid(&mut grid);

    // Allow user to place and remove cells
    place_cells(&mut grid);

    clear_terminal();
    println!("Initial state");
    print_grid(&grid, false, Some((3,3)));

    let mut turn = 0;
    loop {
        // Wait for half a second
        thread::sleep(Duration::from_millis(500));

        // Play turn
        play_turn(&mut turn, &mut grid);
        
        // Show state
        clear_terminal();
        println!("Turn {} (Ctrl+C to quit)", turn);
        print_grid(&grid, false, None);
    }
}

fn play_turn(turn: &mut u16, grid: &mut Vec<Vec<bool>>) {
    let starting_state: Vec<Vec<bool>> = grid.clone();
    for l in 0..starting_state.len() {
        for c in 0..starting_state[l].len() {
            play_cell_turn(&starting_state, grid, c, l);
        }
    }
    *turn += 1;
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

/// Pretty-prints a 2D boolean grid with `■` and `□` characters.
/// 
/// * `grid` - 2D boolean grid, to be printed as `■` and `□`.
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
///                    
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
fn print_grid(grid: &Vec<Vec<bool>>, show_numbers: bool, cursor: Option<(usize, usize)>) {
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
        for (c, cell) in row.iter().enumerate() {
            match (cursor, *cell) {
                (Some((x, y)), true) if (x,y) == (c,l) => print!("●"),
                (Some((x, y)), false) if (x,y) == (c,l) => print!("○"),
                (_, true) => print!("■"),
                (_, false) => print!("□"),
            }
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

fn resize_grid(grid: &mut Vec<Vec<bool>>) {
    // Inspired by [How can I read one character from stdin without having to hit enter?](https://stackoverflow.com/a/55881770/10967642)

    // Set terminal to raw mode to allow reading stdin one key at a time
    let stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    clear_terminal();

    stdout.suspend_raw_mode().unwrap();
    // Print helper message
    println!("* Choose a grid size with arrow keys (currently {}x{})", grid.len(), grid[0].len());
    println!("* Hit return (⏎) to save");
    // Print grid
    print_grid(&grid, true, None);
    stdout.activate_raw_mode().unwrap();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if '⏎' is pressed
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Up => {
                    if grid.len() > 1 {
                        *grid = empty_grid(grid[0].len(), grid.len()-1);
                    }
                }
                termion::event::Key::Down => {
                    *grid = empty_grid(grid[0].len(), grid.len()+1);
                }
                termion::event::Key::Left => {
                    if grid[0].len() > 1 {
                        *grid = empty_grid(grid[0].len()-1, grid.len());
                    }
                }
                termion::event::Key::Right => {
                    *grid = empty_grid(grid[0].len()+1, grid.len());
                }
                _ => {}
            }

            clear_terminal();

            stdout.suspend_raw_mode().unwrap();
            // Print helper message
            println!("* Choose a grid size with arrow keys (currently {}x{})", grid.len(), grid[0].len());
            println!("* Hit return (⏎) to save");
            // Print grid
            print_grid(&grid, true, None);
            stdout.activate_raw_mode().unwrap();
        }
        thread::sleep(Duration::from_millis(50));
    }

    clear_terminal();

    stdout.suspend_raw_mode().unwrap();
}

fn place_cells(grid: &mut Vec<Vec<bool>>) {
    // Inspired by [How can I read one character from stdin without having to hit enter?](https://stackoverflow.com/a/55881770/10967642)

    // Set terminal to raw mode to allow reading stdin one key at a time
    let stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    clear_terminal();

    stdout.suspend_raw_mode().unwrap();
    // Print helper message
    println!("* Move around the grid with arrow keys");
    println!("* Toggle cell state by hitting the space bar on your keyboard");
    println!("* Hit return (⏎) to save");
    // Print grid
    print_grid(&grid, true, None);
    stdout.activate_raw_mode().unwrap();

    let mut cursor: (usize, usize) = (0,0);

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if '⏎' is pressed
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Up     if cursor.1 > 0                 => cursor.1 -= 1,
                termion::event::Key::Down   if cursor.1 < grid.len()-1      => cursor.1 += 1,
                termion::event::Key::Left   if cursor.0 > 0                 => cursor.0 -= 1,
                termion::event::Key::Right  if cursor.0 < grid[0].len()-1   => cursor.0 += 1,
                termion::event::Key::Char(' ') => grid[cursor.1][cursor.0] = !grid[cursor.1][cursor.0],
                _ => {}
            }

            clear_terminal();

            stdout.suspend_raw_mode().unwrap();
            // Print helper message
            println!("* Move around the grid with arrow keys");
            println!("* Toggle cell state by hitting the space bar on your keyboard");
            println!("* Hit return (⏎) to save");
            // Print grid
            print_grid(&grid, true, Some(cursor));
            stdout.activate_raw_mode().unwrap();
        }
        thread::sleep(Duration::from_millis(50));
    }

    clear_terminal();

    stdout.suspend_raw_mode().unwrap();
}

fn clear_terminal() {
    write!(io::stdout(), "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
}

/// Instantiates a new grid filled with `false`.
fn empty_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}
