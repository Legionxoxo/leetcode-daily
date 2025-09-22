#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

class Solution
{
public:
    int maxFrequencyElements(vector<int> &nums)
    {
        unordered_map<int, int> freqMap;
        int maxFreq = 0;
        int total = 0;

        for (const auto &num : nums)
        {
            int freq = ++freqMap[num]; // pre increment to get new value

            if (freq > maxFreq)
            {
                maxFreq = freq;
                total = freq;
            }
            else if (freq == maxFreq)
            {
                total += freq;
            }
        }
        return total;
    }
};

int main()
{
    vector<int> nums = {1, 2, 3, 1, 4};
    Solution sol;
    int result = sol.maxFrequencyElements(nums);

    cout << result << endl; // result should be 2
    return 0;
}