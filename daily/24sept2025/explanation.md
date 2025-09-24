---

## Solution: Fraction to Recurring Decimal

### **Approach**

1. **Handle zero numerator**

    - If the numerator is `0`, the fraction is `0`, so we immediately return `"0"`.

2. **Determine the sign**

    - If exactly one of the numerator or denominator is negative, the result is negative.
    - Use the XOR (`^`) operator to check this condition and prepend `'-'` if needed.

3. **Convert to positive long integers**

    - Convert numerator and denominator to `long` and take their absolute values.
    - This avoids integer overflow for edge cases like `INT_MIN`.

4. **Compute integer part**

    - Divide the numerator by the denominator to get the integer part.
    - Calculate the remainder using modulus (`%`).
    - If remainder is `0`, the fraction terminates, and we return the integer part as a string.

5. **Compute fractional part**

    - Append a decimal point `.` to the result string.
    - Use a hash map (`unordered_map`) to track **remainders and their corresponding positions in the result string**.
    - For each step:

        1. Multiply remainder by 10.
        2. Append the quotient of remainder/divisor to the result string.
        3. Update the remainder.
        4. If the remainder has appeared before, it means the decimal is repeating. Insert `(` at the first occurrence and append `)` at the end.

6. **Return result**

    - The final string represents the fraction as a decimal with repeating parts enclosed in parentheses.

---

### **Time Complexity**

-   **O(n)**, where `n` is the number of digits in the fractional part until it either terminates or starts repeating.
-   Each remainder is processed at most once, so the loop iterates at most `denominator` times.

### **Space Complexity**

-   **O(n)** for storing remainders in the hash map.
-   `n` is the number of unique remainders before a repeat occurs.

---
