// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_BACKTRACE_H
#define ZEN_UTILS_BACKTRACE_H

#include "common/defines.h"
#include <vector>

namespace zen::utils {

/**
 * create backtrace until the func[start,end) or the start_addr func(when to
 * start the call-chain)
 * @param FrameAddr the frame addr(bsp) the trap happened
 * @param PC the pc address(rip) the trap happened
 * @param StartFrameAddr the frame addr(bsp) when the wasm-call-chain
 * started(when start callWasmFunction)
 * @param IgnoredDepth count to ignore of the last backtrace frames
 * @param UntilFuncStart the func start to end the backtrace search e.g.
 * callNative
 * @param UntilFuncEnd the func end to end the backtrace search e.g.
 * callNative_end
 * @param JITCode
 * @param JITCodeEnd
 * @return
 */
std::vector<void *>
createBacktraceUntil(void *FrameAddr, void *PC, void *StartFrameAddr,
                     uint32_t IgnoredDepth, void *UntilFuncStart,
                     void *UntilFuncEnd, void *JITCode, void *JITCodeEnd);

inline void __attribute__((always_inline)) throwCpuIllegalInstructionTrap() {
#ifdef ZEN_ENABLE_CPU_EXCEPTION
#ifdef ZEN_BUILD_TARGET_X86_64
  asm(R"(ud2)");
#elif defined(ZEN_BUILD_TARGET_AARCH64)
  // 0x0000dead means udf #0xdead
  // not use udf because gcc before year-2020 not support the assembly udf
  asm(R"(.inst 0x0000dead)");
#endif // BUILD_TARGET_XXX
#endif // ZEN_ENABLE_CPU_EXCEPTION
}

} // namespace zen::utils

#endif // ZEN_UTILS_BACKTRACE_H
