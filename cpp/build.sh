#! /bin/bash

eval "cmake -S . -B ./build" && cd "./build" && eval "make"