# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

set(COMMON_SRCS const_string_pool.cpp errors.cpp traphandler.cpp)

add_library(common OBJECT ${COMMON_SRCS})
