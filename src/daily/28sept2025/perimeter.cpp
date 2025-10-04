#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

class Solution
{
public:
    int largestPerimeter(vector<int> &nums)
    {
        sort(nums.begin(), nums.end());

        for (int i = 0; i < nums.size() - 2; i++)
        {
            if (nums[i + 1] + nums[i + 2] > nums[i])
                return nums[i] + nums[i + 1] + nums[i + 2];
        }
        return 0;
    }
};

int main()
{
    vector<int> nums = {1, 2, 1, 10};
    Solution sol;
    cout << "The perimeter of triangle is " << endl
         << sol.largestPerimeter(nums);

    return 0;
}