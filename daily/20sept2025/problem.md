---
#📡 LeetCode 3508 – Implement Router

Design a data structure that can efficiently manage data packets in a network router.
---

## Problem Description

A router receives **data packets**, each with the following attributes:

-   **source** – Unique identifier of the machine that generated the packet.
-   **destination** – Unique identifier of the target machine.
-   **timestamp** – The time at which the packet arrived at the router.

You need to implement a `Router` class that supports the following operations:

### Class & Methods

| Method                                                         | Description                                                                                                                                                                                                                                |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Router(int memoryLimit)**                                    | Initializes the router with a fixed memory capacity. `memoryLimit` is the maximum number of packets the router can store at any time. If adding a new packet would exceed this limit, the **oldest packet** must be removed to free space. |
| **bool addPacket(int source, int destination, int timestamp)** | Adds a packet to the router. Returns `true` if successfully added, or `false` if a **duplicate** (same `source`, `destination`, and `timestamp`) already exists.                                                                           |
| **int\[] forwardPacket()**                                     | Forwards (and removes) the next packet in **FIFO** order and returns it as `[source, destination, timestamp]`. Returns an empty array if no packets are available.                                                                         |
| **int getCount(int destination, int startTime, int endTime)**  | Returns the number of packets currently stored (not yet forwarded) that have the given `destination` and whose `timestamp` lies in the inclusive range `[startTime, endTime]`.                                                             |

**Notes**

-   `addPacket` calls will be made in **increasing order of `timestamp`**.

---

## Example 1

```
Input:
["Router", "addPacket", "addPacket", "addPacket", "addPacket", "addPacket", "forwardPacket", "addPacket", "getCount"]
[[3], [1, 4, 90], [2, 5, 90], [1, 4, 90], [3, 5, 95], [4, 5, 105], [], [5, 2, 110], [5, 100, 110]]

Output:
[null, true, true, false, true, true, [2, 5, 90], true, 1]
```

**Explanation**

```
Router router = new Router(3);
router.addPacket(1, 4, 90);   // true
router.addPacket(2, 5, 90);   // true
router.addPacket(1, 4, 90);   // false (duplicate)
router.addPacket(3, 5, 95);   // true
router.addPacket(4, 5, 105);  // true (removes [1,4,90] as limit exceeded)
router.forwardPacket();       // [2, 5, 90]
router.addPacket(5, 2, 110);  // true
router.getCount(5, 100, 110); // 1
```

---

## Example 2

```
Input:
["Router", "addPacket", "forwardPacket", "forwardPacket"]
[[2], [7, 4, 90], [], []]

Output:
[null, true, [7, 4, 90], []]
```

---

## Constraints

-   `2 <= memoryLimit <= 10^5`
-   `1 <= source, destination <= 2 × 10^5`
-   `1 <= timestamp <= 10^9`
-   `1 <= startTime <= endTime <= 10^9`
-   At most `10^5` total calls to `addPacket`, `forwardPacket`, and `getCount`.
-   Calls to `addPacket` are made in **increasing order of timestamp**.

---
