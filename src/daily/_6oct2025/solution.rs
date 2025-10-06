struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        let root = self.find_parent(self.parent[node]);
        self.parent[node] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut parent_a = self.find_parent(a);
        let mut parent_b = self.find_parent(b);
        if parent_a == parent_b {
            return;
        }

        if self.size[parent_a] < self.size[parent_b] {
            std::mem::swap(&mut parent_a, &mut parent_b);
        }

        self.parent[parent_b] = parent_a;
        self.size[parent_a] += self.size[parent_b];
    }
}

pub struct Solution;

impl Solution {
    pub fn run(grid: Vec<Vec<i32>>) -> i32 {
        let mut n = grid.len();
        let mut dsu = DSU::new(n * n);

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut graph = vec![(0, 0); n * n];

        for i in 0..n {
            for j in 0..n {
                graph[grid[i][j] as usize] = (i as usize, j as usize);
            }
        }

        for time in 0..(n * n) {
            let (row, col) = graph[time];

            for (dx, dy) in directions {
                let new_row = row + dx;
                let new_col = col + dy;

                if new_row >= 0
                    && new_row < n as i32
                    && new_col >= 0
                    && new_col < n as i32
                    && grid[new_row as usize][new_col as usize] <= time as i32
                {
                    dsu.union_by_size(
                        (row as usize) * n + (col as usize),
                        (new_row as usize) * n + (new_col as usize),
                    );
                }
            }

            if dsu.find_parent(0) == dsu.find_parent(n * n - 1) {
                return time as i32;
            }
        }
        -1
    }
}
