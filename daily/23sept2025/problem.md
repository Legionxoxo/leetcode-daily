# 🔢 LeetCode 165 – Compare Version Numbers

[Problem Link](https://leetcode.com/problems/compare-version-numbers/)

## Compare two version strings and determine which one is **greater**, **smaller**, or if they are **equal**.

---

## Problem Description

You are given two version numbers, `version1` and `version2`, as strings.

-   Each version string consists of **numeric revisions** separated by dots (`.`).  
    For example: `"1.0.3"`, `"2.5"`, `"7"`.
-   Each revision is an **integer** and may contain **leading zeros**.
-   Missing revisions should be treated as `0`.

Return:

-   `-1` if `version1` is **less than** `version2`
-   `1` if `version1` is **greater than** `version2`
-   `0` if both versions are **equal**

---

### Constraints

-   `1 <= version1.length, version2.length <= 500`
-   `version1` and `version2` contain only digits and `.`.
-   All numeric values in each revision are **non-negative integers**.

---

## Function Signature

| Method                                                   | Description                                                                                    |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| **int compareVersion(string version1, string version2)** | Compares two version strings and returns **-1, 0, or 1** depending on their relative ordering. |

---

## Example 1

**Input**
version1 = "1.2"
version2 = "1.10"

**Process**

-   Split into revisions:
    -   version1 → `[1, 2]`
    -   version2 → `[1, 10]`
-   Compare step by step:
    -   `1 == 1` → continue
    -   `2 < 10` → return `-1`

**Output**
-1
