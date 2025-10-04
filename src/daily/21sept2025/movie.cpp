#include <iostream>
#include <unordered_map>
#include <set>
#include <vector>
using namespace std;

class MovieRentingSystem
{
    unordered_map<int, set<pair<int, int>>> availMovies; // movie -> sorted set of {price, shop}
    set<pair<pair<int, int>, int>> rentedMovies;         // sorted set of {{price, shop}, movie}
    unordered_map<int, unordered_map<int, int>> prices;  // shop -> (movie -> price)

public:
    MovieRentingSystem(int n, vector<vector<int>> &entries)
    {
        for (const auto &entry : entries)
        {
            int shop = entry[0];
            int movie = entry[1];
            int price = entry[2];

            prices[shop][movie] = price;
            availMovies[movie].insert({price, shop});
        }
    }

    vector<int> search(int movie)
    {
        vector<int> result;
        if (availMovies.count(movie))
        {
            int cnt = 0;
            for (const auto &movie : availMovies[movie])
            {
                if (cnt >= 5)
                    break;
                result.push_back(movie.second); // shop
                cnt++;
            }
        }
        return result;
    }

    void rent(int shop, int movie)
    {
        int price = prices[shop][movie];
        availMovies[movie].erase({price, shop});
        rentedMovies.insert({{price, shop}, movie});
    }

    void drop(int shop, int movie)
    {
        int price = prices[shop][movie];
        rentedMovies.erase({{price, shop}, movie});
        availMovies[movie].insert({price, shop});
    }

    vector<vector<int>> report()
    {
        vector<vector<int>> result;
        int cnt = 0;
        for (const auto &rented : rentedMovies)
        {
            if (cnt >= 5)
                break;

            const auto &priceShop = rented.first; //{price, shop}
            int shop = priceShop.second;
            int movie = rented.second;

            result.push_back({shop, movie});
            cnt++;
        }
        return result;
    }
};

int main()
{
    // Sample input data:
    vector<vector<int>> entries = {
        {0, 1, 5},
        {0, 2, 6},
        {0, 3, 7},
        {1, 1, 4},
        {1, 2, 7},
        {2, 1, 5}};

    MovieRentingSystem system(3, entries);

    // 1️⃣ Search movie 1 (cheapest shops first)
    cout << "Search movie 1:\n";
    for (int shop : system.search(1))
        cout << shop << " ";
    cout << "\n";

    // 2️⃣ Rent movie 1 from shop 1
    cout << "Rent movie 1 from shop 1\n";
    system.rent(1, 1);

    // 3️⃣ Report rented movies
    cout << "Report after renting:\n";
    for (auto &r : system.report())
        cout << "Shop " << r[0] << " Movie " << r[1] << "\n";

    // 4️⃣ Drop movie 1 back to shop 1
    cout << "Drop movie 1 back to shop 1\n";
    system.drop(1, 1);

    // 5️⃣ Report again
    cout << "Report after dropping:\n";
    for (auto &r : system.report())
        cout << "Shop " << r[0] << " Movie " << r[1] << "\n";
    return 0;
}