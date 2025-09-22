Here’s a **well-structured `explanation.md`** for your **Max Frequency Elements** C++ solution, following the same style and formatting as your Movie Renting System example:

---

# 💻 Solution – Max Frequency Elements (C++)

This folder contains my **C++ solution** for the **Max Frequency Elements** problem.

---

## 🔹 Problem Overview

Given an array `nums` of positive integers,
we need to return the **total number of elements** in `nums`
that have the **maximum frequency**.

-   **Frequency** of an element = number of times it appears.
-   The answer is the sum of frequencies of all elements that occur the most.

### Example

Input: `nums = [1,2,2,3,1,4]`

-   Frequencies: `{1 → 2, 2 → 2, 3 → 1, 4 → 1}`
-   Max frequency = `2`
-   Elements with max frequency = `{1, 2}`
-   Total = `2 + 2 = 4` ✅

---

## 🔹 Solution Overview

We implemented the `maxFrequencyElements` function using:

1. **Hash Map (`std::unordered_map<int,int>`)**

    - Key: element value
    - Value: current frequency of the element
    - Enables **O(1)** frequency updates and lookups.

2. **Running Counters (`maxFreq`, `total`)**

    - `maxFreq` tracks the highest frequency seen so far.
    - `total` stores the sum of frequencies of all elements that match `maxFreq`.

---

## 🔹 Function Implementation

**1️⃣ Count & Track in One Pass**

Instead of two loops (one for counting, one for summing),
we use **one pass** to do both:

-   Increment frequency of current element:

    ```cpp
    int freq = ++freqMap[num];
    ```

    (Pre-increment ensures `freq` holds the **updated** count.)

-   If `freq > maxFreq`:

    -   Update `maxFreq` to `freq`.
    -   Reset `total` to `freq` (new leader).

-   Else if `freq == maxFreq`:

    -   Add `freq` to `total` (another element reached the top).

This ensures we always maintain:

-   The **current maximum frequency**.
-   The **total count of all elements** having that maximum.

---

## 🔹 Dry Run Example

**Input:**
`nums = [1, 2, 2, 3, 1, 4]`

| Step | num | freqMap\[num] | maxFreq | total | Notes                        |
| ---: | --: | ------------: | ------: | ----: | ---------------------------- |
|    1 |   1 |             1 |       1 |     1 | New max (1), total = 1       |
|    2 |   2 |             1 |       1 |     2 | Matches max, total += 1      |
|    3 |   2 |             2 |       2 |     2 | New max (2), reset total = 2 |
|    4 |   3 |             1 |       2 |     2 | No change                    |
|    5 |   1 |             2 |       2 |     4 | Matches max, total += 2      |
|    6 |   4 |             1 |       2 |     4 | No change                    |

✅ **Answer = 4**

---

## 🔹 Complexity

| Metric    | Complexity                          |
| --------- | ----------------------------------- |
| **Time**  | `O(n)` – Single pass through `nums` |
| **Space** | `O(n)` – Hash map for frequencies   |

---

## ▶️ Compile & Run

```bash
g++ maxFreq.cpp -o maxFreq.exe
./maxFrequency.exe
```

---

This approach efficiently counts frequencies **and** computes the answer in **one pass** without extra loops.
