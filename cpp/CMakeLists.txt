cmake_minimum_required(VERSION 3.16.3)

project(AoC)

add_executable(AoC main.cpp)

add_subdirectory(external/doctest)

add_subdirectory(Day21)

target_include_directories(AoC 
    PRIVATE Day21
    PUBLIC external/doctest/include
)

target_link_directories(AoC 
    PRIVATE Day21/
    PRIVATE external/doctest/src    
)

target_link_libraries(AoC day_21 doctest)
