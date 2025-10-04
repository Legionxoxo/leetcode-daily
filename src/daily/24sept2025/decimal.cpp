#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;

class Solution
{
public:
    string fractionToDecimal(int numerator, int denominator)
    {
        if (numerator == 0)
            return "0";

        unordered_map<long, int> mp; // to track reminder and its position in string
        string result;
        if ((numerator < 0) ^ (denominator < 0))
            result.push_back('-'); // if any of num or den negative, insert -

        long num = labs((long)numerator);
        long den = labs((long)denominator);

        result += to_string(num / den);
        long reminder = num % den;
        if (reminder == 0)
            return result;

        result.push_back('.');
        while (reminder != 0)
        {
            if (mp.count(reminder))
            {
                result.insert(mp[reminder], "(");
                result.push_back(')');
                return result;
            }
            mp[reminder] = result.size(); // Map a remainder → string index to know where the repeating cycle begins.
            reminder *= 10;
            result += to_string(reminder / den);
            reminder %= den;
        }
        return result;
    }
};

int main()
{
    int numerator = 4, denominator = 333;
    Solution sol;
    cout << sol.fractionToDecimal(numerator, denominator) << endl;
    return 0;
}