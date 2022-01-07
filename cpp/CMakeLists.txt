cmake_minimum_required(VERSION 3.16.3)

project(AoC)

add_executable(AoC main.cpp)

add_subdirectory(external/doctest)

add_subdirectory(Day20)

target_include_directories(AoC 
    PRIVATE Day20
    PUBLIC external/doctest/include
)

target_link_directories(AoC 
    PRIVATE Day20/
    PRIVATE external/doctest/src    
)

target_link_libraries(AoC day_20 doctest)
