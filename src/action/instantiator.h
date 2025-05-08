// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_ACTION_INSTANTIATOR_H
#define ZEN_ACTION_INSTANTIATOR_H

namespace zen {

namespace runtime {
class Instance;
} // namespace runtime

namespace action {

class Instantiator {
  using Instance = runtime::Instance;

public:
  void instantiate(Instance &Inst);

private:
  void instantiateGlobals(Instance &Inst);
  void instantiateFunctions(Instance &Inst);
  void instantiateTables(Instance &Inst);
  void instantiateMemories(Instance &Inst);

  void initMemoryByDataSegments(Instance &Inst);

#ifdef ZEN_ENABLE_BUILTIN_WASI
  void instantiateWasi(Instance &Inst);
#endif
};

} // namespace action
} // namespace zen

#endif // ZEN_ACTION_INSTANTIATOR_H
