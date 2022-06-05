cmake_minimum_required(VERSION 3.22)
project(libsmalloc C)

set(CMAKE_C_STANDARD 11)

add_executable(libsmalloc_c_test malloc.c)
