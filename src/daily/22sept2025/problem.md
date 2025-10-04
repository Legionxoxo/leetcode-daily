---

# 🔢 LeetCode 3005 – Count Elements With Maximum Frequency

[Problem Link](https://leetcode.com/problems/count-elements-with-maximum-frequency/)

## Find the total count of elements that appear with the **highest frequency** in the array.

---

## Problem Description

You are given an array `nums` consisting of positive integers.

-   The **frequency** of an element is the number of times it occurs in the array.
-   Your task is to return the **total frequencies** of all elements that have the **maximum frequency**.

In other words, first find the maximum occurrence count of any number, and then sum the occurrences of all numbers that reach that maximum.

---

### Constraints

-   `1 <= nums.length <= 100`
-   `1 <= nums[i] <= 100`

---

## Function Signature

| Method                                          | Description                                                                                      |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **int maxFrequencyElements(vector<int>& nums)** | Returns the **sum of frequencies** of all elements whose frequency equals the maximum frequency. |

---

## Example 1

**Input**

```
nums = [1,2,2,3,1,4]
```

**Process**

-   Frequencies → `{1 → 2, 2 → 2, 3 → 1, 4 → 1}`
-   Max frequency = `2`
-   Elements with max frequency = `{1, 2}`
-   Total = `2 + 2 = 4`

**Output**

```
4
```

---

## Example 2

**Input**

```
nums = [1,2,3,4,5]
```

**Process**

-   Frequencies → `{1 → 1, 2 → 1, 3 → 1, 4 → 1, 5 → 1}`
-   Max frequency = `1`
-   Elements with max frequency = `{1, 2, 3, 4, 5}`
-   Total = `1 + 1 + 1 + 1 + 1 = 5`

**Output**

```
5
```

---

## Notes

-   The answer is **not** the count of elements with max frequency,
    but the **total number of occurrences** of those elements.
-   A single pass solution is possible by updating frequencies and tracking the current maximum on the fly.

---
