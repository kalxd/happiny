cmake_minimum_required(VERSION 3.31)

project(happiny LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(PkgConfig)
pkg_check_modules(GTKMM gtkmm-4.0)

include_directories(${GTKMM_INCLUDE_DIRS})
link_directories(${GTKMM_LIBRARY_DIRS})

add_executable(happiny
  app/main.cc
  app/mainwindow.cc
)

target_link_libraries(
  happiny
  ${GTKMM_LIBRARIES}
  # ${CMAKE_SOURCE_DIR}/tl/target/debug/libtl.a
)
