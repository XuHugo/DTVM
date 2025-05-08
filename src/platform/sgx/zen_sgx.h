// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_SGX_H
#define ZEN_PLATFORM_SGX_H

#include "platform/sgx/zen_sgx_dep.h"
#include "platform/sgx/zen_sgx_file.h"
#include <ctime>
#include <stdio.h>
#include <string.h>
#include <time.h>

/* Determine the wordsize from the preprocessor defines.  */
#if defined __x86_64__ && !defined __ILP32__
#define __WORDSIZE 64
#else
#define __WORDSIZE 32
#endif

#ifdef __x86_64__
#define __WORDSIZE_TIME64_COMPAT32 1
/* Both x86-64 and x32 use the 64-bit system call interface.  */
#define __SYSCALL_WORDSIZE 64
#endif

#ifdef __cplusplus
extern "C" {
#endif

int printf(const char *fmt, ...);

int putchar(int c);

int sprintf(char *str, const char *fmt, ...);

int fprintf(SGXFILE *stream, const char *format, ...);

size_t malloc_usable_size(void *ptr);

struct tm *localtime(const time_t *timep);

char *getenv(const char *getenv);

void _exit(int status);

#ifdef __cplusplus
}
#endif
#endif