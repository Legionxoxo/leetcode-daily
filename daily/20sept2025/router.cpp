#include <iostream>
#include <queue>
#include <set>
#include <unordered_map>
#include <algorithm>
using namespace std;

class Router
{
public:
    typedef pair<int, pair<int, int>> pip;

    queue<pip> q;
    set<pip> set;
    unordered_map<int, pair<int, vector<int>>> packetMap;
    int limit;

    Router(int memoryLimit)
    {
        limit = memoryLimit;
    }

    bool addPacket(int source, int destination, int timestamp)
    {
        if (set.find({source, {destination, timestamp}}) != set.end())
            return false;
        else
        {
            q.push({source, {destination, timestamp}});
            packetMap[destination].second.push_back(timestamp);
            set.insert({source, {destination, timestamp}});
            if (!q.empty())
            {
                pip temp = q.front();
                q.pop();
                packetMap[temp.second.first].first++;
                set.erase(temp);
            }
            return true;
        }
    }

    vector<int> forwardPacket()
    {
        vector<int> ans;
        if (!q.empty())
        {
            pip temp = q.front();
            q.pop();
            packetMap[temp.second.first].first++;
            ans.push_back(temp.first);
            ans.push_back(temp.second.first);
            ans.push_back(temp.second.second);
            set.erase(temp);
        }
        return ans;
    }

    int getCount(int destination, int startTime, int endTime)
    {
        int pos = packetMap[destination].first;
        return upper_bound(packetMap[destination].second.begin() + pos,
                           packetMap[destination].second.end(),
                           endTime) -
               lower_bound(packetMap[destination].second.begin() + pos,
                           packetMap[destination].second.end(),
                           startTime);
    }
};

int main()
{
    Router router(3);

    cout << boolalpha;
    cout << router.addPacket(1, 4, 90) << "\n";
    cout << router.addPacket(2, 5, 90) << "\n";
    cout << router.addPacket(1, 4, 90) << "\n";
    cout << router.addPacket(3, 5, 95) << "\n";
    cout << router.addPacket(4, 5, 105) << "\n";

    vector<int> pkt = router.forwardPacket();
    if (!pkt.empty())
    {
        cout << "[" << pkt[0] << ", " << pkt[1] << ", " << pkt[2] << "]\n";
    }

    cout << router.addPacket(5, 2, 110) << "\n";
    cout << router.getCount(5, 100, 110) << "\n";

    return 0;
}