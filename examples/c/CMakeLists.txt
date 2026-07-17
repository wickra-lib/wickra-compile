# Build and run the C and C++ examples against the wickra-compile C ABI.
#
#   cargo build --release -p compile-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_COMPILE_LIB_DIR defaults to the workspace target/release directory and
# can be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_compile_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_COMPILE_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-compile C ABI library")

set(WICKRA_COMPILE_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_COMPILE_LIB
    NAMES wickra_compile libwickra_compile
    HINTS "${WICKRA_COMPILE_LIB_DIR}"
    REQUIRED)

add_executable(compile compile.c)
target_include_directories(compile PRIVATE "${WICKRA_COMPILE_INCLUDE}")
target_link_libraries(compile PRIVATE "${WICKRA_COMPILE_LIB}")

add_executable(compilepp compile.cpp)
target_include_directories(compilepp PRIVATE "${WICKRA_COMPILE_INCLUDE}")
target_link_libraries(compilepp PRIVATE "${WICKRA_COMPILE_LIB}")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example compile compilepp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_COMPILE_LIB_DIR}/wickra_compile.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME compile COMMAND compile)
add_test(NAME compilepp COMMAND compilepp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(compile compilepp PROPERTIES TIMEOUT 60)
