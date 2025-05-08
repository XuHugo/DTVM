# Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)

project(ZetaEngineExample LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
  dtvmcore
  GIT_REPOSITORY <git path>
  GIT_TAG <git branch or tag>
  GIT_SHALLOW TRUE
)
set(ZEN_ENABLE_BUILTIN_WASI OFF)
FetchContent_MakeAvailable(dtvmcore)
include_directories(${dtvmcore_SOURCE_DIR}/src)

add_executable(zen_example main.c)
target_link_libraries(zen_example PRIVATE dtvmcore)