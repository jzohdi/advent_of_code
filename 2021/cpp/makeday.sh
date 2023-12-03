#! /bin/bash

die () {
    echo >&2 "$@"
    exit 1
}

PROJECT_NAME="AoC"
DAY_NUM=$1
DIR_NAME="Day$DAY_NUM"
COMMON_NAME="day_$DAY_NUM"
CMAKE="CMakeLists.txt"

[ "$#" -eq 1 ] || die "1 argument required, $# provided"

if [ -d "$DIR_NAME" ] 
then
    echo "Directory $DIR_NAME exists."
    exit 1
fi

echo "creating $DIR_NAME"
# read -p "Enter Day Number: " DAY_NUM

# echo "Hello $DAY_NUM, nice to meet you!"

mkdir -p -- "$DIR_NAME"
touch "$DIR_NAME/$COMMON_NAME.cpp"

echo "#include <iostream>
#include <vector>
#include <string>
#include \"$COMMON_NAME.h\"
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include \"../external/doctest/doctest/doctest.h\"

using namespace std;

int part_one()
{
    vector<string> msg{\"Hello\", \"C++\", \"World\", \"from\", \"$DAY_NUM\"};

    for (const string &word : msg)
    {
        cout << word << \" \";
    }
    cout << endl;
    return 1;
}

TEST_CASE(\"Testing day $DAY_NUM\")
{
    CHECK(part_one() == 1);
}" >> "$DIR_NAME/$COMMON_NAME.cpp"

touch "$DIR_NAME/$COMMON_NAME.h"

echo "int main(int, char **);" >> "$DIR_NAME/$COMMON_NAME.h"

touch "$DIR_NAME/$CMAKE"
echo "add_library($COMMON_NAME $COMMON_NAME.cpp)" >> "$DIR_NAME/$CMAKE"

touch "$DIR_NAME/input.txt"
touch "$DIR_NAME/test.txt"

eval "./setday.sh $PROJECT_NAME $DAY_NUM"

eval "./build.sh"