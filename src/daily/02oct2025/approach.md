# Approach

**Problem Link:**  
[Water Bottles II](https://leetcode.com/problems/water-bottles-ii/description/)

---

## Idea / Intuition

We start with `numBottles` full bottles. After drinking them, we can exchange empty bottles for new full bottles.  
The twist compared to **Water Bottles I** is that the number of bottles required for the next exchange **increases by 1 after each exchange**.

Process:

1. Keep a running total of bottles consumed (`totalBottles`).
2. While we have at least `numExchange` bottles:
    - Subtract `numExchange` bottles for the exchange.
    - Add 1 new full bottle (drink it, then it becomes an empty).
    - Increase `totalBottles` by 1.
    - Increase `numExchange` by 1, since the next exchange requires more bottles.
3. Stop when `numBottles < numExchange`.  
   The result is `totalBottles`.

---

## Time and Space Complexity

-   **Time Complexity:** O(k), where `k` is the number of exchanges possible.  
    In the worst case, this can be proportional to the initial `numBottles`.
-   **Space Complexity:** O(1)  
    We only use a few integer variables.

---

## Test Cases

| Test Case | Input                              | Expected Output | Explanation                                                                                                                |
| --------- | ---------------------------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------- |
| 1         | `numBottles = 13, numExchange = 6` | `15`            | Drink 13 → exchange 6 (get 1 new) → need 7 for next, have 8 → exchange 7 (get 1 new) → total = 15                          |
| 2         | `numBottles = 10, numExchange = 3` | `13`            | 10 → exchange 3 (get 1) → next need 4, have 8 → exchange 4 (get 1) → next need 5, have 5 → exchange 5 (get 1) → total = 13 |
| 3         | `numBottles = 5, numExchange = 5`  | `6`             | 5 → exchange 5 (get 1) → next need 6, have 1 → stop                                                                        |
| 4         | `numBottles = 2, numExchange = 3`  | `2`             | Not enough to exchange                                                                                                     |
| 5         | `numBottles = 1, numExchange = 2`  | `1`             | Single bottle only                                                                                                         |
