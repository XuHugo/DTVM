# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

file(GLOB_RECURSE SINGLEPASS_SRCS CONFIGURE_DEPENDS common/*.cpp singlepass.cpp)

if(ZEN_BUILD_TARGET_X86_64)
  file(GLOB_RECURSE SINGLEPASS_X64_SRCS CONFIGURE_DEPENDS x64/*.cpp)
  list(APPEND SINGLEPASS_SRCS ${SINGLEPASS_X64_SRCS})
elseif(ZEN_BUILD_TARGET_AARCH64)
  file(GLOB_RECURSE SINGLEPASS_A64_SRCS CONFIGURE_DEPENDS a64/*.cpp)
  list(APPEND SINGLEPASS_SRCS ${SINGLEPASS_A64_SRCS})
endif()

add_library(singlepass OBJECT ${SINGLEPASS_SRCS})

target_link_libraries(singlepass PRIVATE asmjit::asmjit)
