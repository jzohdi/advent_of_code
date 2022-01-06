cmake_minimum_required(VERSION 3.16.3)

project(AoC)

add_executable(AoC main.cpp)

add_subdirectory(Day18)

target_include_directories(AoC PRIVATE Day18)

target_link_directories(AoC PRIVATE Day18/)

target_link_libraries(AoC day_18)
