//! # Read-Write Lock (Writer-Priority)
//! # 读写锁（写者优先）
//!
//! In this exercise, you will implement a **writer-priority** read-write lock from scratch using atomics.
//! 在本练习中，你将从头开始使用原子操作实现一个**写者优先**的读写锁。
//! Multiple readers may hold the lock concurrently; a writer holds it exclusively.
//! 多个读者可以同时持锁；写者则独占持锁。
//!
//! **Note:** Rust's standard library already provides [`std::sync::RwLock`]. This exercise implements
//! **注意：**Rust 标准库已经提供了 [`std::sync::RwLock`]。本练习实现
//! a minimal version for learning the protocol and policy without using the standard one.
//! 一个用于学习协议和策略的最简版本，不使用标准库中的实现。
//!
//! ## Common policies for read-write locks
//! ## 读写锁的常见策略
//! Different implementations can give different **priority** when both readers and writers are waiting:
//! 当读者和写者都在等待时，不同实现可能赋予不同的**优先级**：
//!
//! - **Reader-priority (读者优先)**: New readers are allowed to enter while a writer is waiting, so writers
//! - **读者优先**：即使有写者等待，也允许新读者进入，因此读者持续到来时
//!   may be starved if readers keep arriving.
//!   写者可能会一直饥饿。
//! - **Writer-priority (写者优先)**: Once a writer is waiting, no new readers are admitted until that writer
//! - **写者优先**：一旦有写者等待，就不再接纳新读者，直到该写者运行；
//!   has run; this exercise implements this policy.
//!   本练习实现此策略。
//! - **Read-write fair (读写公平)**: Requests are served in a fair order (e.g. FIFO or round-robin), so
//! - **读写公平**：按公平顺序（例如 FIFO 或轮询）处理请求，因此
//!   neither readers nor writers are systematically starved.
//!   读者或写者都不会持续遭受饥饿。
//!
//! ## Key Concepts
//! ## 核心概念
//! - **Readers**: share access; many threads can hold a read lock at once.
//! - **读者**：共享访问；多个线程可同时持有读锁。
//! - **Writer**: exclusive access; only one writer, and no readers while the writer holds the lock.
//! - **写者**：独占访问；同一时间只能有一个写者持锁，且写者持锁时没有读者。
//! - **Writer-priority (this implementation)**: when at least one writer is waiting, new readers block
//! - **写者优先（本实现）**：至少有一个写者等待时，新读者会阻塞
//!   until the writer runs.
//!   直到写者运行。
//!
//! ## State (single atomic)
//! ## 状态（单个原子变量）
//! We use one `AtomicU32`: low bits = reader count, two flags = writer holding / writer waiting.
//! 使用一个 `AtomicU32`：低位表示读者数量，两个标志位表示写者持锁/等待。
//! All logic is implemented with compare_exchange and load/store; no use of `std::sync::RwLock`.
//! 所有逻辑都通过 compare_exchange 和 load/store 实现；不使用 `std::sync::RwLock`。

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

/// Maximum number of concurrent readers (fits in state bits).
/// 可同时持锁的最大读者数（适配状态位）。
const READER_MASK: u32 = (1 << 30) - 1;
/// Bit set when a writer holds the lock.
/// 写者持锁时置位。
const WRITER_HOLDING: u32 = 1 << 30;
/// Bit set when at least one writer is waiting (writer-priority: block new readers).
/// 至少有一个写者等待时置位（写者优先：阻止新读者进入）。
const WRITER_WAITING: u32 = 1 << 31;

