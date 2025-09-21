# 🎬 LeetCode 1912 – Design Movie Rental System

[Problem Link](https://leetcode.com/problems/design-movie-rental-system/)

## Design a system to efficiently manage **renting and returning movies** across multiple shops.

## Problem Description

You are tasked with implementing a `MovieRentingSystem` class that manages movies at various shops. Each shop can have multiple movies, and each movie has a price. The system should allow searching for available movies, renting them, returning them, and reporting the cheapest rented movies.

Each movie is given as a 2D integer array `entries` where:
entries[i] = [shopi, moviei, pricei]

indicates that shop `shopi` has a copy of movie `moviei` with a rental price of `pricei`. Each shop carries **at most one copy** of a movie.

---

## Class & Methods

| Method                                         | Description                                                                                                                                                                       |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **MovieRentingSystem(int n, int[][] entries)** | Initializes the system with `n` shops and the movies in `entries`.                                                                                                                |
| **List<Integer> search(int movie)**            | Returns **up to 5 cheapest shops** that have an unrented copy of the given movie. Shops are sorted by **price → shop ID**. Returns empty list if no shop has the movie available. |
| **void rent(int shop, int movie)**             | Rents the given movie from the specified shop.                                                                                                                                    |
| **void drop(int shop, int movie)**             | Returns the previously rented movie to the shop.                                                                                                                                  |
| **List<List<Integer>> report()**               | Returns **up to 5 rented movies** as `[shop, movie]` pairs. Movies are sorted by **price → shop → movie**. Returns empty list if no movies are rented.                            |

**Notes**

-   Searching only returns movies that are currently available (not rented).
-   `report()` only includes movies that are currently rented.
-   If fewer than 5 results are found in `search()` or `report()`, return all results.
-   The test cases guarantee that `rent()` is called only if the shop has the movie available, and `drop()` is called only if the shop has the movie rented.

---

## Example 1

Input:
["MovieRentingSystem","search","rent","rent","report","drop","search"]
[[3, [[0,1,5],[0,2,6],[0,3,7],[1,1,4],[1,2,7],[2,1,5]]], [1], [0,1], [1,2], [], [1,2], [2]]

Output:
[null, [1,0,2], null, null, [[0,1],[1,2]], null, [0,1]]
