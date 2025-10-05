# Approach — Pacific Atlantic Water Flow

Link: [https://leetcode.com/problems/pacific-atlantic-water-flow/description/?envType=daily-question&envId=2025-10-05](https://leetcode.com/problems/pacific-atlantic-water-flow/description/?envType=daily-question&envId=2025-10-05)

---

## Problem Restatement

Given a matrix `heights` of size `m × n`, each cell has an integer height. Water can flow from a cell to any of its four neighbors (up, down, left, right) **if and only if** the neighbor’s height is **less than or equal** to the current cell’s height.

Find all coordinates `(i, j)` from which water can flow to **both** the Pacific and Atlantic oceans.

-   The Pacific touches the top and left edges of the matrix.
-   The Atlantic touches the bottom and right edges of the matrix.

Return the list of valid coordinates.

---

## Key Insight & Strategy

Instead of starting a search (DFS/BFS) from **every cell** and checking if you can reach both oceans (which would be very inefficient), reverse the direction:

1. **Start from the oceans and move inward**, i.e., from boundary cells that touch an ocean.
2. Perform **BFS (or DFS)** from those boundaries, moving only to neighbors **of higher or equal height** (so that water _could_ flow from those neighbors down to the ocean).
3. Mark all cells reachable in this reversed sense as able to flow into that ocean.
4. Do this separately for Pacific boundary and Atlantic boundary.
5. The intersection of the two reachable sets gives the cells that can reach **both** oceans.

This exploits the monotonicity constraint (you can only go “uphill or level” in the reversed graph) and ensures each cell is processed at most once per ocean.

---

## Step-by-Step Approach

1. **Pre-check / Edge Cases**

    - If `heights` is empty or `heights[0]` is empty, return an empty result.
    - Let `m = heights.size()`, `n = heights[0].size()`.

2. **Prepare Directions**

    - Use an array `dir = [(1,0), (-1,0), (0,1), (0,-1)]` to move in the four cardinal directions.

3. **Initialize BFS Queues for Oceans**

    - Create two queues (or stacks) `Q_Pacific` and `Q_Atlantic`.
    - For Pacific, enqueue all cells on the top row `(0, j)` for `j = 0..n-1` and the left column `(i, 0)` for `i = 0..m-1`.
    - For Atlantic, enqueue all cells on the bottom row `(m − 1, j)` and the right column `(i, n − 1)`.

4. **BFS / Flood Fill (Reversed Flow)**

    - Write a helper that takes a queue and returns a 2D boolean matrix `reachable` of size `m × n`, initially all `false`.
    - While the queue is not empty:

        - Pop a cell `(r, c)`.
        - If `reachable[r][c]` is already `true`, skip.
        - Mark `reachable[r][c] = true`.
        - For each neighbor `(nr, nc)` among four directions:

            - If out-of-bounds, skip.
            - If `reachable[nr][nc]` is already true, skip.
            - If `heights[nr][nc] < heights[r][c]`, skip (since reversed direction must go uphill or flat).
            - Otherwise, enqueue `(nr, nc)`.

5. **Compute Intersection**

    - Call BFS from Pacific queue → get `canReachPacific`.
    - Call BFS from Atlantic queue → get `canReachAtlantic`.
    - Iterate over all cells `(i, j)`, and collect those where both `canReachPacific[i][j]` and `canReachAtlantic[i][j]` are `true`.

6. **Return the List of Coordinates**

---

## Complexity Analysis

-   **Time Complexity**: O(m × n)
    Each BFS explores each cell at most once (per ocean), doing constant work per neighbor. So overall ~2 × (m·n) → O(mn).

-   **Space Complexity**: O(m × n)
    For the `visited / reachable` matrices and the BFS queues in the worst case.

---

## Why This Reversal Trick Works

-   Trying to start from every cell and check whether it can reach both oceans requires redundant repeated traversals, leading to O((mn) × traversal) time — too slow.
-   Reversing the problem (flow from ocean inward) leverages that water flows downhill in the forward direction, so in reverse you can only go uphill or flat. This forms a directed graph you can traverse once per ocean.
-   Intersection finds cells that are reachable in both reversed graphs, which is equivalent to cells from which water can flow to both oceans in the original direction.
