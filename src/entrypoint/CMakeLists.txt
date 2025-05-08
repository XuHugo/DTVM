# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

if(ZEN_BUILD_TARGET_X86_64)
  set(ENTRYPOINT_SRCS arch/callNative_x86_64.S entrypoint.cpp)
elseif(ZEN_BUILD_TARGET_AARCH64)
  set(ENTRYPOINT_SRCS arch/callNative_aarch64.S entrypoint.cpp)
endif()

add_library(entrypoint OBJECT ${ENTRYPOINT_SRCS})
