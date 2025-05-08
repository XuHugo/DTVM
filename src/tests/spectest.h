// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_TESTS_SPECTEST_H
#define ZEN_TESTS_SPECTEST_H

#include "common/defines.h"
#include "common/enums.h"
#include "common/type.h"
#include "runtime/config.h"
#include "utils/filesystem.h"

#include <functional>
#include <string>
#include <string_view>
#include <tuple>
#include <utility>
#include <vector>

namespace zen::test {

class SpecTest {
  using TypedValue = common::TypedValue;
  using RunMode = common::RunMode;

public:
  enum class CommandID {
    Unknown,
    Module,
    Action,
    Register,
    AssertReturn,
    AssertTrap,
    AssertExhaustion,
    AssertMalformed,
    AssertInvalid,
    AssertUnlinkable,
    AssertUninstantiable,
  };

  explicit SpecTest(filesystem::path Root) : TestsuiteRoot(std::move(Root)) {}

  std::vector<std::pair<std::string, std::string>> enumerate() const;
  std::string findFilePath(const std::string &CategoryName,
                           const std::string &UnitName,
                           const std::string &Filename) const;
  bool compare(const std::pair<std::string, std::string> &Expected,
               const TypedValue &Got) const;
  bool
  compares(const std::vector<std::pair<std::string, std::string>> &Expected,
           const std::vector<TypedValue> &Got) const;
  bool stringContains(std::string_view Expected, std::string_view Got) const;

  void run(const std::pair<std::string, std::string> &UnitPair);

  using ModuleCallback = std::string(const std::string &ModName,
                                     const std::string &Filename);
  std::function<ModuleCallback> OnInstantiate;

  using InstantiateCallback = std::string(const std::string &Filename);
  std::function<InstantiateCallback> OnTrapInstantiate;

  using InvokeCallback =
      std::tuple<std::vector<TypedValue>, std::string, uint64_t>(
          const std::string &ModName, const std::string &Field,
          const std::vector<TypedValue> &Params);
  std::function<InvokeCallback> OnInvoke;

  const runtime::RuntimeConfig &getConfig() const { return Config; }
  void setConfig(const runtime::RuntimeConfig &Config) {
    this->Config = Config;
  }

private:
  filesystem::path TestsuiteRoot;
  runtime::RuntimeConfig Config;
};

} // namespace zen::test

#endif // ZEN_TESTS_SPECTEST_H
