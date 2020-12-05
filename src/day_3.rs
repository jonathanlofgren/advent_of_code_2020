use crate::utils;
type Grid = Vec<Vec<char>>;

fn count_trees(grid: &Grid, right: usize, down: usize) -> usize {
    get_path(grid, right, down)
        .iter()
        .filter(|&c| c == &'#')
        .count()
}

fn trees_multiplied(grid: &Grid, slopes: &[(usize, usize)]) -> i64 {
    slopes.iter()
        .map(|(right, down)| count_trees(grid, *right, *down) as i64)
        .product()
}

fn get_path(grid: &Grid, right: usize, down: usize) -> Vec<char> {
    let coords = (0..).step_by(right).zip((0..).step_by(down));
    let rows = grid.len();
    let cols = grid[0].len();

    coords
        .map(|coord| {
            if coord.1 < rows {
                Some(grid[coord.1][coord.0 % cols])
            } else {
                None
            }
        })
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

pub fn main() {
    let lines: Vec<String> = utils::read_lines_to_vec("data/day_3.txt");
    let grid: Grid = lines.iter().map(|l| l.chars().collect()).collect();
    let part_2_slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    println!("======== Day 3 ========");
    println!("Part 1 = {}", count_trees(&grid, 3, 1));
    println!("Part 2 = {}", trees_multiplied(&grid, &part_2_slopes));
}


#[test]
fn test_get_path() {
    let grid: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.'],
        vec!['.', '#', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['#', '.', '.', '.'],
    ];
    let expected = vec!['.', '#', '.', '.', '#'];

    let path = get_path(&grid, 1, 1);

    assert_eq!(path, expected);
}

#[test]
fn test_trees_multiplied() {
    let grid: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.'],
        vec!['.', '#', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['#', '.', '.', '.'],
    ];
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 1), (1, 1)];

    assert_eq!(trees_multiplied(&grid, &slopes), 8);
}