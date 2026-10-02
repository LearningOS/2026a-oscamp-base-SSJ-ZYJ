//! # RAII Spin Lock Guard
//! # RAII 自旋锁守卫
//!
//! In this exercise, you will implement an RAII guard for a spin lock, causing the lock to be automatically released when leaving scope.
//! 在本练习中，你将为自旋锁实现 RAII 守卫，使离开作用域时自动释放锁。
//! This is a classic application of Rust's ownership system in systems programming.
//! 这是 Rust 所有权系统在系统编程中的经典应用。
//!
//! ## Key Points
//! ## 要点
//! - RAII (Resource Acquisition Is Initialization) pattern
//! - RAII（资源获取即初始化）模式
//! - `Deref` / `DerefMut` traits for transparent access
//! - `Deref` / `DerefMut` trait，实现透明访问
//! - `Drop` trait for automatic release
//! - `Drop` trait，实现自动释放
//! - Why manual lock/unlock is unsafe (forgetting unlock, panic without release)
//! - 手动加锁/解锁为何不安全（可能忘记解锁，或 panic 后未释放）

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

/// Spin lock guard: RAII handle holding the lock.
/// 自旋锁守卫：持有锁的 RAII 句柄。
/// Automatically releases the lock when SpinGuard is dropped.
/// SpinGuard 被丢弃时会自动释放锁。
pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire lock, returning SpinGuard.
    /// 获取锁，并返回 SpinGuard。
    ///
    /// TODO: Spin-wait to acquire lock (compare_exchange), return SpinGuard on success.
    /// TODO：自旋等待获取锁（compare_exchange），成功后返回 SpinGuard。
    pub fn lock(&self) -> SpinGuard<'_, T> {
        // TODO: Spin-wait to acquire lock
        // TODO：自旋等待获取锁
        // TODO: Return SpinGuard { lock: self }
        // TODO：返回 SpinGuard { lock: self }
        todo!()
    }
}

// TODO: Implement Deref trait for SpinGuard
// TODO：为 SpinGuard 实现 Deref trait
// Return &T, obtained via self.lock.data.get()
// 返回 &T，通过 self.lock.data.get() 获取
impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!()
    }
}

// TODO: Implement DerefMut trait for SpinGuard
// TODO：为 SpinGuard 实现 DerefMut trait
// Return &mut T
// 返回 &mut T
impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        todo!()
    }
}

// TODO: Implement Drop trait for SpinGuard
// TODO：为 SpinGuard 实现 Drop trait
// Set lock.locked to false (Release ordering)
// 将 lock.locked 设为 false（Release 顺序）
impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_guard_auto_release() {
        let lock = SpinLock::new(0u32);
        {
            let mut guard = lock.lock();
            *guard = 42;
            // guard drops here, automatically releasing lock
            // 守卫在此处丢弃，并自动释放锁
        }
        // Should be able to acquire lock again
        // 应能够再次获取锁
        let guard = lock.lock();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_guard_deref() {
        let lock = SpinLock::new(String::from("hello"));
        let guard = lock.lock();
        assert_eq!(guard.len(), 5);
        assert_eq!(&*guard, "hello");
    }

    #[test]
    fn test_guard_deref_mut() {
        let lock = SpinLock::new(Vec::<i32>::new());
        {
            let mut guard = lock.lock();
            guard.push(1);
            guard.push(2);
            guard.push(3);
        }
        let guard = lock.lock();
        assert_eq!(&*guard, &[1, 2, 3]);
    }

    #[test]
    fn test_concurrent_with_guard() {
        let lock = Arc::new(SpinLock::new(0u64));
        let mut handles = vec![];

        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = l.lock();
                    *guard += 1;
                    // guard automatically released
                    // 守卫会自动释放锁
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*lock.lock(), 10000);
    }

    #[test]
    fn test_panic_safety() {
        let lock = Arc::new(SpinLock::new(0u32));
        let l = Arc::clone(&lock);

        let result = thread::spawn(move || {
            let mut guard = l.lock();
            *guard = 42;
            panic!("intentional panic");
        })
        .join();

        assert!(result.is_err());
        // Even if thread panics, guard's Drop should release lock
        // 即使线程发生 panic，守卫的 Drop 也应释放锁
        // Note: this test may have different results due to panic unwind behavior
        // 注意：此测试在 panic unwind 行为不同的情况下可能会有不同结果
    }
}
