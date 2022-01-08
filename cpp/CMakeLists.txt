cmake_minimum_required(VERSION 3.16.3)

project(AoC)

add_executable(AoC main.cpp)

add_subdirectory(external/doctest)

add_subdirectory(Day19)

target_include_directories(AoC 
    PRIVATE Day19
    PUBLIC external/doctest/include
)

target_link_directories(AoC 
    PRIVATE Day19/
    PRIVATE external/doctest/src    
)

target_link_libraries(AoC day_19 doctest)
