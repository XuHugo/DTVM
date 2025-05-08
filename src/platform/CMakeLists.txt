# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
set(PLATFORM_SRCS "")
if((CMAKE_SYSTEM_NAME STREQUAL "Linux" OR CMAKE_SYSTEM_NAME STREQUAL "Darwin")
   AND ZEN_ENABLE_SGX STREQUAL "OFF"
)
  set(PLATFORM_SRCS posix/map.cpp)
endif()

if(ZEN_ENABLE_SGX)
  set(PLATFORM_SRCS
      sgx/zen_sgx.cpp
      sgx/zen_sgx_map.cpp
      sgx/zen_sgx_file.cpp
      sgx/zen_sgx_string.cpp
      sgx/zen_sgx_time.cpp
      sgx/zen_sgx_mman.cpp
  )
endif()

if(PLATFORM_SRCS)
  add_library(platform OBJECT ${PLATFORM_SRCS})
endif()
