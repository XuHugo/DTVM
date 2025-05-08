// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_HOST_WASI_WASI_H
#define ZEN_HOST_WASI_WASI_H

#include <string>
#include <vector>

#include "wni/helper.h"

struct fd_table;
struct fd_prestats;
struct argv_environ_values;

namespace zen::host {

#undef EXPORT_MODULE_NAME
#define EXPORT_MODULE_NAME wasi_snapshot_preview1

AUTO_GENERATED_FUNCS_DECL

struct WASIContext {
  struct ::fd_table *curfds;
  struct ::fd_prestats *prestats;
  struct ::argv_environ_values *argv_environ;
  char *argv_buf;
  char **argv_list;
  char *env_buf;
  char **env_list;
  VNMIEnv *vnmi_env; // temporally, will be moved into wni
};

} // namespace zen::host

#endif // ZEN_HOST_WASI_WASI_H
