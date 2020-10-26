fn main() {
    let width: usize = 2;
    let height: usize = 3;

    let state: Vec<Vec<bool>> = empty_grid(width, height);

    println!("{:?}", state);
}

fn empty_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}
