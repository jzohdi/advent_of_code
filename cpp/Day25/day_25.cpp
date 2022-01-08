#include <iostream>
#include <vector>
// #include <string>
#include "day_25.h"
// #include <map>
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../external/doctest/doctest/doctest.h"

using namespace std;

int part_one(string file_path)
{
    ifstream infile(file_path);
    string line;
    map<tuple<int, int>, int> map;
    int row = 0;
    int cols = 0;
    int rows = 0;
    int num_steps = 0;
    bool did_move = true;
    while (getline(infile, line))
    {
        if (cols == 0)
        {
            cols = line.size();
        }
        for (int i = 0; i < line.size(); i++)
        {
            if (line[i] == '>')
            {
                map[make_tuple(row, i)] = 1;
            }
            else if (line[i] == 'v')
            {
                map[make_tuple(row, i)] = 2;
            }
        }
        ++row;
    }
    rows = row;
    while (did_move)
    {
        did_move = false;
        if (move_many(&map, 1, rows - 1, cols - 1))
        {
            did_move = true;
        }
        if (move_many(&map, 2, rows - 1, cols - 1))
        {
            did_move = true;
        }
        num_steps++;
    }
    print_map(&map, rows - 1, cols - 1);
    return num_steps;
}
tuple<int, int> new_dest(int dir, tuple<int, int> curr, int max_row, int max_col)
{
    int row = get<0>(curr);
    int col = get<1>(curr);
    if (dir == 1)
    {
        col++;
    }
    else
    {
        row++;
    }
    if (row > max_row)
    {
        row = 0;
    }
    if (col > max_col)
    {
        col = 0;
    }
    return make_tuple(row, col);
}
bool move_many(map<tuple<int, int>, int> *map, int curr_dir, int max_row, int max_col)
{
    bool did_move = false;
    vector<tuple<int, int>> to_erase;
    vector<tuple<int, int>> to_move;
    for (const auto &pair : (*map))
    {
        int dir = pair.second;
        if (dir == curr_dir)
        {
            tuple<int, int> dest = new_dest(curr_dir, pair.first, max_row, max_col);
            int row = get<0>(dest);
            int col = get<1>(dest);
            tuple<int, int> new_key = make_tuple(row, col);
            if ((*map).find(new_key) == (*map).end())
            {
                to_erase.push_back(pair.first);
                to_move.push_back(new_key);
                did_move = true;
            }
        }
    }
    for (const auto &tup : to_move)
    {
        (*map)[tup] = curr_dir;
    }
    for (const auto &tup : to_erase)
    {
        (*map).erase(tup);
    }
    to_erase.clear();
    return did_move;
}

void print_map(map<tuple<int, int>, int> *map, int rows, int cols)
{
    for (int i = 0; i <= rows; i++)
    {
        for (int j = 0; j <= cols; j++)
        {
            tuple<int, int> key = make_tuple(i, j);
            if ((*map).find(key) == (*map).end())
            {
                cout << ".";
            }
            else
            {
                int value = (*map).at(key);
                if (value == 1)
                {
                    cout << '>';
                }
                else
                {
                    cout << 'v';
                }
            }
        }
        cout << endl;
    }
}

TEST_CASE("Testing day 25")
{
    CHECK(part_one("../Day25/test.txt") == 58);

    SUBCASE("Checking part one input.text")
    {
        int result_part_1 = part_one("../Day25/input.txt");
        REQUIRE(result_part_1 > 1);
        cout << "result part 1: " << result_part_1 << endl;
    }
}