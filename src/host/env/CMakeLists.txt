# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

if(ZEN_ENABLE_BUILTIN_LIBC)
  add_definitions(-DZEN_ENABLE_BUILTIN_LIBC)
endif()

if(ZEN_ENABLE_MOCK_CHAIN_TEST)
  add_definitions(-DZEN_ENABLE_MOCK_CHAIN_TEST)
endif()

if(ZEN_ENABLE_ASSEMBLYSCRIPT_TEST)
  add_definitions(-DZEN_ENABLE_ASSEMBLYSCRIPT_TEST)
endif()

add_library(host_env OBJECT env.cpp)
