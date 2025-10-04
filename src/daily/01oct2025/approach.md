# Approach

**Problem Link:**  
[Water Bottles](https://leetcode.com/problems/water-bottles/description/)

## Idea / Intuition

We start with `numBottles` full bottles. After drinking, every empty bottle can be exchanged for a new full one if we have at least `numExchange` empty bottles.

The process:

1. Keep a running total of bottles consumed (`totalBottles`).
2. While we have enough empty bottles to exchange:
    - Calculate how many new bottles we get: `numBottles / numExchange`.
    - Add this count to `totalBottles`.
    - Update `numBottles` as:  
      new bottles + leftover bottles (`numBottles / numExchange + numBottles % numExchange`).
3. Once we cannot exchange anymore, stop.  
   The result is `totalBottles`.

---

## Time and Space Complexity

-   **Time Complexity:** O(log n)  
    Each iteration reduces the number of bottles (since you exchange), so the loop runs about `O(log numBottles)` times.
-   **Space Complexity:** O(1)  
    We only use a few integer variables.

---

## Test Cases

| Test Case | Input                              | Expected Output | Explanation                                                             |
| --------- | ---------------------------------- | --------------- | ----------------------------------------------------------------------- |
| 1         | `numBottles = 9, numExchange = 3`  | `13`            | Drink 9 → exchange 9/3=3 → drink 3 → exchange 1 → total = 13            |
| 2         | `numBottles = 15, numExchange = 4` | `19`            | Drink 15 → exchange 3 → drink 3 → exchange 0 (remainder 3) → total = 19 |
| 3         | `numBottles = 5, numExchange = 5`  | `6`             | Drink 5 → exchange 1 → drink 1 → total = 6                              |
| 4         | `numBottles = 2, numExchange = 3`  | `2`             | No exchange possible                                                    |
| 5         | `numBottles = 1, numExchange = 2`  | `1`             | Single bottle only                                                      |
