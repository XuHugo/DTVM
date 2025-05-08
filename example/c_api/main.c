// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "zetaengine-c.h"
#include <assert.h>
#include <stdio.h>

static void env_print_i32(ZenInstanceRef Instance, int X) {
  printf("print_i32: %d\n", X);
}
static void env_print_i32_f32(ZenInstanceRef Instance, int X, float Y) {
  printf("print_i32_f32: %d %g\n", X, Y);
}
static void env_print_f64_f64(ZenInstanceRef Instance, double X, double Y) {
  printf("print_f64_f64: %g %g\n", X, Y);
}
static void env_print_f32(ZenInstanceRef Instance, float X) {
  printf("print_f32: %g\n", X);
}
static void env_print_f64(ZenInstanceRef Instance, double X) {
  printf("print_f64: %g\n", X);
}
static void env_print_str(ZenInstanceRef Instance, uint32_t Offset) {
  char *Str = ZenGetHostMemAddr(Instance, Offset);
  printf("print_str: %s", Str);
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("usage: %s <wasm file> <function name> <function args...>\n",
           argv[0]);
    return 1;
  }

  const char *Filename = argv[1];
  const char *FuncName = argv[2];
  const char **Args = (const char **)(argv + 3);
  char ErrBuf[128] = {0};
  const uint32_t ErrBufSize = sizeof(ErrBuf);
  uint32_t NumArgs = argc - 3;
  ZenValue Results[1] = {0};
  uint32_t NumResults;
  ZenRuntimeConfig RuntimeConfig = {
      .Mode = ZenModeSinglepass,
      .DisableWasmMemoryMap = false,
      .DisableWASI = true,
      .EnableStatistics = false,
      .EnableGdbTracingHook = false,
  };
  ZenRuntimeRef Runtime = NULL;
  ZenHostModuleDescRef HostModuleDesc = NULL;
  ZenHostModuleRef HostModule = NULL;
  ZenModuleRef Module = NULL;
  ZenIsolationRef Isolation = NULL;
  ZenInstanceRef Instance = NULL;

  ZenEnableLogging();

  Runtime = ZenCreateRuntime(&RuntimeConfig);
  if (!Runtime) {
    printf("failed to create runtime\n");
    goto failed;
  }

  ZenType ArgTypesI32[] = {ZenTypeI32};
  ZenType ArgTypesI32F32[] = {ZenTypeI32, ZenTypeF32};
  ZenType ArgTypesF64F64[] = {ZenTypeF64, ZenTypeF64};
  ZenType ArgTypesF32[] = {ZenTypeF32};
  ZenType ArgTypesF64[] = {ZenTypeF64};

  ZenHostFuncDesc HostFuncDescs[] = {
      {
          .Name = "print_i32",
          .NumArgs = 1,
          .ArgTypes = ArgTypesI32,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_i32,
      },
      {
          .Name = "print_i32_f32",
          .NumArgs = 2,
          .ArgTypes = ArgTypesI32F32,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_i32_f32,
      },
      {
          .Name = "print_f64_f64",
          .NumArgs = 2,
          .ArgTypes = ArgTypesF64F64,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_f64_f64,
      },
      {
          .Name = "print_f32",
          .NumArgs = 1,
          .ArgTypes = ArgTypesF32,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_f32,
      },
      {
          .Name = "print_f64",
          .NumArgs = 1,
          .ArgTypes = ArgTypesF64,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_f64,
      },
      {
          .Name = "print_str",
          .NumArgs = 1,
          .ArgTypes = ArgTypesI32,
          .NumReturns = 0,
          .RetTypes = NULL,
          .Ptr = (void *)env_print_str,
      },
  };

  HostModuleDesc = ZenCreateHostModuleDesc(Runtime, "env", HostFuncDescs, 6);
  if (!HostModuleDesc) {
    printf("failed to create host module desc\n");
    goto failed;
  }

  HostModule = ZenLoadHostModule(Runtime, HostModuleDesc);
  if (!HostModule) {
    printf("failed to load host module\n");
    goto failed;
  }

  Module = ZenLoadModuleFromFile(Runtime, Filename, ErrBuf, ErrBufSize);
  if (!Module) {
    printf("failed to load module: %s\n", ErrBuf);
    goto failed;
  }

  Isolation = ZenCreateIsolation(Runtime);
  if (!Isolation) {
    printf("failed to create isolation\n");
    goto failed;
  }

  Instance = ZenCreateInstance(Isolation, Module, ErrBuf, ErrBufSize);
  if (!Instance) {
    printf("failed to create instance: %s\n", ErrBuf);
    goto failed;
  }

  if (!ZenCallWasmFuncByName(Runtime, Instance, FuncName, Args, NumArgs,
                             Results, &NumResults)) {
    bool HasError = ZenGetInstanceError(Instance, ErrBuf, ErrBufSize);
    assert(HasError);
    printf("failed to call function: %s\n", ErrBuf);
    goto failed;
  }

  if (NumResults > 0) {
    switch (Results[0].Type) {
    case ZenTypeI32:
      printf("result: %d\n", Results[0].Value.I32);
      break;
    case ZenTypeI64:
      printf("result: %ld\n", Results[0].Value.I64);
      break;
    case ZenTypeF32:
      printf("result: %f\n", Results[0].Value.F32);
      break;
    case ZenTypeF64:
      printf("result: %lf\n", Results[0].Value.F64);
      break;
    default:
      printf("error: unexpected result!\n");
      break;
    }
  }

  return 0;

failed:
  if (Instance) {
    if (!ZenDeleteInstance(Isolation, Instance)) {
      printf("failed to delete instance\n");
    }
  }

  if (Isolation) {
    if (!ZenDeleteIsolation(Runtime, Isolation)) {
      printf("failed to delete isolation\n");
    }
  }

  if (Module) {
    if (!ZenDeleteModule(Runtime, Module)) {
      printf("failed to delete module\n");
    }
  }

  if (HostModule) {
    if (!ZenDeleteHostModule(Runtime, HostModule)) {
      printf("failed to delete host module\n");
    }
  }

  if (HostModuleDesc) {
    ZenDeleteHostModuleDesc(Runtime, HostModuleDesc);
  }

  if (Runtime) {
    ZenDeleteRuntime(Runtime);
  }

  return 1;
}