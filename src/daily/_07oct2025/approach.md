Use a set<int> dryDays to store indices where rains[i] == 0 (dry days).

Use a unordered_map<int, int> fullLakes to store which day a lake was last filled.

Iterate over each day:

If it rains on a lake:

If the lake is already full → flood → return {}.

If it was full before, we must find a dry day after the last fill to empty it before raining again.

Use dryDays.lower_bound(fullLakes[lake]) to find such a day.

If found → assign that dry day to dry this lake.

If not → flood → return {}.

If it’s a dry day, just push its index into dryDays for potential use.

Fill the answer array:

-1 for rainy days.

The lake number for dry days that are used to prevent a flood.

Any arbitrary lake (e.g., 1) for unused dry days.
