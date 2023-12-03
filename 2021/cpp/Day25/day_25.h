#include <string>
#include <map>

using namespace std;

int main(int, char **);
void print_map(map<tuple<int, int>, int> *map, int rows, int cols);
bool move_many(map<tuple<int, int>, int> *map, int curr_dir, int max_row, int max_col);