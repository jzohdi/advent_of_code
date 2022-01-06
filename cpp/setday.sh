PROJECT_NAME=$1
DAY_NUM=$2
DIR_NAME="Day$DAY_NUM"
COMMON_NAME="day_$DAY_NUM"
CMAKE="CMakeLists.txt"

echo "Setting Project: $PROJECT_NAME to day: $DAY_NUM. directory: $DIR_NAME"

sed -i 's/Day[0-9]*\/day_[0-9]*.h/'"$DIR_NAME\/$COMMON_NAME.h"'/g' main.cpp

rm -f "$CMAKE"

touch "$CMAKE"

echo "cmake_minimum_required(VERSION 3.16.3)

project($PROJECT_NAME)

add_executable($PROJECT_NAME main.cpp)

add_subdirectory($DIR_NAME)

target_include_directories($PROJECT_NAME PRIVATE $DIR_NAME)

target_link_directories($PROJECT_NAME PRIVATE $DIR_NAME/)

target_link_libraries($PROJECT_NAME $COMMON_NAME)" >> "$CMAKE"