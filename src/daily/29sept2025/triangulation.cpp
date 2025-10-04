#include <iostream>
#include <vector>
#include <cassert>
using namespace std;

class Solution
{
public:
    int minScoreTriangulation(vector<int> &values)
    {
        int n = values.size();
        vector<vector<int>> dp(n, vector<int>(n, 0));

        for (int i = n - 1; i >= 0; i--)
        {
            for (int j = i + 1; j < n; j++)
            {
                for (int k = i + 1; k < j; k++)
                {

                    // dp[i][j]=min(dp[i][j],left_score + triangle_score + right_score)
                    // left_score=dp[i][k],right_score=dp[k][j]
                    // triangle_score=values[i]×values[k]×values[j]

                    dp[i][j] = min(dp[i][j] == 0 ? INT_MAX : dp[i][j],
                                   dp[i][k] + values[i] * values[k] * values[j] + dp[k][j]);
                }
            }
        }
        return dp[0][n - 1];
    }
};

int main()
{
    Solution sol;

    vector<int> values = {3, 7, 4, 5};
    int test1 = sol.minScoreTriangulation(values);
    assert(test1 == 144);
    cout << "Test 1 passed !" << endl;

    vector<int> values2 = {1, 3, 1, 4};
    int test2 = sol.minScoreTriangulation(values2);
    assert(test2 == 7);
    cout << "Test 2 passed !" << endl;

    cout << "✅All Test cases passed !";

    return 0;
}