/*
 * Copyright (C) 2019 Intel Corporation.
 * Copyright (C) 2021-2023 the DTVM authors.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

    .text
    .align 2

#ifdef ZEN_BUILD_PLATFORM_DARWIN
    .globl _rollbackWasmVirtualStack
_rollbackWasmVirtualStack:
#else
    .globl rollbackWasmVirtualStack
    .type  rollbackWasmVirtualStack, @function
rollbackWasmVirtualStack:
#endif
    // virtualstack return address
    mov x29, sp
    // put ret address to old sp+8(x1 as old rsp)
    str x30, [x1]
    // put OldRsp(second param) to rsp
    mov x29, x1
    mov sp, x29
    // JmpBuf(third param) as return
    mov x0, x2
    ret

#ifdef ZEN_BUILD_PLATFORM_DARWIN
    .globl _startWasmFuncStack
_startWasmFuncStack:
#else
    .globl startWasmFuncStack
    .type  startWasmFuncStack, @function
startWasmFuncStack:
#endif
    sub sp, sp, #0x60
    // old return address
    mov x29, sp
    // put old ret address to old sp+08
    str x30, [x29, #0x08]

    // put old rsp to OldRspPtr(third param)
    str x29, [x2]

    // put NewRsp(second param) to rsp
    mov x29, x1
    mov sp, x29
    // StackInfo still first param(x0)
    // call InVirtualStackFuncPtr(x4)
    blr x4
    // not run again(InVirtualStackFuncPtr will call longjmp)
    add sp, sp, #0x60
    ret

#ifdef ZEN_BUILD_PLATFORM_DARWIN
    .globl _callNative
_callNative:
#else
    .globl callNative
    .type  callNative, %function
callNative:
#endif
    /* x0 - function ptr */
    /* x1 - argv */
    /* x2 - n_stacks */
    /* x3 - skip_instance_processing */

    sub sp, sp, #0x60
    stp x19, x20, [sp, #0x50]   /* save the registers */
    stp x21, x22, [sp, #0x40]
    stp x23, x24, [sp, #0x30]
    stp x25, x26, [sp, #0x20]
    stp x27, x28, [sp, #0x10]
    str x29, [sp]

    mov x19, x0     /* x19 = function ptr */
    mov x20, x1     /* x20 = argv */
    mov x21, x2     /* x21 = nstacks */
    mov x22, x3     /* x22 = skip_instance_processing */
    mov x29, sp     /* save the sp before call function */

    /* Fill in float-point registers */
    ld1  {v0.2D, v1.2D, v2.2D, v3.2D}, [x20], #64   /* v0 = argv[0], v1 = argv[1], v2 = argv[2], v3 = argv[3]*/
    ld1  {v4.2D, v5.2D, v6.2D, v7.2D}, [x20], #64   /* v4 = argv[4], v5 = argv[5], v6 = argv[6], v7 = argv[7]*/

    /* Fill inteter registers */
    ldp x0, x1, [x20], #16      /* x0 = argv[8] = exec_env, x1 = argv[9] */
    ldp x2, x3, [x20], #16      /* x2 = argv[10], x3 = argv[11] */
    ldp x4, x5, [x20], #16      /* x4 = argv[12], x5 = argv[13] */
    ldp x6, x7, [x20], #16      /* x6 = argv[14], x7 = argv[15] */

    /* Now x20 points to stack args */

    /* Directly call the function if no args in stack */
    cbz x21, prepare_instance

    /* Fill all stack args: reserve stack space and fill one by one */
    mov x23, sp
    bic sp,  x23, #15           /* Ensure stack is 16 bytes aligned */
    lsl x23, x21, #3            /* x23 = nstacks * 8 */
    add x23, x23, #15           /* x23 = (x23 + 15) & ~15 */
    bic x23, x23, #15
    sub sp, sp, x23             /* reserved stack space for stack arguments */
    mov x23, sp

loop_stack_args:                /* copy stack arguments to stack */
    cbz x21, prepare_instance
    ldr x24, [x20], #8
    str x24, [x23], #8
    sub x21, x21, #1
    b loop_stack_args

prepare_instance:
#ifdef ZEN_ENABLE_JIT
    /* skip instance-related processing if the instance doesn't exists */
    cbnz x22, call_function

#ifdef ZEN_ENABLE_SINGLEPASS_JIT
    /* prepare `instance` for singlepass JIT */
    mov x28, x0                 /* instance */
    ldr x27, [x28, #0x40]       /* offsetof(Instance, _global_var_data) == 0x40 */
    ldr x26, [x28, #0x50]       /* offsetof(Instance, _memories) == 0x50 */
    ldr x25, [x26, #0x08]       /* offsetof(Instance::MemoryInstance, _memory_size) == 0x08 */
    ldr x26, [x26, #0x10]       /* offsetof(Instance::MemoryInstance, _memory_base) == 0x10 */
#endif // ZEN_ENABLE_SINGLEPASS_JIT

    /* calculate stack boundary */
    ldr x21, [x0, #0x68]        /* offsetof(Instance, _jit_stack_size) == 0x68 */
    sub x21, sp, x21
    str x21, [x0, #0x70]        /* offsetof(Instance, _jit_stack_boundary) == 0x70 */
#endif // ZEN_ENABLE_JIT

call_function:
    mov x20, x30                /* save x30(lr) */
    blr x19
    mov sp, x29                 /* restore sp which is saved before calling fuction*/

return:
    mov x30,  x20               /* restore x30(lr) */
    ldp x19, x20, [sp, #0x50]   /* restore the registers in stack */
    ldp x21, x22, [sp, #0x40]
    ldp x23, x24, [sp, #0x30]
    ldp x25, x26, [sp, #0x20]
    ldp x27, x28, [sp, #0x10]
    ldr x29, [sp]
    add sp, sp, #0x60           /* restore sp */
    ret

/* mark end of the function */
#ifdef ZEN_BUILD_PLATFORM_DARWIN
    .globl _callNative_end
_callNative_end:
#else
    .globl callNative_end
callNative_end:
#endif
