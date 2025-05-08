// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_FILESYSTEM_H
#define ZEN_UTILS_FILESYSTEM_H

#if defined(__cpp_lib_filesystem)
#define ZEN_INCLUDE_STD_FS_EXPERIMENTAL 0
#elif defined(__cpp_lib_experimental_filesystem)
#define ZEN_INCLUDE_STD_FS_EXPERIMENTAL 1
#elif !defined(__has_include)
#define ZEN_INCLUDE_STD_FS_EXPERIMENTAL 1
#elif __has_include(<filesystem>)
#define ZEN_INCLUDE_STD_FS_EXPERIMENTAL 0
#elif __has_include(<experimental/filesystem>)
#define ZEN_INCLUDE_STD_FS_EXPERIMENTAL 1
#else
#error Could not find either system header "<filesystem>" or "<experimental/filesystem>"
#endif

#if ZEN_INCLUDE_STD_FS_EXPERIMENTAL
#include <experimental/filesystem>
namespace filesystem = std::experimental::filesystem;
#else
#include <filesystem>
namespace filesystem = std::filesystem;
#endif

#undef ZEN_INCLUDE_STD_FS_EXPERIMENTAL

#endif // ZEN_UTILS_FILESYSTEM_H
