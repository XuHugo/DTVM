// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "platform/sgx/zen_sgx.h"
#include "platform/sgx/zen_sgx_dep.h"
#include "sgx_error.h"

int printf(const char *fmt, ...) {
  char buf[BUFSIZ] = {'\0'};
  va_list ap;
  va_start(ap, fmt);
  vsnprintf(buf, BUFSIZ, fmt, ap);
  va_end(ap);
  const char *msg = (const char *)buf;
  ocall_print_string(msg);
  return (int)strnlen(buf, BUFSIZ - 1) + 1;
}

int putchar(int c) { return 0; }

int sprintf(char *str, const char *fmt, ...) { return strlen(str); }

int fprintf(SGXFILE *stream, const char *fmt, ...) {
  char buf[BUFSIZ] = {'\0'};
  va_list ap;
  va_start(ap, fmt);
  vsnprintf(buf, BUFSIZ, fmt, ap);
  va_end(ap);
  const char *msg = (const char *)buf;
  ocall_print_string(msg);
  return (int)strnlen(buf, BUFSIZ - 1) + 1;
}

size_t malloc_usable_size(void *ptr) {
  if (ptr == NULL) {
    return 0;
  }
  size_t *sizePtr = (size_t *)ptr - 1;
  size_t blockSize = *sizePtr;
  return 0;
}

tm *localtime(const time_t *timep) { return nullptr; }

char *getenv(const char *getenv) { return ""; }

void _exit(int status) {}