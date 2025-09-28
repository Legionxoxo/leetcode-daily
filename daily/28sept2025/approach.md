Largest Perimeter Triangle
Problem Link: https://leetcode.com/problems/largest-perimeter-triangle/?envType=daily-question&envId=2025-09-28

Approach: Greedy with Sorting
The problem asks us to find the largest perimeter of a non-zero area triangle that can be formed using three side lengths from the input array nums.

Core Principle (Triangle Inequality Theorem)
For three lengths a,b,c to form a valid triangle, the sum of the lengths of any two sides must be greater than the length of the third side:

a+b>c

a+c>b

b+c>a

If we assume the sides are sorted such that a≥b≥c, the first two conditions (a+b>c and a+c>b) are always true (since a alone is ≥b and ≥c). Therefore, we only need to check the most restrictive condition: b+c>a (the sum of the two smaller sides must be greater than the largest side).

Algorithm Steps
Sort the Array: Sort the input array nums in descending order. This is the key greedy step. By sorting, we ensure that as we iterate, we are always considering the largest possible side lengths first.

Iterate and Check: Iterate through the sorted array starting from the first element (index i) up to the element at nums.size() - 3. In each iteration, consider a triplet of consecutive elements:

a=nums[i] (Largest side)

b=nums[i+1] (Second largest side)

c=nums[i+2] (Third largest side)

Find the Optimal Triangle: Check the triangle inequality: nums[i+1] + nums[i+2] > nums[i].

If the condition is met, we have found a valid triangle. Since we are iterating using the largest numbers first, this triplet must yield the largest possible perimeter. Immediately return the perimeter: nums[i]+nums[i+1]+nums[i+2].

No Triangle Found: If the loop completes without finding any such triplet, it means no valid triangle can be formed from the given lengths. Return 0.

Time and Space Complexity
Time Complexity: O(NlogN)

The dominant operation is the initial sorting of the array, which takes O(NlogN).

The subsequent loop runs at most N−2 times, which is O(N).

Total complexity is dominated by the sort: O(NlogN).

Space Complexity: O(1) or O(logN)

If an in-place sort is used (like in the provided C++ code), the space complexity is O(1) (excluding the input array storage) or O(logN) if recursive call stack space for quicksort/mergesort is counted.
