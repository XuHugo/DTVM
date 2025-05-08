// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "platform/sgx/zen_sgx_file.h"

SGXFILE *sgx_stdout;
SGXFILE *sgx_stderr;

int open(const char *pathname, int flags, ...) { return 0; }

int remove(const char *pathname) { return 0; }

int ftruncate(int fd, off_t length) { return 0; }

off_t lseek(int fd, off_t offset, int whence) { return 0; }

ssize_t sgx_write(int fd, const void *buf, size_t n) { return n; }

int close(int fd) { return 0; }

int fileno(SGXFILE *stream) { return 0; }

int isatty(int fd) { return 1; }

SGXFILE *fopen(const char *filename, const char *mode) { return NULL; }

int fclose(SGXFILE *stream) { return 0; }