//! # Memory Ordering and Synchronization
//! # 内存顺序与同步
//!
//! In this exercise, you will use correct memory ordering to implement thread synchronization primitives.
//! 在本练习中，你将使用正确的内存顺序实现线程同步原语。
//!
//! ## Key Concepts
//! ## 核心概念
//! - `Ordering::Relaxed`: No synchronization guarantees
//! - `Ordering::Relaxed`：不提供同步保证
//! - `Ordering::Acquire`: Read operation, prevents subsequent reads/writes from being reordered before this operation
//! - `Ordering::Acquire`：读取操作，防止后续读写被重排到此操作之前
//! - `Ordering::Release`: Write operation, prevents preceding reads/writes from being reordered after this operation
//! - `Ordering::Release`：写入操作，防止此前读写被重排到此操作之后
//! - `Ordering::AcqRel`: Both Acquire and Release semantics
//! - `Ordering::AcqRel`：同时具有 Acquire 和 Release 语义
//! - `Ordering::SeqCst`: Sequentially consistent (global ordering)
//! - `Ordering::SeqCst`：顺序一致（全局有序）
//!
//! ## Release-Acquire Pairing
//! ## Release-Acquire 配对
//! When thread A writes with Release, and thread B reads the same location with Acquire,
//! 线程 A 使用 Release 写入后，线程 B 使用 Acquire 读取同一位置时，
//! thread B will see all writes that thread A performed before the Release.
//! 线程 B 将看到线程 A 在 Release 操作之前执行的所有写入。

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Use Release-Acquire semantics to safely pass data between two threads.
/// 使用 Release-Acquire 语义在线程间安全传递数据。
///
/// `produce` writes data first, then sets flag with Release;
/// `produce` 先写入数据，然后使用 Release 设置标志；
/// `consume` reads flag with Acquire, ensuring it sees the data.
/// `consume` 使用 Acquire 读取标志，从而确保能看到数据。
pub struct FlagChannel {
    data: AtomicU32,
    ready: AtomicBool,
}

impl FlagChannel {
    pub const fn new() -> Self {
        Self {
            data: AtomicU32::new(0),
            ready: AtomicBool::new(false),
        }
    }

    /// Producer: store data first, then set ready flag.
    /// 生产者：先存储数据，然后设置 ready 标志。
    ///
    /// TODO: Choose correct Ordering
    /// TODO：选择正确的 Ordering
    /// - What Ordering should be used for writing data?
    /// - 写入数据应使用什么 Ordering？
    /// - What Ordering should be used for writing ready? (ensuring data writes are visible to consumer)
    /// - 写入 ready 应使用什么 Ordering？（确保消费者能看到数据写入）
    pub fn produce(&self, value: u32) {
        // TODO: Store data (choose appropriate Ordering)
        // TODO：存储数据（选择合适的 Ordering）
        // TODO: Set ready = true (choose appropriate Ordering so data writes complete before this)
        // TODO：设置 ready = true（选择合适的 Ordering，使数据写入先于此操作完成）
        self.data.store(value, Ordering::Relaxed);
        self.ready.store(true, Ordering::Release);
    }

    /// Consumer: spin-wait for ready flag, then read data.
    /// 消费者：自旋等待 ready 标志，然后读取数据。
    ///
    /// TODO: Choose correct Ordering
    /// TODO：选择正确的 Ordering
    /// - What Ordering should be used for reading ready? (ensuring it sees data writes from produce)
    /// - 读取 ready 应使用什么 Ordering？（确保能看到 produce 写入的数据）
    /// - What Ordering should be used for reading data?
    /// - 读取数据应使用什么 Ordering？
    pub fn consume(&self) -> u32 {
        // TODO: Spin-wait for ready to become true (choose appropriate Ordering)
        // TODO：自旋等待 ready 变为 true（选择合适的 Ordering）
        // TODO: Read data (choose appropriate Ordering)
        // TODO：读取数据（选择合适的 Ordering）
        while !self.ready.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        self.data.load(Ordering::Relaxed)
    }

    /// Reset channel state
    /// 重置通道状态。
    pub fn reset(&self) {
        self.ready.store(false, Ordering::Relaxed);
        self.data.store(0, Ordering::Relaxed);
    }
}

/// A simple once-initializer using SeqCst.
/// 使用 SeqCst 的简单一次性初始化器。
/// Guarantees `init` is executed only once, and all threads see the initialized value.
/// 保证 `init` 只执行一次，并使所有线程都能看到已初始化的值。
pub struct OnceCell {
    initialized: AtomicBool,
    value: AtomicU32,
}

impl OnceCell {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: AtomicU32::new(0),
        }
    }

    /// Attempt initialization. If not yet initialized, store value and return true.
    /// 尝试初始化。如果尚未初始化，则存储该值并返回 true。
    /// If already initialized, return false.
    /// 如果已经初始化，则返回 false。
    ///
    /// Hint: use `compare_exchange` to ensure only one thread succeeds.
    /// 提示：使用 `compare_exchange` 确保只有一个线程成功。
    pub fn init(&self, val: u32) -> bool {
        // TODO: Use compare_exchange to ensure initialization only once
        // TODO：使用 compare_exchange 确保只初始化一次
        // Store value on success
        // 成功时存储值
        if self
            .initialized
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.value.store(val, Ordering::Release);
            true
        } else {
            false
        }
    }

    /// Get value. Returns Some if initialized, otherwise None.
    /// 获取值。若已初始化则返回 Some，否则返回 None。
    pub fn get(&self) -> Option<u32> {
        // TODO: Check initialized flag, then read value
        // TODO：检查 initialized 标志，然后读取值
        if self.initialized.load(Ordering::Acquire) {
            Some(self.value.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_flag_channel() {
        let ch = Arc::new(FlagChannel::new());
        let ch2 = Arc::clone(&ch);

        let producer = thread::spawn(move || {
            ch2.produce(42);
        });

        let consumer = thread::spawn(move || ch.consume());

        producer.join().unwrap();
        let val = consumer.join().unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn test_flag_channel_large_value() {
        let ch = Arc::new(FlagChannel::new());
        let ch2 = Arc::clone(&ch);

        let producer = thread::spawn(move || {
            ch2.produce(0xDEAD_BEEF);
        });

        let val = ch.consume();
        producer.join().unwrap();
        assert_eq!(val, 0xDEAD_BEEF);
    }

    #[test]
    fn test_once_cell_init_once() {
        let cell = OnceCell::new();
        assert!(cell.init(42));
        assert!(!cell.init(100));
        assert_eq!(cell.get(), Some(42));
    }

    #[test]
    fn test_once_cell_not_initialized() {
        let cell = OnceCell::new();
        assert_eq!(cell.get(), None);
    }

    #[test]
    fn test_once_cell_concurrent() {
        let cell = Arc::new(OnceCell::new());
        let mut handles = vec![];

        for i in 0..10 {
            let c = Arc::clone(&cell);
            handles.push(thread::spawn(move || c.init(i)));
        }

        let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        // Exactly one thread initializes successfully
        // 恰有一个线程成功完成初始化
        assert_eq!(results.iter().filter(|&&r| r).count(), 1);
        assert!(cell.get().is_some());
    }
}
