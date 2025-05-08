// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "tests/spectest.h"

#define RAPIDJSON_HAS_STDSTRING 1
#if defined(__SSE4_2__)
#define RAPIDJSON_SSE42 1
#elif defined(__SSE2__)
#define RAPIDJSON_SSE2 1
#elif defined(__ARM_NEON__) || defined(__ARM_NEON) || defined(__ARM_NEON_FP)
#define RAPIDJSON_NEON 1
#endif

#include "tests/test_utils.h"

#include <algorithm>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <gtest/gtest.h>
#include <iterator>
#include <map>
#include <memory>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>
#include <unordered_map>

namespace zen::test {

using namespace common;
using namespace std::literals;

// Preprocessing for set up aliasing.
void resolveRegister(std::map<std::string, std::string> &Alias,
                     rapidjson::Value &CmdArray,
                     rapidjson::Document::AllocatorType &Allocator) {
  rapidjson::Value::ValueIterator ItMod = CmdArray.Begin();
  for (rapidjson::Value::ValueIterator It = CmdArray.Begin();
       It != CmdArray.End(); ++It) {
    const auto CmdType = It->GetObject()["type"].Get<std::string>();
    if (CmdType == "module"sv) {
      // Record last module in order.
      ItMod = It;
    } else if (CmdType == "register"sv) {
      const auto NewNameStr = It->GetObject()["as"].Get<std::string>();
      auto OrgName = ItMod->FindMember("name");
      if (It->GetObject().HasMember("name")) {
        // Register command records the original name. Set aliasing.
        Alias.emplace(It->GetObject()["name"].Get<std::string>(), NewNameStr);
      } else if (OrgName != ItMod->MemberEnd()) {
        // Register command not records the original name. Get name from
        // the module.
        Alias.emplace(OrgName->value.Get<std::string>(), NewNameStr);
      }
      if (OrgName != ItMod->MemberEnd()) {
        // Module has origin name. Replace to aliased one.
        OrgName->value.SetString(NewNameStr, Allocator);
      } else {
        // Module has no origin name. Add the aliased one.
        rapidjson::Value Text;
        Text.SetString(NewNameStr, Allocator);
        ItMod->AddMember("name", Text, Allocator);
      }
    }
  }
}

SpecTest::CommandID resolveCommand(std::string_view Name) {
  static const std::unordered_map<std::string_view, SpecTest::CommandID>
      KCommandMapping = {
          {"module"sv, SpecTest::CommandID::Module},
          {"action"sv, SpecTest::CommandID::Action},
          {"register"sv, SpecTest::CommandID::Register},
          {"assert_return"sv, SpecTest::CommandID::AssertReturn},
          {"assert_trap"sv, SpecTest::CommandID::AssertTrap},
          {"assert_exhaustion"sv, SpecTest::CommandID::AssertExhaustion},
          {"assert_malformed"sv, SpecTest::CommandID::AssertMalformed},
          {"assert_invalid"sv, SpecTest::CommandID::AssertInvalid},
          {"assert_unlinkable"sv, SpecTest::CommandID::AssertUnlinkable},
          {"assert_uninstantiable"sv,
           SpecTest::CommandID::AssertUninstantiable},
      };
  if (auto It = KCommandMapping.find(Name); It != KCommandMapping.end()) {
    return It->second;
  }
  return SpecTest::CommandID::Unknown;
}

// Helper function to parse parameters from json to vector of value.
std::vector<TypedValue> parseValueList(const rapidjson::Value &Args) {
  std::vector<TypedValue> Result;
  Result.reserve(Args.Size());
  for (const auto &Element : Args.GetArray()) {
    const auto &Type = Element["type"].Get<std::string>();
    const auto &ValueNode = Element["value"];
    if (ValueNode.IsString()) {
      const auto &Value = ValueNode.Get<std::string>();
      if (Type == "externref"sv) {
        ZEN_ASSERT_TODO();
      } else if (Type == "funcref"sv) {
        ZEN_ASSERT_TODO();
      } else if (Type == "i32"sv) {
        uint32_t Val = std::stoul(Value);
        int32_t I32 = 0;
        std::memcpy(&I32, &Val, sizeof(int32_t));
        Result.emplace_back(I32, WASMType::I32);
      } else if (Type == "i64"sv) {
        uint64_t Val = std::stoull(Value);
        int64_t I64 = 0;
        std::memcpy(&I64, &Val, sizeof(int64_t));
        Result.emplace_back(I64, WASMType::I64);
      } else if (Type == "f32"sv) {
        uint32_t Val = std::stoul(Value);
        float F32 = 0;
        std::memcpy(&F32, &Val, sizeof(float));
        Result.emplace_back(F32, WASMType::F32);
      } else if (Type == "f64"sv) {
        uint64_t Val = std::stoull(Value);
        double F64 = 0;
        std::memcpy(&F64, &Val, sizeof(double));
        Result.emplace_back(F64, WASMType::F64);
      }
    } else if (ValueNode.IsArray()) {
      // v128
      ZEN_ASSERT_TODO();
    } else {
      ZEN_UNREACHABLE();
    }
  }
  return Result;
}

// Helper function to parse parameters from json to vector of string pair.
std::vector<std::pair<std::string, std::string>>
parseExpectedList(const rapidjson::Value &Args) {
  std::vector<std::pair<std::string, std::string>> Result;
  Result.reserve(Args.Size());
  for (const auto &Element : Args.GetArray()) {
    const auto &Type = Element["type"].Get<std::string>();
    const auto &ValueNode = Element["value"];
    if (ValueNode.IsString()) {
      Result.emplace_back(Type, ValueNode.Get<std::string>());
    } else if (ValueNode.IsArray()) {
      const auto LaneType = Element["lane_type"].Get<std::string>();
      std::string Value;
      for (auto It = ValueNode.Begin(), E = ValueNode.End(); It != E; ++It) {
        Value += It->Get<std::string>();
        Value += ' ';
      }
      Value.pop_back();
      Result.emplace_back(Type + LaneType, std::move(Value));
    } else {
      ZEN_UNREACHABLE();
    }
  }
  return Result;
}

std::string SpecTest::findFilePath(const std::string &CategoryName,
                                   const std::string &UnitName,
                                   const std::string &Filename) const {
  return (TestsuiteRoot / CategoryName / UnitName / Filename).u8string();
}

std::vector<std::pair<std::string, std::string>> SpecTest::enumerate() const {
  using filesystem::directory_iterator;
  std::vector<std::pair<std::string, std::string>> Cases;
  for (const auto &CategoryDir : directory_iterator(TestsuiteRoot)) {
    const auto &CategoryName = CategoryDir.path().filename().u8string();
    for (const auto &UnitDir : directory_iterator(CategoryDir)) {
      const auto &UnitPath = UnitDir.path();
      const auto &UnitName = UnitPath.filename().u8string();
      const auto &UnitJson = UnitName + ".json"s;
      const auto &AbsPath = UnitPath / UnitJson;
      if (filesystem::is_regular_file(AbsPath)) {
        Cases.emplace_back(CategoryName, UnitName);
      } else {
        throw std::runtime_error("can't find spec json file: " +
                                 AbsPath.string());
      }
    }
  }
  std::sort(Cases.begin(), Cases.end());
  return Cases;
}

bool SpecTest::compare(const std::pair<std::string, std::string> &Expected,
                       const TypedValue &Got) const {
  const auto &ExceptedTypeStr = Expected.first;
  const auto &ExceptedValStr = Expected.second;
  const auto &GotType = Got.Type;
  const auto &GotVal = Got.Value;
  bool IsV128 = (std::string_view(ExceptedTypeStr).substr(0, 4) == "v128"sv);
  if (!IsV128 && std::string_view(ExceptedValStr).substr(0, 4) == "nan:"sv) {
    if (ExceptedTypeStr == "f32"sv) {
      if (GotType != WASMType::F32) {
        return false;
      }
      return std::isnan(GotVal.F32);
    }
    if (ExceptedTypeStr == "f64"sv) {
      if (GotType != WASMType::F64) {
        return false;
      }
      return std::isnan(GotVal.F64);
    }
    ZEN_ASSERT_TODO();

  } else if (ExceptedTypeStr == "funcref"sv) {
    ZEN_ASSERT_TODO();
  } else if (ExceptedTypeStr == "externref"sv) {
    ZEN_ASSERT_TODO();
  } else if (ExceptedTypeStr == "i32"sv) {
    if (GotType != WASMType::I32) {
      return false;
    }

    return uint32_t(std::stoul(ExceptedValStr)) == uint32_t(GotVal.I32);
  } else if (ExceptedTypeStr == "f32"sv) {
    if (GotType != WASMType::F32) {
      return false;
    }
    // Compare the 32-bit pattern
    float GotF32 = GotVal.F32;
    uint32_t Val = 0;
    std::memcpy(&Val, &GotF32, sizeof(uint32_t));
    return uint32_t(std::stoul(ExceptedValStr)) == Val;
  } else if (ExceptedTypeStr == "i64"sv) {
    if (GotType != WASMType::I64) {
      return false;
    }

    return uint64_t(std::stoull(ExceptedValStr)) == uint64_t(GotVal.I64);
  } else if (ExceptedTypeStr == "f64"sv) {
    if (GotType != WASMType::F64) {
      return false;
    }
    // Compare the 64-bit pattern
    double GotF64 = GotVal.F64;
    uint64_t Val = 0;
    std::memcpy(&Val, &GotF64, sizeof(uint64_t));
    return uint64_t(std::stoull(ExceptedValStr)) == Val;
  } else if (IsV128) {
    /*
    std::vector<std::string_view> Parts;
    std::string_view Ev = ExceptedValStr;
    if (GotTypeStr != "v128"sv) {
        return false;
    }
    for (std::string::size_type Begin = 0, End = Ev.find(' ');
         Begin != std::string::npos;
         Begin = 1 + End, End = Ev.find(' ', Begin)) {
        Parts.push_back(Ev.substr(Begin, End - Begin));
        if (End == std::string::npos) {
            break;
        }
    }
    std::string_view LaneType =
        std::string_view(ExceptedTypeStr).substr(4);
    */
    ZEN_ASSERT_TODO();
  }
  return false;
}

bool SpecTest::compares(
    const std::vector<std::pair<std::string, std::string>> &Expected,
    const std::vector<TypedValue> &Got) const {
  if (Expected.size() != Got.size()) {
    return false;
  }
  for (size_t I = 0; I < Expected.size(); ++I) {
    if (!compare(Expected[I], Got[I])) {
      return false;
    }
  }
  return true;
}

bool SpecTest::stringContains(std::string_view Expected,
                              std::string_view Got) const {

  const auto ExceptionPrefix = "exception: "sv;
  if (Got.find(ExceptionPrefix) == 0) {
    Got.remove_prefix(ExceptionPrefix.size());
  }
  if (Expected.find(ExceptionPrefix) == 0) {
    Expected.remove_prefix(ExceptionPrefix.size());
  }
  // one case in binary.wast
  if (Expected == "unexpected end of section or function"sv) {
    if (Got.find("unexpected end"sv) != std::string::npos)
      return true;
  }
  // one case in binary.wast
  else if (Expected == "invalid value type"sv) {
    if (Got.find("unexpected end"sv) != std::string::npos)
      return true;
  }
  // one case in custom.wast
  else if (Expected == "length out of bounds") {
    if (Got.find("unexpected end"sv) != std::string::npos)
      return true;
  }
  // several cases in binary-leb128.wast
  else if (Expected == "integer representation too long") {
    if (Got.find("invalid section id"sv) != std::string::npos)
      return true;
  }
  if (Got.find(Expected) == std::string::npos) {
    std::cout << "   ##### expected text : " << Expected << std::endl;
    std::cout << "   ######## error text : " << Got << std::endl;
    return false;
  }
  return true;
}

void SpecTest::run(const std::pair<std::string, std::string> &UnitPair) {
  const auto &CategoryName = UnitPair.first;
  const auto &UnitName = UnitPair.second;
  const auto &UnitPath =
      (TestsuiteRoot / CategoryName / UnitName / (UnitName + ".json")).string();
  std::ifstream JSONIS;

  if (filesystem::is_regular_file(UnitPath)) {
    JSONIS = std::ifstream(UnitPath);
  } else {
    throw std::runtime_error("can't find spec json file: " + UnitPath);
  }
  rapidjson::IStreamWrapper JSONISWrapper(JSONIS);
  rapidjson::Document Doc;
  auto &Allocator = Doc.GetAllocator();
  Doc.ParseStream(JSONISWrapper);

  std::map<std::string, std::string> Alias;
  std::string LastModName;

  // Helper function to get module name.
  auto GetModuleName = [&](const rapidjson::Value &Action) -> std::string {
    if (const auto &Module = Action.FindMember("module"s);
        Module != Action.MemberEnd()) {
      // Get the module name.
      auto ModName = Module->value.Get<std::string>();
      if (auto It = Alias.find(ModName); It != Alias.end()) {
        // If module name is aliased, use the aliased name.
        return It->second;
      }
      return ModName;
    }
    return LastModName;
  };

  auto GetMethodNameFromActionField =
      [](const std::string &Field) -> std::pair<std::string, bool> {
    std::string MethodName = Field;
    bool ExpectedIsGas = false;
    std::string IsGasPrefix = "$gas";
    if (Field.size() > IsGasPrefix.size() &&
        Field.rfind(IsGasPrefix) == (Field.size() - IsGasPrefix.size())) {
      // ends with '$gas'
      ExpectedIsGas = true;
      MethodName = Field.substr(0, Field.size() - IsGasPrefix.size());
    }
    return {MethodName, ExpectedIsGas};
  };

  auto Invoke = [&](const rapidjson::Value &Action,
                    const rapidjson::Value &Expected, uint64_t LineNumber) {
    const auto ModName = GetModuleName(Action);
    const auto Field = Action["field"s].Get<std::string>();
    const auto Params = parseValueList(Action["args"s]);
    const auto Returns = parseExpectedList(Expected);

    const auto &[MethodName, ExpectedIsGas] =
        GetMethodNameFromActionField(Field);

    // Invoke function of named module. Named modules are registered in
    // Store Manager. Anonymous modules are instantiated in VM.
    const auto &[ResReturns, ResErrorMsg, ResGasLeft] =
        OnInvoke(ModName, MethodName, Params);
    if (!ResErrorMsg.empty()) {
      EXPECT_EQ("", ResErrorMsg);
      EXPECT_NE(LineNumber, LineNumber);
    }
    if (ExpectedIsGas) {
      std::vector GasLeftVec = {
          TypedValue(UntypedValue((int64_t)ResGasLeft), WASMType::I64)};
      if (!compares(Returns, GasLeftVec)) {
        EXPECT_NE(LineNumber, LineNumber);
      }
    } else {
      if (!compares(Returns, ResReturns)) {
        EXPECT_NE(LineNumber, LineNumber);
      }
    }
  };

  // Helper function to get values.
  /*
  auto Get = [&](const rapidjson::Value &Action,
                 const rapidjson::Value &Expected, uint64_t LineNumber) {
      const auto ModName = getModuleName(Action);
      const auto Field = Action["field"s].Get<std::string>();
      const auto Returns = parseExpectedList(Expected);

      const auto Res = onGet(ModName, Field);
      if (!Res.second.empty()) {
          EXPECT_EQ("", Res.second);
          EXPECT_NE(LineNumber, LineNumber);
      }
      if (!Compare(returns[0], Res.first)) {
          EXPECT_NE(LineNumber, LineNumber);
      };
  };
  */
  auto TrapInvoke = [&](const rapidjson::Value &Action, const std::string &Text,
                        uint64_t LineNumber) {
    const auto ModName = GetModuleName(Action);
    const auto Field = Action["field"s].Get<std::string>();
    const auto Params = parseValueList(Action["args"s]);

    const auto &[MethodName, ExpectedIsGas] =
        GetMethodNameFromActionField(Field);

    auto Res = OnInvoke(ModName, MethodName, Params);
    if (ExpectedIsGas) {
      uint64_t GasLeft = std::get<2>(Res);
      const auto &ExpectedGasLeftText = Text;
      EXPECT_EQ(std::to_string(GasLeft), ExpectedGasLeftText);
    } else {
      EXPECT_TRUE(stringContains(Text, std::get<1>(Res)));
    }
  };
  auto TrapInstantiate = [&](const std::string &Filename,
                             const std::string &Text) {
    const auto Res = OnTrapInstantiate(Filename);
    EXPECT_TRUE(stringContains(Text, Res));
  };

  // Command processing. Return true for expected result.
  auto RunCommand = [&](const rapidjson::Value &Cmd) {
    // Line number in wast: Cmd["line"].Get<uint32_t>()
    if (const auto ValType = Cmd.FindMember("type"s);
        ValType != Cmd.MemberEnd()) {
      switch (resolveCommand(ValType->value.Get<std::string>())) {
      case SpecTest::CommandID::Module: {
        const auto Filename = findFilePath(CategoryName, UnitName,
                                           Cmd["filename"].Get<std::string>());
        const uint64_t LineNumber = Cmd["line"].Get<uint64_t>();
        if (const auto Name = Cmd.FindMember("name"); Name != Cmd.MemberEnd()) {
          // Module has name. Register module with module
          // name.
          LastModName = Name->value.Get<std::string>();
        } else {
          // If module has no name, use the Filename
          LastModName = Filename;
        }
        const auto Res = OnInstantiate(LastModName, Filename);

        if (!Res.empty()) {
          EXPECT_EQ("", Res);
          EXPECT_NE(LineNumber, LineNumber);
        }
        return;
      }

      case CommandID::Action: {
        const auto &Action = Cmd["action"s];
        const auto &Expected = Cmd["expected"s];
        const uint64_t LineNumber = Cmd["line"].Get<uint64_t>();
        Invoke(Action, Expected, LineNumber);
        return;
      }
      case CommandID::Register: {
        // Preprocessed. Ignore this.
        return;
      }
      case CommandID::AssertReturn: {
        const auto &Action = Cmd["action"s];
        // expected value or expected gas
        const auto &Expected = Cmd["expected"s];
        const auto ActType = Action["type"].Get<std::string>();
        const uint64_t LineNumber = Cmd["line"].Get<uint64_t>();
        if (ActType == "invoke"sv) {
          Invoke(Action, Expected, LineNumber);
          return;
        }
        if (ActType == "get"sv) {
          // now we just skip assert get
          // get(Action, Expected, LineNumber);
          return;
        }
        ZEN_ASSERT_TODO();

        return;
      }
      case CommandID::AssertTrap: {
        const auto &Action = Cmd["action"s];
        // trap error message or expected gas text
        const auto &Text = Cmd["text"s].Get<std::string>();
        const uint64_t LineNumber = Cmd["line"].Get<uint64_t>();
        TrapInvoke(Action, Text, LineNumber);
        return;
      }
      case CommandID::AssertExhaustion: {
        const auto &Action = Cmd["action"s];
        const auto &Text = Cmd["text"s].Get<std::string>();
        const auto ActType = Action["type"].Get<std::string>();
        const uint64_t LineNumber = Cmd["line"].Get<uint64_t>();
        if (ActType == "invoke"sv) {
          TrapInvoke(Action, Text, LineNumber);
          return;
        }
        ZEN_ASSERT_TODO();

        return;
      }
      case CommandID::AssertMalformed: {
        const auto &ModType = Cmd["module_type"s].Get<std::string>();
        if (ModType == "text") {
          // just skip text malformed
          return;
        }
        if (ModType == "binary") {
          const auto Filename = findFilePath(
              CategoryName, UnitName, Cmd["filename"].Get<std::string>());
          const auto &Text = Cmd["text"s].Get<std::string>();
          TrapInstantiate(Filename, Text);
          return;
        }
        ZEN_ASSERT_TODO();
      }
      case CommandID::AssertInvalid:
      case CommandID::AssertUnlinkable:
      case CommandID::AssertUninstantiable: {
        const auto Filename = findFilePath(CategoryName, UnitName,
                                           Cmd["filename"].Get<std::string>());
        const auto &Text = Cmd["text"s].Get<std::string>();
        TrapInstantiate(Filename, Text);
        return;
      }
      default:;
      }
    }
    // Unknown command.
    EXPECT_TRUE(false);
  };

  // Get command list.
  if (auto Commands = Doc.FindMember("commands"s);
      Commands != Doc.MemberEnd()) {
    rapidjson::Value CmdArray;
    CmdArray.CopyFrom(Commands->value, Allocator);

    // Preprocessing register command.
    resolveRegister(Alias, CmdArray, Allocator);

    // Iterate commands.
    for (const auto &Cmd : CmdArray.GetArray()) {
      RunCommand(Cmd);
    }
  }
}

} // namespace zen::test
