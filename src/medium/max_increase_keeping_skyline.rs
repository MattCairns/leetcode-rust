pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let col = grid[0].len();
    let mut grid_max = grid.clone();

    (0..row).for_each(|i| {
        (0..col).for_each(|j| {
            let mut max_col = grid[i][j];
            let mut max_row = grid[i][j];
            (0..col).for_each(|x| {
                if max_col < grid[x][j] {
                    max_col = grid[x][j];
                }
            });
            (0..row).for_each(|y| {
                if max_row < grid[i][y] {
                    max_row = grid[i][y];
                }
            });

            grid_max[i][j] = if max_col < max_row { max_col } else { max_row };
        });
    });

    let s1: i32 = grid_max.iter().map(|x| -> i32 { x.iter().sum() }).sum();
    let s2: i32 = grid.iter().map(|x| -> i32 { x.iter().sum() }).sum();
    s1 - s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_increase_keeping_skyline() {
        let grid = vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ];

        assert_eq!(max_increase_keeping_skyline(grid), 35);
    }
}
