---
# 💻 Solution – Compare Version Numbers (C++)

This folder contains my **C++ solution** for the **Compare Version Numbers** problem.
---

## 🔹 Problem Overview

You are given two version strings `version1` and `version2`,
each consisting of **numeric revisions** separated by dots (`.`).
Compare them and return:

-   `-1` if `version1` < `version2`
-   `1` if `version1` > `version2`
-   `0` if both are equal

⚡ Each revision is compared as an **integer** (leading zeros are ignored).
Missing revisions are considered as **0**.

### Example

Input:
`version1 = "1.2"`
`version2 = "1.10"`

-   Compare 1 vs 1 → equal ✅
-   Compare 2 vs 10 → `2 < 10` → **Output = -1**

---

## 🔹 Solution Overview

We implemented the `compareVersion` function using:

1. **String Streams (`std::stringstream`)**

    - Splits each version string by `.` into numeric segments.
    - Reads revisions in order, just like processing tokens.

2. **Integer Parsing (`stoi`)**

    - Converts each revision to an integer to ignore leading zeros.

3. **Iterative Comparison**

    - Compare corresponding revisions one by one.
    - Missing parts are treated as `0`.

---

## 🔹 Function Implementation

**1️⃣ Tokenize & Compare**

-   Use `std::stringstream` to read each revision:

    ```cpp
    stringstream s1(version1), s2(version2);
    string v1, v2;
    ```

-   At every iteration:

    -   Extract next token (if no more, treat as empty).
    -   Convert to integer using `stoi`.
    -   Compare:

        -   If unequal → return `-1` or `1` immediately.
        -   If equal → continue.

-   Stop when **both streams** are exhausted.

---

## 🔹 Dry Run Example

**Input:**
`version1 = "1.2"`
`version2 = "1.10"`

| Step |  v1 |  v2 | num1 | num2 | Result                    |
| ---: | --: | --: | ---: | ---: | ------------------------- |
|    1 |   1 |   1 |    1 |    1 | Equal → continue          |
|    2 |   2 |  10 |    2 |   10 | 2 < 10 → **return -1** ✅ |

✅ **Answer = -1**

---

## 🔹 Complexity

| Metric    | Complexity                               |
| --------- | ---------------------------------------- |
| **Time**  | `O(max(n, m))` – n, m are string lengths |
| **Space** | `O(1)` – Constant extra space            |

---

## ▶️ Compile & Run

```bash
g++ compareVersion.cpp -o compareVersion.exe
./compareVersion.exe
```

---

This solution cleanly splits and compares versions revision-by-revision,
handling **leading zeros** and **unequal lengths** in a single pass.
