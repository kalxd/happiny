cmake_minimum_required(VERSION 3.31)

project(happiny LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 COMPONENTS Widgets REQUIRED)

qt_standard_project_setup(REQUIRES 6.5)

# xgffi
include_directories(${CMAKE_SOURCE_DIR}/xgffi/target/cxxbridge/xgffi/src)

if (${CMAKE_BUILD_TYPE} STREQUAL "Release")
  set(LIB_XGFFI ${CMAKE_SOURCE_DIR}/xgffi/target/release/libxgffi.a)
else()
  set(LIB_XGFFI ${CMAKE_SOURCE_DIR}/xgffi/target/debug/libxgffi.a)
endif()

qt_add_executable(happiny
  app/main.cc
  # app/mainwindow.cc
)

target_link_libraries(
  happiny
  PRIVATE Qt6::Widgets
  PRIVATE ${LIB_XGFFI}
)
