#include <iostream>
#include <vector>
#include <string>
#include "day_20.h"
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../external/doctest/doctest/doctest.h"

using namespace std;

int part_one()
{
    vector<string> msg{"Hello", "C++", "World", "from", "20"};

    for (const string &word : msg)
    {
        cout << word << " ";
    }
    cout << endl;
    return 1;
}

TEST_CASE("testing the factorial function")
{
    CHECK(part_one() == 1);
}
