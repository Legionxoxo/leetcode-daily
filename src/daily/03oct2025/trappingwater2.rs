use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn trapping_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        if m == 0 {
            return 0;
        }
        let n = height_map[0].len();
        if n == 0 {
            return 0;
        }

        let mut visited = vec![vec![false; n]; m];
        let mut heap = BinaryHeap::new(); // min-heap via Reverse

        //Push all boundary cells
        for i in 0..m {
            heap.push(Reverse((height_map[i][0], i, 0)));
            heap.push(Reverse((height_map[i][n - 1], i, n - 1)));
            visited[i][0] = true;
            visited[i][n - 1] = true;
        }

        for j in 0..n {
            heap.push(Reverse((height_map[0][j], 0, j)));
            heap.push(Reverse((height_map[m - 1][j], m - 1, j)));
            visited[0][j] = true;
            visited[m - 1][j] = true;
        }

        let mut water = 0;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Reverse((height, row, col))) = heap.pop() {
            for (dx, dy) in &dirs {
                let new_row = row as i32 + dx;
                let new_col = col as i32 + dy;

                if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                    continue;
                }
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                if visited[new_row][new_col] {
                    continue;
                }

                visited[new_row][new_col] = true;

                if height_map[new_row][new_col] < height {
                    water += height - height_map[new_row][new_col];
                    heap.push(Reverse((height, new_row, new_col)));
                } else {
                    heap.push(Reverse((height_map[new_row][new_col], new_row, new_col)));
                }
            }
        }
        water
    }
}

fn main() {
    let height_map = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1],
    ];

    let result1 = Solution::trapping_water(height_map);

    println!(
        "Correct answer is 4. The code produced result -> {}",
        result1
    );
}
