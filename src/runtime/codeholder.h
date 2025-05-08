// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_CODEHOLDER_H
#define ZEN_RUNTIME_CODEHOLDER_H

#include "common/const_string_pool.h"
#include "common/defines.h"
#include "runtime/object.h"

namespace zen::runtime {

class CodeHolder : public RuntimeObject<CodeHolder> {
  friend class RuntimeObjectDestroyer;

public:
  enum class HolderKind { kFile, kRawData };

  static CodeHolderUniquePtr newFileCodeHolder(Runtime &RT,
                                               const std::string &Filename);

  static CodeHolderUniquePtr newRawDataCodeHolder(Runtime &RT, const void *Data,
                                                  size_t Size);

  HolderKind getKind() const { return Kind; }

  const void *getData() const { return Data; }

  size_t getSize() const { return Size; }

private:
  CodeHolder(Runtime &RT, HolderKind K)
      : RuntimeObject<CodeHolder>(RT), Kind(K) {}

  virtual ~CodeHolder();

  void releaseFileCodeHolder();

  void releaseRawDataCodeHolder();

  HolderKind Kind;

  const void *Data = nullptr;

  size_t Size = 0;
};

} // namespace zen::runtime

#endif // ZEN_RUNTIME_CODEHOLDER_H
