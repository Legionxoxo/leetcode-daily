### 🟢 [Minimum Score Triangulation of Polygon](https://leetcode.com/problems/minimum-score-triangulation-of-polygon/?envType=daily-question&envId=2025-09-29)

---

## Approach & Explanation

We are given a convex polygon with `n` vertices.
Each vertex has an integer value, and we need to split the polygon into triangles (a triangulation).
The **score** of a triangle is the product of the values of its three vertices.
Our goal is to find the **minimum total score** of any triangulation.

---

### Key Insight – Optimal Substructure

If we pick any two vertices `i` and `j` as boundaries of a sub-polygon,
the optimal triangulation inside that sub-polygon can be broken into **smaller sub-polygons**.
Choosing a third vertex `k` (where `i < k < j`) forms one triangle `(i, k, j)` and splits the region into:

-   Left sub-polygon `[i … k]`
-   Right sub-polygon `[k … j]`

The total cost is:

```
cost = dp[i][k] + (values[i] * values[k] * values[j]) + dp[k][j]
```

We need to try all possible `k` and pick the minimum.

---

### Dynamic Programming Formulation

-   **State:** `dp[i][j]` = minimum score to triangulate the sub-polygon from vertex `i` to vertex `j`.
-   **Base Case:**

    -   If the sub-polygon has fewer than 3 vertices (`j - i < 2`), no triangle can be formed → cost = `0`.

-   **Transition:**

    ```
    dp[i][j] = min over all k in (i, j)
               [ dp[i][k] + values[i]*values[k]*values[j] + dp[k][j] ]
    ```

-   **Goal:** `dp[0][n-1]` gives the minimum score for the entire polygon.

---

### Filling Order

To ensure smaller sub-polygons are solved first:

1. Iterate `i` **backwards** (from `n-1` down to `0`).
2. For each `i`, iterate `j` **forwards** (from `i+1` up to `n-1`).
3. For each `(i, j)`, test every `k` between them.

This guarantees that when we calculate `dp[i][j]`,
the subproblems `dp[i][k]` and `dp[k][j]` are already computed.

---

### Complexity

-   **Time Complexity:** `O(n³)`

    -   For every pair `(i, j)` we try all possible `k`.

-   **Space Complexity:** `O(n²)`

    -   A 2D DP table of size `n x n` is required.

---

### Summary

The polygon is solved bottom-up:

-   Start with small triangles,
-   Expand to larger sub-polygons,
-   Combine their optimal scores to compute the final minimum.
