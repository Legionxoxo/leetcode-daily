---
# 💻 Solution – LeetCode 3508: Implement Router (C++)

This folder contains my **C++ solution** for the [3508. Implement Router](https://leetcode.com/problems/implement-router/) problem.
---

## 🔹 Solution Overview

We implemented the `Router` class using **three main data structures**:

1. **Queue (`std::queue`)** – Stores packets in **FIFO order** for forwarding.
2. **Set (`std::set`)** – Tracks all active packets to prevent **duplicates**.
3. **Map (`std::unordered_map<int, pair<int, vector<int>>>`)** – For each destination, keeps:

    - `first`: start index of forwarded packets
    - `second`: timestamps of packets, sorted by arrival

---

## 🔹 Function Implementations

**1️⃣ `addPacket(source, destination, timestamp)`**

-   Checks if the packet already exists in the set → returns `false` if duplicate.
-   Otherwise:

    -   Pushes packet to the queue
    -   Inserts packet into the set
    -   Appends timestamp to the map for the destination

-   If queue size exceeds `memoryLimit`:

    -   Removes the oldest packet from queue
    -   Updates the set and increments start index in the map for that destination

-   Returns `true` if added successfully

---

**2️⃣ `forwardPacket()`**

-   Removes the front packet from the queue (FIFO).
-   Deletes it from the set.
-   Marks it as forwarded in the map by incrementing the start index.
-   Returns `[source, destination, timestamp]` of the forwarded packet.
-   Returns empty if no packets remain.

---

**3️⃣ `getCount(destination, startTime, endTime)`**

-   Uses the destination’s timestamp vector starting from the current start index (ignoring forwarded packets).
-   Uses **binary search (`lower_bound` / `upper_bound`)** to count timestamps in `[startTime, endTime]`.
-   Returns the count.

---

## 🔹 Dry Run Example

**Memory Limit:** 3

| Operation               | Result / Notes                   |
| ----------------------- | -------------------------------- |
| `addPacket(1, 4, 90)`   | true                             |
| `addPacket(2, 5, 90)`   | true                             |
| `addPacket(1, 4, 90)`   | false (duplicate)                |
| `addPacket(3, 5, 95)`   | true                             |
| `addPacket(4, 5, 105)`  | true (oldest `[1,4,90]` removed) |
| `forwardPacket()`       | `[2,5,90]`                       |
| `addPacket(5, 2, 110)`  | true                             |
| `getCount(5, 100, 110)` | 1 (`[4,5,105]`)                  |

**Output Sequence:**

```
true, true, false, true, true, [2,5,90], true, 1
```

---

## ▶️ Compile & Run

```bash
g++ daily/20-09-2025/20-09-2025.cpp -o temp.exe
./temp.exe
```

---
