#include <iostream>
#include <vector>
#include <string>
#include "day_19.h"

using namespace std;

void run()
{
    vector<string> msg{"Hello", "C++", "World", "from", "19"};

    for (const string &word : msg)
    {
        cout << word << " ";
    }
    cout << endl;
}
