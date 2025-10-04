#include <iostream>
#include <string>
#include <sstream>
using namespace std;

class Solution
{
public:
    int compareVersion(string version1, string version2)
    {
        stringstream s1(version1), s2(version2);
        string v1, v2;

        while (true)
        {

            if (!getline(s1, v1, '.'))
                v1 = "";
            if (!getline(s2, v2, '.'))
                v2 = "";

            if (v1.empty() && v2.empty())
                break;

            int num1 = v1.empty() ? 0 : stoi(v1);
            int num2 = v2.empty() ? 0 : stoi(v2);

            if (num1 < num2)
                return -1;
            if (num1 > num2)
                return 1;
        }
        return 0;
    }
};

int main()
{
    string version1 = "1.2";
    string version2 = "1.10";

    Solution sol;
    int res = sol.compareVersion(version1, version2);

    // Use std::boolalpha to print true/false instead of 1/0
    cout << boolalpha;
    cout << "version1 < version2 ? " << (res < 0) << endl;
    cout << "version1 > version2 ? " << (res > 0) << endl;
    cout << "version1 == version2 ? " << (res == 0) << endl;

    return 0;
}