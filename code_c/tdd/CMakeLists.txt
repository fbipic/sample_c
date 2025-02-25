cmake_minimum_required(VERSION 3.10)

# Check for CppUTest and bail out if they don't have it
if(DEFINED ENV{CPPUTEST_HOME})
  message("Using CppUTest found in $ENV{CPPUTEST_HOME}")
else()
  message("CPPUTEST_HOME is not set; You must tell CMake where to find CppUTest")
  return()
endif()

# set the project name
project(tdd_hworld)

# include CppUTest headers
include_directories($ENV{CPPUTEST_HOME}/include)

# add cpputest library
add_library(lib_cpputest STATIC IMPORTED)
set_property(TARGET lib_cpputest PROPERTY
             IMPORTED_LOCATION $ENV{CPPUTEST_HOME}/cpputest_build/lib/libCppUTest.a)


# add the executable
add_executable(runtest ut_main.cpp ut_hworld.cpp)

target_link_libraries(runtest lib_cpputest)
