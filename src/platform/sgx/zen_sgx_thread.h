// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_SGX_THREAD_H
#define ZEN_PLATFORM_SGX_THREAD_H

class SgxMutex {};

class SgxSharedMutex {};

template <typename Mutex> class SgxSharedLock {
public:
  SgxSharedLock(Mutex Mtx) {}
  SgxSharedLock() {}
};

template <typename Mutex> class SgxUniqueLock {
public:
  SgxUniqueLock(Mutex Mtx) {}
  SgxUniqueLock() {}
};

template <typename Mutex> class SgxLockGuard {
public:
  SgxLockGuard(Mutex Mtx) {}
  SgxLockGuard() {}
};
#endif