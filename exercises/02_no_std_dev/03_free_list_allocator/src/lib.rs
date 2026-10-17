//! # Free-List Allocator
//! # 空闲链表分配器
//!
//! Building on the bump allocator, implement a Free-List Allocator that supports memory reclamation.
//! 基于 bump 分配器，实现一个支持内存回收的空闲链表分配器。
//!
//! ## How It Works
//! ## 工作原理
//!
//! A Free-List Allocator uses a linked list to track all freed memory blocks.
//! 空闲链表分配器使用链表跟踪所有已释放的内存块。
//! On allocation, it first searches the list for a suitable block (first-fit strategy);
//! 分配时，它首先搜索链表以查找合适的块（首次适配策略）；
//! if none is found, it falls back to allocating from the unused region.
//! 如果找不到，则回退到未使用的区域进行分配。
//! On deallocation, the block is inserted at the head of the list.
//! 释放时，将该块插入链表头部。
//!
//! ```text
//! free_list -> [block A: 64B] -> [block B: 128B] -> [block C: 32B] -> null
//! ```
//!
//! Each free block stores a `FreeBlock` struct at its head (containing block size and next pointer).
//! 每个空闲块在其开头存储一个 `FreeBlock` 结构体（包含块大小和下一个指针）。
//!
//! ## Task
//! ## 任务
//!
//! Implement `FreeListAllocator`'s `alloc` and `dealloc` methods:
//! 实现 `FreeListAllocator` 的 `alloc` 和 `dealloc` 方法：
//!
//! ### alloc
//! ### alloc（分配）
//! 1. Traverse the free_list, find the first block with `size >= layout.size()` and proper alignment (first-fit)
//! 1. 遍历 free_list，找到第一个满足 `size >= layout.size()` 且对齐合适的块（首次适配）
//! 2. If found, remove it from the list and return it
//! 2. 如果找到，从链表中移除并返回它
//! 3. If not found, allocate from the `bump` region (same as bump allocator)
//! 3. 如果未找到，则从 `bump` 区域分配（与 bump 分配器相同）
//!
//! ### dealloc
//! ### dealloc（释放）
//! 1. Write `FreeBlock` header info at the freed block
//! 1. 在已释放块处写入 `FreeBlock` 头部信息
//! 2. Insert it at the head of free_list
//! 2. 将它插入 free_list 头部
//!
//! ## Key Concepts
//! ## 关键概念
//!
//! - Intrusive linked list
//! - 内嵌式链表
//! - `*mut T` read/write: `ptr.write(val)` / `ptr.read()`
//! - `*mut T` 读写：`ptr.write(val)` / `ptr.read()`
//! - Memory alignment checks
//! - 内存对齐检查

#![cfg_attr(not(test), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::Ordering;

/// Free block header, stored at the beginning of each free memory block
/// 空闲块头部，存储在每个空闲内存块的开头
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

pub struct FreeListAllocator {
    #[allow(dead_code)]
    heap_start: usize,
    heap_end: usize,
    /// Bump pointer: unallocated region starts here
    /// Bump 指针：未分配区域从此处开始
    bump_next: core::sync::atomic::AtomicUsize,
    /// Free list head (protected by Mutex in test, UnsafeCell otherwise)
    /// 空闲链表头部（测试中由 Mutex 保护，其他情况下使用 UnsafeCell）
    #[cfg(test)]
    free_list: std::sync::Mutex<*mut FreeBlock>,
    #[cfg(not(test))]
    free_list: core::cell::UnsafeCell<*mut FreeBlock>,
}

#[cfg(test)]
unsafe impl Send for FreeListAllocator {}
#[cfg(test)]
unsafe impl Sync for FreeListAllocator {}
#[cfg(not(test))]
unsafe impl Send for FreeListAllocator {}
#[cfg(not(test))]
unsafe impl Sync for FreeListAllocator {}

