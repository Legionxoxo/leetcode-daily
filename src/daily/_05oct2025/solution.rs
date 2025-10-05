use std::collections::VecDeque;
pub struct Solution;

impl Solution {
    pub fn run(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row: usize = heights.len();
        let col: usize = heights[0].len();

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn bfs(
            heights: &Vec<Vec<i32>>,
            q: &mut VecDeque<(usize, usize)>,
            directions: &[(i32, i32); 4],
        ) -> Vec<Vec<bool>> {
            let rows = heights.len();
            let cols = heights[0].len();

            let mut visited = vec![vec![false; cols]; rows];

            while let Some((r, c)) = q.pop_front() {
                if visited[r][c] {
                    continue;
                }

                visited[r][c] = true;

                for &(dr, dc) in directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr < 0
                        || nc < 0
                        || nr >= rows as i32
                        || nc >= cols as i32
                        || visited[nr as usize][nc as usize]
                        || heights[nr as usize][nc as usize] < heights[r][c]
                    {
                        continue;
                    }

                    q.push_back((nr as usize, nc as usize));
                }
            }
            visited
        }

        let mut pacific = VecDeque::new();
        let mut atlantic = VecDeque::new();

        for i in 0..row {
            pacific.push_back((i, 0));
            atlantic.push_back((i, col - 1));
        }
        for j in 0..col {
            pacific.push_back((0, j));
            atlantic.push_back((row - 1, j));
        }

        let can_pacific = bfs(&heights, &mut pacific, &directions);
        let can_atlantic = bfs(&heights, &mut atlantic, &directions);

        let mut result = vec![];

        for i in 0..row {
            for j in 0..col {
                if can_pacific[i][j] && can_atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_run() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];

        let result1 = super::Solution::run(heights);
        assert_eq!(
            result1,
            [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        )
    }
}
