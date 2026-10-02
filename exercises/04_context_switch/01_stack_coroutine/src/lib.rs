//! # Stackful Coroutine and Context Switch (riscv64)
//! # 有栈协程与上下文切换（riscv64）
//!
//! In this exercise, you implement the minimal context switch using inline assembly,
//! 在本练习中，你将使用内联汇编实现最小化的上下文切换，
//! which is the core mechanism of OS thread scheduling. This crate is **riscv64 only**;
//! 这是操作系统线程调度的核心机制。本 crate 仅支持 **riscv64**；
//! run `cargo test` on riscv64 Linux, or use the repo's normal flow (`./check.sh` / `oscamp`) on x86 with QEMU.
//! 请在 riscv64 Linux 上运行 `cargo test`，或在 x86 上通过 QEMU 使用仓库的常规流程（`./check.sh` / `oscamp`）。
//!
//! ## Key Concepts
//! ## 核心概念
//! - **Callee-saved registers**: Save and restore them on switch so the switched-away task can resume correctly later.
//! - **被调用者保存寄存器**：切换时保存和恢复这些寄存器，以便被切出的任务之后能正确恢复执行。
//! - **Stack pointer `sp`** and **return address `ra`**: Restore them in the new context; the first time we switch to a task, `ret` jumps to `ra` (the entry point).
//! - **栈指针 `sp`** 和 **返回地址 `ra`**：在新上下文中恢复它们；首次切换到任务时，`ret` 会跳转到 `ra`（入口点）。
//! - Inline assembly: `core::arch::asm!`
//! - 内联汇编：`core::arch::asm!`
//!
//! ## riscv64 ABI (for this exercise)
//! ## riscv64 ABI（本练习使用）
//! - Callee-saved: `sp`, `ra`, `s0`–`s11`. The `ret` instruction is `jalr zero, 0(ra)`.
//! - 被调用者保存寄存器：`sp`、`ra`、`s0`–`s11`。`ret` 指令为 `jalr zero, 0(ra)`。
//! - First and second arguments: `a0` (old context), `a1` (new context).
//! - 第一个和第二个参数：`a0`（旧上下文）、`a1`（新上下文）。

#![cfg(target_arch = "riscv64")]

/// Saved register state for one task (riscv64). Layout must match the offsets used in the asm below: for one task (riscv64). Layout must match the offsets used in the asm below:
/// 单个任务保存的寄存器状态（riscv64）。布局必须与下方汇编使用的偏移匹配：单个任务的布局必须与下方汇编使用的偏移匹配：
/// `sp` at 0, `ra` at 8, then `s0`–`s11` at 16, 24, … 104.
/// `sp` 位于偏移 0，`ra` 位于偏移 8，随后是 `s0`–`s11`，偏移为 16、24、…、104。
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TaskContext {
    pub sp: u64,
    pub ra: u64,
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
}

impl TaskContext {
    pub const fn empty() -> Self {
        Self {
            sp: 0,
            ra: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }

    /// Initialize this context so that when we switch to it, execution starts at `entry`.
    /// 初始化此上下文，使切换到它时从 `entry` 开始执行。
    ///
    /// - Set `ra = entry` so that the first `ret` in the new context jumps to `entry`.
    /// - 设置 `ra = entry`，使新上下文中的首次 `ret` 跳转到 `entry`。
    /// - Set `sp = stack_top` with 16-byte alignment (RISC-V ABI requires 16-byte aligned stack at function entry).
    /// - 设置 `sp = stack_top`，并确保 16 字节对齐（RISC-V ABI 要求函数入口处栈指针 16 字节对齐）。
    /// - Leave `s0`–`s11` zero; they will be loaded on switch.
    /// - 将 `s0`–`s11` 保持为零；切换时会加载它们。
    pub fn init(&mut self, stack_top: usize, entry: usize) {
        todo!("set ra = entry, sp = stack_top (16-byte aligned)")
    }
}

/// Switch from `old` to `new` context: save current callee-saved regs into `old`, load from `new`, then `ret` (jumps to `new.ra`).
/// 从 `old` 切换到 `new` 上下文：将当前被调用者保存寄存器存入 `old`，从 `new` 加载，然后执行 `ret`（跳转至 `new.ra`）。
///
/// In asm: store `sp`, `ra`, `s0`–`s11` to `[a0]` (old), load from `[a1]` (new), zero `a0`/`a1` so we do not leak pointers into the new context, then `ret`.
/// 在汇编中：将 `sp`、`ra`、`s0`–`s11` 存入 `[a0]`（旧上下文），从 `[a1]`（新上下文）加载，将 `a0`/`a1` 置零以避免新上下文泄漏指针，然后执行 `ret`。
///
/// Must be `#[unsafe(naked)]` to prevent the compiler from generating a prologue/epilogue.
/// 必须使用 `#[unsafe(naked)]`，防止编译器生成序言/尾声。
pub unsafe fn switch_context(old: &mut TaskContext, new: &TaskContext) {
    todo!("save callee-saved regs to old, load from new, then ret; use #[unsafe(naked)] + naked_asm!, see module doc for riscv64 ABI and layout")
}

const STACK_SIZE: usize = 1024 * 64;

/// Allocate a stack for a coroutine. Returns `(buffer, stack_top)` where `stack_top` is the high address
/// 为协程分配栈。返回 `(buffer, stack_top)`，其中 `stack_top` 是高地址
/// (stack grows down). The buffer must be kept alive for the lifetime of the context using this stack.
/// （栈向低地址增长）。在使用此栈的上下文生命周期内，必须保持 buffer 存活。
pub fn alloc_stack() -> (Vec<u8>, usize) {
    todo!("allocate stack buffer, return (buffer, stack_top) with stack_top 16-byte aligned")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    extern "C" fn task_entry() {
        COUNTER.store(42, Ordering::SeqCst);
        loop {
            std::hint::spin_loop();
        }
    }

    #[test]
    fn test_alloc_stack() {
        let (buf, top) = alloc_stack();
        assert_eq!(top, buf.as_ptr() as usize + STACK_SIZE);
        assert!(top % 16 == 0);
    }

    #[test]
    fn test_context_init() {
        let (buf, top) = alloc_stack();
        let _ = buf;
        let mut ctx = TaskContext::empty();
        let entry = task_entry as *const () as usize;
        ctx.init(top, entry);
        assert_eq!(ctx.ra, entry as u64);
        assert!(ctx.sp != 0);
    }

    #[test]
    fn test_switch_to_task() {
        COUNTER.store(0, Ordering::SeqCst);

        static mut MAIN_CTX_PTR: *mut TaskContext = std::ptr::null_mut();
        static mut TASK_CTX_PTR: *mut TaskContext = std::ptr::null_mut();

        extern "C" fn cooperative_task() {
            COUNTER.store(99, Ordering::SeqCst);
            unsafe {
                switch_context(&mut *TASK_CTX_PTR, &*MAIN_CTX_PTR);
            }
        }

        let (_stack_buf, stack_top) = alloc_stack();
        let mut main_ctx = TaskContext::empty();
        let mut task_ctx = TaskContext::empty();
        task_ctx.init(stack_top, cooperative_task as *const () as usize);

        unsafe {
            MAIN_CTX_PTR = &mut main_ctx;
            TASK_CTX_PTR = &mut task_ctx;
            switch_context(&mut main_ctx, &task_ctx);
        }

        assert_eq!(COUNTER.load(Ordering::SeqCst), 99);
    }
}
