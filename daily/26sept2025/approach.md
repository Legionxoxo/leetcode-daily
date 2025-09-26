# 🔺 LeetCode – Valid Triangle Number

[Problem Link](https://leetcode.com/problems/valid-triangle-number/description/?envType=daily-question&envId=2025-09-26)

---

## Problem Summary

You are given an array of integers `nums` representing side lengths.
Return the number of **triplets** `(i, j, k)` such that these three numbers can form a **valid triangle**.

A triangle is valid if:

```

a + b > c

```

for all sides `a ≤ b ≤ c`.

---

## Approach

### 1️⃣ Sort the Array

Sort `nums` in **non-decreasing order**.
This ensures that if we pick any three numbers with indices `(left, right, i)` and `i` is the largest index,
`nums[i]` will always be the **largest side**.

### 2️⃣ Fix the Largest Side

Iterate from the **end of the array** (`i = n - 1` down to `2`), treating `nums[i]` as the largest side `c`.

### 3️⃣ Two-Pointer Search for Remaining Sides

For each fixed `i`:

-   Initialize two pointers:
    -   `left` at the start of the array (`0`)
    -   `right` just before `i` (`i - 1`)
-   While `left < right`:
    -   If `nums[left] + nums[right] > nums[i]`
        -   This means **all elements between `left` and `right-1`** with `nums[right]` will also satisfy the triangle condition (because the array is sorted).
        -   **Count all these pairs at once**: add `(right - left)` to the answer.
        -   Move `right` one step left to try a smaller second side.
    -   Else
        -   If the sum is too small, move `left` one step right to increase the smaller side.

### 4️⃣ Continue for All `i`

Repeat this process until all possible `i` values are checked.

---

## Time Complexity

-   Sorting the array: **O(n log n)**
-   Outer loop for each `i`: **O(n)**
-   Inner two-pointer traversal: **O(n)** (because each pointer moves at most `n` steps overall)

**Total:** `O(n^2)`
This is efficient enough for `n` up to a few thousand.

---

## Space Complexity

-   Sorting is done in-place.
-   Only a few extra variables are used.

**Total:** `O(1)` (ignoring sorting overhead).

```

```
