cmake_minimum_required(VERSION 3.10)
project(CppWebServer)

set(CMAKE_CXX_STANDARD 17)

add_subdirectory(deps/Crow)
add_subdirectory(deps/json)

add_executable(CppWebServer src/main.cpp)
target_link_libraries(CppWebServer PRIVATE Crow nlohmann_json::nlohmann_json)