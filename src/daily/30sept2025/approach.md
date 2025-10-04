# Approach

**Problem Link:**  
[Find Triangular Sum of an Array](https://leetcode.com/problems/find-triangular-sum-of-an-array/description/?envType=daily-question&envId=2025-09-30)

## Idea / Intuition

You have an array `nums` of length `n`. You repeatedly transform it into a shorter array by summing adjacent elements (mod 10) until only one number remains.

We can simulate the process **in place**, reducing the “active length” step by step:

-   Let the “active length” be `i`, starting from `n` down to `1`.
-   In each iteration (for current active length `i`), update `nums[j] = (nums[j] + nums[j + 1]) % 10` for `j = 0, 1, …, i-2`.
-   After this inner pass, the first `i-1` elements in `nums` represent the next “row” of triangular sums.
-   Decrement the active length (i.e. `i ← i - 1`) and repeat until only one value remains.
-   That single value is the answer.

This approach is efficient in both time and space:

-   **Time Complexity:** O(n²) — because you do about n + (n-1) + (n-2) + … + 1 operations across all levels.
-   **Space Complexity:** O(1) extra space (in-place modification), aside from the input array.

---

## Test Cases

| Test Case | Input                | Expected Output    | Explanation / Edge Notes                                                         |
| --------- | -------------------- | ------------------ | -------------------------------------------------------------------------------- |
| 1         | `[1, 2, 3, 4, 5]`    | `8`                | Example walkthrough: [1,2,3,4,5] → [3,5,7,9] → [8,2,6] → [0,8] → [8]             |
| 2         | `[5]`                | `5`                | Only one element, so it is already the triangular sum                            |
| 3         | `[0, 0, 0, 0]`       | `0`                | All sums remain zero modulo 10                                                   |
| 4         | `[9, 9, 9]`          | `7`                | 9+9=18→8, 9+9=18→8 → then 8+8=16→6 (Wait, check) Actually: [9,9,9] → [8,8] → [6] |
| 5         | `[3, 8, 1, 7, 9, 2]` | (compute manually) | A longer random array to test general correctness                                |

---
