# 💻 Solution – Movie Renting System (C++)

This folder contains my **C++ solution** for the Movie Renting System problem.

---

## 🔹 Solution Overview

We implemented the `MovieRentingSystem` class using **three main data structures**:

1. **Map (`std::unordered_map<int, std::set<pair<int,int>>>`)** –  
    For each movie, stores a sorted set of `{price, shop}` pairs for **available movies**, so we can quickly find the cheapest shops.
   availMovies[1] = { {4,1}, {5,0}, {5,2} };
   // Cheapest shop for movie 1 is 1 (price 4), then 0 (price 5), then 2 (price 5)

2. **Set (`std::set<pair<pair<int,int>, int>>`)** –  
    Stores rented movies in the form `{{price, shop}, movie}`.  
    Automatically sorted by **price → shop → movie**, which allows easy retrieval of the **cheapest rented movies**.
   rentedMovies = { {{4,1},1}, {{5,0},1} };
   // Sorted automatically: first by price, then shop, then movie

3. **Nested Map (`std::unordered_map<int, std::unordered_map<int,int>>`)** –  
    Maps `{shop -> {movie -> price}}` for fast price lookup.
   prices[0][1] = 5; // Movie 1 at shop 0 costs 5
   prices[1][2] = 7; // Movie 2 at shop 1 costs 7

---

## 🔹 Function Implementations

**1️⃣ `search(movie)`**

-   Returns the **cheapest 5 shops** that have the movie available.
-   Iterates through the sorted set of available `{price, shop}` for that movie.
-   Stops after **5 results** or if fewer are available.

---

**2️⃣ `rent(shop, movie)`**

-   Finds the movie's price using `prices[shop][movie]`.
-   Removes `{price, shop}` from `availMovies[movie]`.
-   Inserts `{{price, shop}, movie}` into `rentedMovies`.

---

**3️⃣ `drop(shop, movie)`**

-   Finds the movie's price using `prices[shop][movie]`.
-   Removes `{{price, shop}, movie}` from `rentedMovies`.
-   Adds `{price, shop}` back to `availMovies[movie]`.

---

**4️⃣ `report()`**

-   Returns the **top 5 rented movies** as `{shop, movie}`.
-   Iterates over `rentedMovies` (sorted by price → shop → movie) and collects the first 5 entries.

---

## 🔹 Dry Run Example

**Available Movies:**

| Shop | Movie | Price |
| ---- | ----- | ----- |
| 0    | 1     | 5     |
| 0    | 2     | 6     |
| 1    | 1     | 4     |
| 1    | 2     | 7     |
| 2    | 1     | 5     |

**Operations / Results:**

| Operation   | Result / Notes                           |
| ----------- | ---------------------------------------- |
| `search(1)` | `[1, 0, 2]` (cheapest shops for movie 1) |
| `rent(1,1)` | Movie 1 rented from shop 1               |
| `report()`  | `[1,1]` (shop 1, movie 1)                |
| `drop(1,1)` | Movie 1 returned to shop 1               |
| `report()`  | `[]` (no rented movies)                  |

---

## ▶️ Compile & Run

```bash
g++ daily/21sept2025/movie.cpp -o movie.exe
./movie.exe
```
