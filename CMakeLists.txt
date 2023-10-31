cmake_minimum_required(VERSION 3.26)
project(liborzklv C)

set(CMAKE_C_STANDARD 23)

add_library(liborzklv SHARED source/library.c
)
