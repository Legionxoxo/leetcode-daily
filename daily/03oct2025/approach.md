# Trapping Rain Water II – Approach

**Problem Link:** [LeetCode – Trapping Rain Water II](https://leetcode.com/problems/trapping-rain-water-ii/?envType=daily-question&envId=2025-10-03)

## Problem Restatement

We are given a 2D grid where each cell represents the elevation of terrain. We need to compute how much water can be trapped after raining. Unlike the 1D version, here water can flow in **four directions** (up, down, left, right).

## Key Intuition

-   Water trapping depends on the **minimum boundary height** surrounding a cell.
-   Instead of checking all directions for each cell (which is expensive), we can simulate water filling **from the boundary inward**, always starting with the **lowest wall**.
-   Think of it like filling a swimming pool: water leaks out of the lowest boundary first, so we must process from that point.

## Approach

### 1. Use a Min-Heap (Priority Queue)

-   Push all **boundary cells** (edges of the grid) into a **min-heap**, storing their heights.
-   Mark them as **visited** so we don’t process them again.
-   These boundary walls act as the starting walls of the container.

### 2. Process Cells from Lowest Boundary

-   While the heap is not empty:

    -   Pop the **lowest-height cell**.
    -   For each **unvisited neighbor** (up, down, left, right):

        -   If the neighbor’s height is **lower than the current boundary**, then water is trapped:

            -   Trapped water = `currentBoundaryHeight - neighborHeight`.
            -   Update the neighbor’s effective height to `currentBoundaryHeight` (since water raises its level).

        -   Otherwise, no water is trapped; use the neighbor’s own height as the new boundary.
        -   Push the neighbor into the heap with its effective height.

    -   Mark the neighbor as visited.

### 3. Continue Until All Cells Are Processed

-   Repeat until the heap is empty.
-   Accumulate the trapped water in each step.

## Why This Works

-   By always expanding from the **lowest boundary first**, we ensure that water trapping is correctly computed—no overestimation.
-   This method implicitly ensures that each cell’s trapping is governed by the _lowest_ possible wall around it.

## Complexity Analysis

-   **Time Complexity:** `O(m * n * log(m * n))`

    -   Because each cell is processed and heap operations cost `O(log(m * n))`.

-   **Space Complexity:** `O(m * n)`

    -   Due to storing the visited grid and the heap entries.

## Example

For the grid:

```
1 4 3 1 3 2
3 2 1 3 2 4
2 3 3 2 3 1
```

-   Initialize the heap with boundary cells.
-   Expand inward, trapping water when encountering lower neighbors.
-   Final total trapped water = **4**.

---