impl FreeListAllocator {
    /// # Safety
    /// # 安全性
    /// `heap_start..heap_end` must be a valid readable and writable memory region.
    /// `heap_start..heap_end` 必须是有效的可读写内存区域。
    pub unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self {
            heap_start,
            heap_end,
            bump_next: core::sync::atomic::AtomicUsize::new(heap_start),
            #[cfg(test)]
            free_list: std::sync::Mutex::new(null_mut()),
            #[cfg(not(test))]
            free_list: core::cell::UnsafeCell::new(null_mut()),
        }
    }

    #[cfg(test)]
    fn free_list_head(&self) -> *mut FreeBlock {
        *self.free_list.lock().unwrap()
    }

    #[cfg(test)]
    fn set_free_list_head(&self, head: *mut FreeBlock) {
        *self.free_list.lock().unwrap() = head;
    }

    #[cfg(not(test))]
    fn free_list_head(&self) -> *mut FreeBlock {
        unsafe { *self.free_list.get() }
    }

    #[cfg(not(test))]
    fn set_free_list_head(&self, head: *mut FreeBlock) {
        unsafe { *self.free_list.get() = head }
    }
}

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ensure block is at least large enough to hold a FreeBlock header (for future dealloc)
        // 确保块至少足以容纳一个 FreeBlock 头部（供将来 dealloc 使用）
        let size = layout.size().max(core::mem::size_of::<FreeBlock>());
        let align = layout.align().max(core::mem::align_of::<FreeBlock>());

        // TODO: Step 1 — traverse free_list, find a suitable block (first-fit)
        // TODO: 步骤 1 — 遍历 free_list，找到合适的块（首次适配）
        //
        // Hints:
        // 提示：
        // - Use prev_ptr and curr to traverse the list
        // - 使用 prev_ptr 和 curr 遍历链表
        // - Check if curr address satisfies align, and (*curr).size >= size
        // - 检查 curr 地址是否满足 align，且 (*curr).size >= size
        // - If found, remove it from the list (update prev's next or the free_list head)
        // - 如果找到，从链表中移除它（更新 prev 的 next 或 free_list 头部）
        // - Return curr as *mut u8
        // - 将 curr 作为 *mut u8 返回
        let mut prev: *mut FreeBlock = null_mut();
        let mut curr = self.free_list_head();

        while !curr.is_null() {
            let curr_addr = curr as usize;
            if curr_addr % align == 0 && (*curr).size >= size {
                let next = (*curr).next;
                if prev.is_null() {
                    self.set_free_list_head(next);
                } else {
                    (*prev).next = next;
                }
                return curr as *mut u8;
            }
            prev = curr;
            curr = (*curr).next;
        }
        // TODO: Step 2 — no suitable block in free_list, allocate from bump region
        // TODO: 步骤 2 — free_list 中没有合适的块，从 bump 区域分配
        //
        // Same logic as 02_bump_allocator's alloc
        // 与 02_bump_allocator 的 alloc 使用相同逻辑
        let mut current = self.bump_next.load(Ordering::SeqCst);
        loop {
            let aligned = (current + align - 1) & !(align - 1);
            let end = aligned + size;

            if end > self.heap_end {
                return null_mut();
            }

            match self.bump_next.compare_exchange_weak(
                current,
                end,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(core::mem::size_of::<FreeBlock>());

        // TODO: Insert the freed block at the head of free_list
        // TODO: 将已释放块插入 free_list 头部
        //
        // Steps:
        // 步骤：
        // 1. Cast ptr to *mut FreeBlock
        // 1. 将 ptr 转换为 *mut FreeBlock
        // 2. Write FreeBlock { size, next: current list head }
        // 2. 写入 FreeBlock { size, next: 当前链表头部 }
        // 3. Update free_list head to ptr
        // 3. 将 free_list 头部更新为 ptr
        let block = ptr as *mut FreeBlock;
        let head = self.free_list_head();
        block.write(FreeBlock { size, next: head });
        self.set_free_list_head(block);
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: usize = 4096;

    fn make_allocator() -> (FreeListAllocator, Vec<u8>) {
        let mut heap = vec![0u8; HEAP_SIZE];
        let start = heap.as_mut_ptr() as usize;
        let alloc = unsafe { FreeListAllocator::new(start, start + HEAP_SIZE) };
        (alloc, heap)
    }

    #[test]
    fn test_alloc_basic() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(32, 8).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_alloc_alignment() {
        let (alloc, _heap) = make_allocator();
        for align in [1, 2, 4, 8, 16] {
            let layout = Layout::from_size_align(8, align).unwrap();
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % align, 0, "align={align}");
        }
    }

    #[test]
    fn test_dealloc_and_reuse() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null());

        // After freeing, the next allocation should reuse the same block
        // 释放后，下一次分配应重用同一个块
        unsafe { alloc.dealloc(p1, layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        assert!(!p2.is_null());
        assert_eq!(p1, p2, "should reuse the freed block");
    }

    #[test]
    fn test_multiple_alloc_dealloc() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(128, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        let p3 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null() && !p2.is_null() && !p3.is_null());

        unsafe { alloc.dealloc(p2, layout) };
        unsafe { alloc.dealloc(p1, layout) };

        let q1 = unsafe { alloc.alloc(layout) };
        let q2 = unsafe { alloc.alloc(layout) };
        assert!(!q1.is_null() && !q2.is_null());
    }

    #[test]
    fn test_oom() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "should return null when exceeding heap");
    }
}
