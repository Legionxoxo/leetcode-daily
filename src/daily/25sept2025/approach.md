# 🏔️ LeetCode 120 – Triangle

[Problem Link](https://leetcode.com/problems/triangle/)

---

## Approach

We need to find the minimum path sum from the top of the triangle to the bottom, moving only to adjacent numbers on the next row.

Instead of using recursion (which causes heavy call stack usage and can lead to TLE on large inputs), we use a **bottom-up dynamic programming** strategy:

1. Start from the **second-last row** of the triangle and move upward.
2. For each element `triangle[i][j]`, compute:
   triangle[i][j] += min(triangle[i+1][j], triangle[i+1][j+1])

This means each cell accumulates the minimum path sum from itself to the bottom. 3. By the time we reach the top row, `triangle[0][0]` contains the minimum path sum.

To optimize space, instead of modifying the triangle directly, we can use a **1D DP array**:

-   Initialize `dp` as the last row of the triangle.
-   Iterate from the second-last row upward, updating `dp[j]` as:
    dp[j] = triangle[i][j] + min(dp[j], dp[j+1])

-   After processing all rows, `dp[0]` holds the final answer.

This eliminates recursion overhead and reduces memory usage.

---

## Time Complexity

-   **O(n²)** – Each cell is processed exactly once, where `n` is the number of rows.

## Space Complexity

-   **O(n)** – Using a 1D DP array that stores only one row at a time.
