cmake_minimum_required(VERSION 3.27)
project(hyprshell
        DESCRIPTION "hyprshell for Hyprland"
        VERSION 1.0
)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_INCLUDE_CURRENT_DIR TRUE)
file(GLOB_RECURSE SRC "*.cpp")
configure_file(defs.in defs.hpp @ONLY)

add_library(hyprshell SHARED ${SRC})

find_package(PkgConfig REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET
        hyprland
        libdrm
        libinput
        libudev
        pangocairo
        pixman-1
        wayland-server
        xkbcommon
)
target_link_libraries(hyprshell PRIVATE rt PkgConfig::deps)

install(TARGETS hyprshell)
