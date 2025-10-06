# 🏊‍♂️ Swim in Rising Water – Problem Approach

Link: [https://leetcode.com/problems/swim-in-rising-water/description/?envType=daily-question&envId=2025-10-06]

## Problem Overview

Given an `n x n` grid representing elevations, at time `t`, the water level is `t` everywhere. You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most `t`. Your task is to determine the minimum time `t` until you can reach the bottom-right corner `(n-1, n-1)` starting from the top-left corner `(0, 0)`.

## Approach

### 1. **Understanding the Problem**

-   **Grid Representation**: Each cell in the grid has an elevation value.
-   **Water Level**: At any given time `t`, the water level is `t` everywhere.
-   **Movement Restriction**: You can only move to adjacent cells if both the current and the adjacent cell's elevation are less than or equal to `t`.
-   **Objective**: Find the minimum `t` such that there exists a path from `(0, 0)` to `(n-1, n-1)` where all cells along the path have elevations ≤ `t`.

### 2. **Algorithmic Strategy**

To solve this problem efficiently, we can employ a **Binary Search** combined with a **Depth-First Search (DFS)** or **Breadth-First Search (BFS)**.

#### a. **Binary Search**

-   **Purpose**: To find the minimum time `t` where a path exists.
-   **Range**: The binary search will operate between the minimum and maximum elevation values in the grid.

    -   **Low Bound (`l`)**: The minimum elevation in the grid.
    -   **High Bound (`h`)**: The maximum elevation in the grid.

#### b. **DFS/BFS for Path Validation**

-   For a given `t`, use DFS or BFS to check if there's a path from `(0, 0)` to `(n-1, n-1)` where all traversed cells have elevations ≤ `t`.
-   **DFS/BFS Details**:

    -   Start from `(0, 0)`.
    -   Explore all 4 possible directions (up, down, left, right).
    -   Only move to adjacent cells if their elevation is ≤ `t` and they haven't been visited yet.
    -   If you reach `(n-1, n-1)`, return `true`; otherwise, return `false`.

#### c. **Combining Binary Search and DFS/BFS**

-   Perform a binary search on `t`:

    -   **Midpoint Calculation**: `mid = (l + h) / 2`.
    -   **Path Check**: Use DFS/BFS to check if a path exists for the current `mid`.

        -   If a path exists, it means we might be able to swim at an even lower water level, so set `h = mid - 1`.
        -   If no path exists, it means we need to wait for a higher water level, so set `l = mid + 1`.

-   The process continues until `l` exceeds `h`, at which point the lowest `t` for which a path exists is found.

### 3. **Time Complexity**

-   **Binary Search Complexity**: `O(log(max_elevation - min_elevation))`, where `max_elevation` and `min_elevation` are the maximum and minimum elevation values in the grid.
-   **DFS/BFS Complexity**: `O(n^2)`, where `n` is the dimension of the grid.
-   **Overall Complexity**: `O(n^2 * log(max_elevation - min_elevation))`.

### 4. **Space Complexity**

-   The space complexity is primarily due to the storage required for the visited cells during DFS/BFS, which is `O(n^2)`.

### 5. **Edge Cases**

-   **Single Cell Grid**: If the grid is `1x1`, no swimming is needed; the answer is `0`.
-   **Disconnected Regions**: Ensure that the algorithm correctly identifies when no path exists due to elevation constraints.

## Conclusion

By combining binary search with DFS/BFS, we efficiently determine the minimum time required to swim from the top-left to the bottom-right corner of the grid. This approach balances the need to explore different water levels with the necessity of checking path feasibility, leading to an optimal solution.

---