/// Writer-priority read-write lock. Implemented from scratch; does not use `std::sync::RwLock`.
/// 写者优先读写锁。从头实现，不使用 `std::sync::RwLock`。
pub struct RwLock<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock. Blocks (spins) until no writer holds and no writer is waiting (writer-priority).
    /// 获取读锁。直到没有写者持锁且没有写者等待时才返回（自旋等待，写者优先）。
    ///
    /// TODO: Implement read lock acquisition
    /// TODO：实现读锁获取
    /// 1. In a loop, load state (Acquire).
    /// 1. 在循环中加载状态（Acquire）。
    /// 2. If WRITER_HOLDING or WRITER_WAITING is set, spin_loop and continue (writer-priority: no new readers while writer waits).
    /// 2. 若设置了 WRITER_HOLDING 或 WRITER_WAITING，则调用 spin_loop 并继续（写者优先：有写者等待时不接纳新读者）。
    /// 3. If reader count (state & READER_MASK) is already READER_MASK, spin and continue.
    /// 3. 若读者计数（state & READER_MASK）已达到 READER_MASK，则自旋并继续。
    /// 4. Try compare_exchange(s, s + 1, AcqRel, Acquire); on success return RwLockReadGuard { lock: self }.
    /// 4. 尝试 compare_exchange(s, s + 1, AcqRel, Acquire)；成功时返回 RwLockReadGuard { lock: self }。
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        // TODO
        loop {
            let state = self.state.load(Ordering::Acquire);

            if state & (WRITER_HOLDING | WRITER_WAITING) != 0 {
                core::hint::spin_loop();
                continue;
            }

            if state & READER_MASK == READER_MASK {
                core::hint::spin_loop();
                continue;
            }

            match self.state.compare_exchange_weak(
                state,
                state + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return RwLockReadGuard { lock: self },
                Err(_) => {
                    core::hint::spin_loop();
                    continue;
                }
            }
        }
    }

    /// Acquire the write lock. Blocks until no readers and no other writer.
    /// 获取写锁。直到没有读者且没有其他写者时才返回。
    ///
    /// TODO: Implement write lock acquisition (writer-priority)
    /// TODO：实现写锁获取（写者优先）
    /// 1. Set WRITER_WAITING first: fetch_or(WRITER_WAITING, Release) so new readers will block.
    /// 1. 先设置 WRITER_WAITING：调用 fetch_or(WRITER_WAITING, Release)，使新读者阻塞。
    /// 2. In a loop: load state; if any readers (READER_MASK) or WRITER_HOLDING, spin_loop and continue.
    /// 2. 在循环中加载状态；若有任何读者（READER_MASK）或 WRITER_HOLDING，则调用 spin_loop 并继续。
    /// 3. Try compare_exchange(WRITER_WAITING, WRITER_HOLDING, ...) to take the lock; or compare_exchange(0, WRITER_HOLDING, ...) if a writer just released.
    /// 3. 尝试 compare_exchange(WRITER_WAITING, WRITER_HOLDING, ...) 获取锁；若写者刚释放，则尝试 compare_exchange(0, WRITER_HOLDING, ...)。
    /// 4. On success return RwLockWriteGuard { lock: self }.
    /// 4. 成功时返回 RwLockWriteGuard { lock: self }。
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        // TODO
        self.state.fetch_or(WRITER_WAITING, Ordering::Release);

        loop {
            let state = self.state.load(Ordering::Acquire);

            if state & (READER_MASK | WRITER_HOLDING) != 0 {
                core::hint::spin_loop();
                continue;
            }

            let expected = if state & WRITER_WAITING != 0 {
                WRITER_WAITING
            } else {
                0
            };

            match self.state.compare_exchange_weak(
                expected,
                WRITER_HOLDING,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return RwLockWriteGuard { lock: self },
                Err(_) => {
                    core::hint::spin_loop();
                    continue;
                }
            }
        }
    }
}

/// Guard for a read lock; releases the read lock on drop.
/// 读锁守卫；丢弃时释放读锁。
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

// TODO: Implement Deref for RwLockReadGuard
// TODO：为 RwLockReadGuard 实现 Deref
// Return shared reference to data: unsafe { &*self.lock.data.get() }
// 返回数据的共享引用：unsafe { &*self.lock.data.get() }
impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

// TODO: Implement Drop for RwLockReadGuard
// TODO：为 RwLockReadGuard 实现 Drop
// Decrement reader count: self.lock.state.fetch_sub(1, Ordering::Release)
// 减少读者计数：self.lock.state.fetch_sub(1, Ordering::Release)
impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

/// Guard for a write lock; releases the write lock on drop.
/// 写锁守卫；丢弃时释放写锁。
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

// TODO: Implement Deref for RwLockWriteGuard
// TODO：为 RwLockWriteGuard 实现 Deref
// Return shared reference: unsafe { &*self.lock.data.get() }
// 返回共享引用：unsafe { &*self.lock.data.get() }
impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

// TODO: Implement DerefMut for RwLockWriteGuard
// TODO：为 RwLockWriteGuard 实现 DerefMut
// Return mutable reference: unsafe { &mut *self.lock.data.get() }
// 返回可变引用：unsafe { &mut *self.lock.data.get() }
impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

// TODO: Implement Drop for RwLockWriteGuard
// TODO：为 RwLockWriteGuard 实现 Drop
// Clear writer bits so lock is free: self.lock.state.fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release)
// 清除写者标志位以释放锁：self.lock.state.fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release)
impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .state
            .fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_multiple_readers() {
        let lock = Arc::new(RwLock::new(0u32));
        let mut handles = vec![];
        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let g = l.read();
                assert_eq!(*g, 0);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_writer_excludes_readers() {
        let lock = Arc::new(RwLock::new(0u32));
        let lock_w = Arc::clone(&lock);
        let writer = thread::spawn(move || {
            let mut g = lock_w.write();
            *g = 42;
        });
        writer.join().unwrap();
        let g = lock.read();
        assert_eq!(*g, 42);
    }

    #[test]
    fn test_concurrent_reads_after_write() {
        let lock = Arc::new(RwLock::new(Vec::<i32>::new()));
        {
            let mut g = lock.write();
            g.push(1);
            g.push(2);
        }
        let mut handles = vec![];
        for _ in 0..5 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let g = l.read();
                assert_eq!(g.len(), 2);
                assert_eq!(&*g, &[1, 2]);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_writes_serialized() {
        let lock = Arc::new(RwLock::new(0u64));
        let mut handles = vec![];
        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut g = l.write();
                    *g += 1;
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*lock.read(), 1000);
    }
}
