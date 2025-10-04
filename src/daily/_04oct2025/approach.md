# Approach: Container With Most Water

[LeetCode problem page](https://leetcode.com/problems/container-with-most-water/description/?envType=daily-question&envId=2025-10-04).

## Problem Overview

The problem requires finding two lines from a given array of heights that, together with the x-axis, form a container capable of holding the most water. The area of water the container can hold is determined by the formula:

```
Area = min(height[i], height[j]) * (j - i)
```

Where:

-   `height[i]` and `height[j]` are the heights of the two lines.
-   `(j - i)` is the width between the two lines.

## Approach

### 1. Two-Pointer Technique

The optimal solution employs a two-pointer approach:

-   **Initialize two pointers**: Place one pointer at the beginning (`i = 0`) and the other at the end (`j = n - 1`) of the array.
-   **Calculate the area**: Compute the area formed by the lines at positions `i` and `j`.
-   **Update the maximum area**: Keep track of the maximum area encountered.
-   **Move the pointers**:

    -   If `height[i] < height[j]`, increment `i` to potentially find a taller line.
    -   Otherwise, decrement `j` to potentially find a taller line.

-   **Repeat** the process until the pointers meet.

This approach ensures that all possible pairs are considered, and the maximum area is found efficiently.

### 2. Time Complexity

The time complexity of this approach is O(n), where n is the number of elements in the input array. This is because each pointer moves at most `n` times, leading to a linear scan of the array.

### 3. Space Complexity

The space complexity is O(1), as the algorithm uses only a constant amount of extra space.

## Example

Given the array `height = [1, 8, 6, 2, 5, 4, 8, 3, 7]`, the two-pointer approach proceeds as follows:

-   **Initial pointers**: `i = 0`, `j = 8`
-   **First area calculation**: `min(1, 7) * (8 - 0) = 7`
-   **Move pointer**: Since `height[0] < height[8]`, increment `i` to 1.
-   **Subsequent calculations**: Continue this process, updating the pointers and calculating areas.
-   **Final result**: The maximum area found is 49.

## Conclusion

The two-pointer technique efficiently solves the "Container With Most Water" problem by systematically considering all possible pairs of lines and updating the maximum area found. This approach is optimal in terms of both time and space complexity.

---
