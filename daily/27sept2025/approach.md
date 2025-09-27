# 🔺 LeetCode – Largest Triangle Area

[Problem Link](https://leetcode.com/problems/largest-triangle-area/description/?envType=daily-question&envId=2025-09-27)

---

## Problem Understanding

You are given a set of points on a 2D plane.
The task is to **find the maximum area of a triangle** that can be formed by any three of these points.

---

## Approach

1. **Brute Force Enumeration**

    - The largest triangle can be formed by **any** combination of three distinct points.
    - There is no guaranteed shortcut to skip checking certain triplets because any three points could potentially yield the maximum area.

2. **Compute Area for Each Triplet**

    - For each combination of three points `(i, j, k)`, compute the triangle's area using the **shoelace formula** (determinant method):
      [
      \text{Area} = \frac{1}{2} \times |x_1(y_2-y_3) + x_2(y_3-y_1) + x_3(y_1-y_2)|
      ]
    - Keep track of the **maximum area** encountered.

3. **Why Not Faster?**

    - Although the maximum-area triangle must have its vertices on the convex hull,

        - Computing the hull is (O(n \log n)), and
        - Checking all triplets of hull points is (O(h^3)).

    - In the worst case, all points lie on the hull ((h = n)), so the complexity remains (O(n^3)).
    - Therefore, a true asymptotic improvement is not possible for the exact answer.

---

## Time Complexity

-   **Triple nested loop:** (O(n^3))
-   **Area calculation per triplet:** (O(1))
-   **Overall:** (O(n^3))

## Space Complexity

-   Only a few variables for coordinates and area calculations: **(O(1))**
