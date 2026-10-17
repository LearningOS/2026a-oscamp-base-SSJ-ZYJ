//! # Thread Creation
//! # 线程创建
//!
//! In this exercise, you will learn how to create threads and pass data between threads.
//! 本练习将介绍如何创建线程并在线程间传递数据。
//!
//! ## Concepts
//! ## 概念
//! - `std::thread::spawn` creates a new thread
//! - `std::thread::spawn` 创建新线程
//! - `move` closures capture variable ownership
//! - `move` 闭包会捕获变量所有权
//! - `JoinHandle::join()` waits for thread completion and retrieves return value
//! - `JoinHandle::join()` 等待线程完成并获取其返回值
//!
//! ## Advanced Thread Operations
//! ## 高级线程操作
//! - **Thread sleep**: `thread::sleep` pauses the current thread.
//! - **线程休眠**：`thread::sleep` 会暂停当前线程。
//! - **Thread‑local storage**: `thread_local!` macro defines static variables unique to each thread.
//! - **线程局部存储**：`thread_local!` 宏定义每个线程独有的静态变量。
//! - **Thread naming**: `Builder::name` assigns a name for debugging.
//! - **线程命名**：`Builder::name` 为线程指定名称，便于调试。
//! - **Thread priority**: Set via `thread::Builder` (platform‑dependent).
//! - **线程优先级**：通过 `thread::Builder` 设置（取决于平台）。
//! - **Thread pools**: Libraries like `rayon` manage thread reuse.
//! - **线程池**：`rayon` 等库会管理线程的复用。
//! - **Thread communication**: Use `std::sync::mpsc` (multi‑producer single‑consumer) or third‑party crates (e.g., `crossbeam`).
//! - **线程通信**：使用 `std::sync::mpsc`（多生产者、单消费者）或第三方 crate（如 `crossbeam`）。
//! - **Shared state**: `Arc<Mutex<T>>` or `Arc<RwLock<T>>` safely share mutable data across threads.
//! - **共享状态**：使用 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>` 在线程间安全共享可变数据。
//! - **Synchronization primitives**: `Barrier` synchronizes multiple threads, `Condvar` implements condition variables.
//! - **同步原语**：`Barrier` 用于同步多个线程，`Condvar` 用于实现条件变量。
//! - **Thread park/unpark**: `thread::park` blocks a thread, `unpark` wakes it, useful for custom scheduling.
//! - **线程 park/unpark**：`thread::park` 会阻塞线程，`unpark` 会将其唤醒，可用于自定义调度。
//! - **Get current thread handle**: `thread::current()`.
//! - **获取当前线程句柄**：`thread::current()`。
//! - **Scoped threads**: `crossbeam::scope` or standard‑library `thread::scope` (Rust 1.63+) allow threads to borrow stack data without `move`.
//! - **作用域线程**：`crossbeam::scope` 或标准库 `thread::scope`（Rust 1.63+）允许线程借用栈上数据，无需使用 `move`。
//!
//! Rust statically prevents data races through the ownership system and the `Send` and `Sync` traits.
//! Rust 通过所有权系统以及 `Send` 和 `Sync` trait 在静态层面防止数据竞争。
//! Types that implement `Send` can be transferred across thread boundaries.
//! 实现 `Send` 的类型可以在线程间传递。
//! Types that implement `Sync` can be referenced from multiple threads simultaneously.
//! 实现 `Sync` 的类型可以被多个线程同时引用。
//! Most Rust standard types are `Send + Sync`; exceptions include `Rc<T>` (non‑atomic reference counting) and raw pointers.
//! 大多数 Rust 标准库类型都实现了 `Send + Sync`；例外包括 `Rc<T>`（非原子引用计数）和裸指针。
//!
//! ## Exercise Structure
//! ## 练习结构
//! 1. **Basic exercises** (`double_in_thread`, `parallel_sum`) – introduce fundamental thread creation.
//! 1. **基础练习**（`double_in_thread`、`parallel_sum`）——介绍线程创建的基本用法。
//! 2. **Advanced exercises** (`named_sleeper`, `increment_thread_local`, `scoped_slice_sum`, `handle_panic`) – explore additional thread operations.
//! 2. **进阶练习**（`named_sleeper`、`increment_thread_local`、`scoped_slice_sum`、`handle_panic`）——探索更多线程操作。
//! Each function includes a `TODO` comment indicating where you need to write code.
//! 每个函数都包含一个 `TODO` 注释，标明需要编写代码的位置。
//! Run `cargo test` to check your implementations.
//! 运行 `cargo test` 检查你的实现。

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

// ============================================================================
// Example Code: Advanced Thread Patterns
// 高级线程模式示例代码
// ============================================================================
// The following examples illustrate additional thread‑related concepts that are
// 以下示例说明了其他与线程相关的概念，这些概念在
// useful in real‑world Rust concurrent programming.
// 实际 Rust 并发编程中很有用。

/// Example: Handling thread panic.
/// 示例：处理线程 panic。
///
/// `join()` returns a `Result`. If the thread panics, the `Result` is an `Err`.
/// `join()` 返回一个 `Result`。如果线程发生 panic，`Result` 会是 `Err`。
/// This demonstrates how to catch and handle a panic from a spawned thread.
/// 此示例展示如何捕获并处理生成线程中的 panic。
///
/// ```rust
/// use std::thread;
///
/// fn panic_handling_example() {
///     let handle = thread::spawn(|| {
///         // Simulate a panic
///         // 模拟 panic
///         panic!("Thread panicked!");
///     });
///
///     match handle.join() {
///         Ok(_) => println!("Thread completed successfully."),
///         Err(e) => println!("Thread panicked: {:?}", e),
///     }
/// }
/// ```
///
/// In contrast, the exercises below use `unwrap()` for simplicity, assuming
/// 相比之下，下面的练习为简洁起见使用 `unwrap()`，前提是
/// that the threads never panic.
/// 这些线程不会发生 panic。

/// Example: Named thread and custom stack size.
/// 示例：命名线程并设置自定义栈大小。
///
/// Using `thread::Builder` you can assign a name to a thread (helpful for
/// 使用 `thread::Builder` 可以为线程指定名称（便于
/// debugging) and set its stack size.
/// 调试），也可以设置其栈大小。
///
/// ```rust
/// use std::thread;
///
/// fn named_thread_example() {
///     let builder = thread::Builder::new()
///         .name("my-worker".into())
///         .stack_size(32 * 1024); // 32 KiB
///
///     let handle = builder.spawn(|| {
///         println!("Hello from thread: {:?}", thread::current().name());
///         42
///     }).unwrap();
///
///     let result = handle.join().unwrap();
///     println!("Thread returned: {}", result);
/// }
/// ```

/// Example: Scoped threads (Rust 1.63+).
/// 示例：作用域线程（Rust 1.63+）。
///
/// Scoped threads allow borrowing stack data without moving ownership.
/// 作用域线程允许借用栈上数据，而不必转移所有权。
/// The threads are guaranteed to finish before the scope ends, so references
/// 保证这些线程会在作用域结束前完成，因此引用始终有效。
/// remain valid.
///
/// ```rust
/// use std::thread;
///
/// fn scoped_thread_example() {
///     let a = vec![1, 2, 3];
///     let b = vec![4, 5, 6];
///
///     let (sum_a, sum_b) = thread::scope(|s| {
///         let h1 = s.spawn(|| a.iter().sum::<i32>());
///         let h2 = s.spawn(|| b.iter().sum::<i32>());
///         (h1.join().unwrap(), h2.join().unwrap())
///     });
///
///     // `a` and `b` are still accessible here.
///     // 此处仍可访问 `a` 和 `b`。
///     println!("sum_a = {}, sum_b = {}", sum_a, sum_b);
/// }
/// ```

/// Example: Thread‑local storage.
/// 示例：线程局部存储。
///
/// Each thread gets its own independent copy of a `thread_local!` variable.
/// 每个线程都会获得 `thread_local!` 变量各自独立的副本。
///
/// ```rust
/// use std::cell::RefCell;
/// use std::thread;
///
/// thread_local! {
///     static THREAD_ID: RefCell<usize> = RefCell::new(0);
/// }
///
/// fn thread_local_example() {
///     THREAD_ID.with(|id| {
///         *id.borrow_mut() = 1;
///     });
///
///     let handle = thread::spawn(|| {
///         THREAD_ID.with(|id| {
///             *id.borrow_mut() = 2;
///         });
///         THREAD_ID.with(|id| println!("Thread local value: {}", *id.borrow()));
///     });
///
///     handle.join().unwrap();
///
///     THREAD_ID.with(|id| println!("Main thread value: {}", *id.borrow()));
/// }
/// ```

// ============================================================================
// Exercise Functions
// ============================================================================

/// Multiply each element of a vector by 2 in a new thread, returning the result vector.
/// 在线程中将向量的每个元素乘以 2，并返回结果向量。
///
/// Hint: Use `thread::spawn` and `move` closure.
/// 提示：使用 `thread::spawn` 和 `move` 闭包。
#[allow(unused_variables)]
pub fn double_in_thread(numbers: Vec<i32>) -> Vec<i32> {
    // TODO: Create a new thread to multiply each element of numbers by 2
    // TODO：创建一个新线程，将 numbers 的每个元素乘以 2
    // Use thread::spawn and move closure
    // 使用 thread::spawn 和 move 闭包
    let handle = thread::spawn(move || numbers.into_iter().map(|x| x * 2).collect::<Vec<i32>>());
    // Use join().unwrap() to get result
    // 使用 join().unwrap() 获取结果
    handle.join().unwrap()
}

/// Sum two vectors in parallel, returning a tuple of two sums.
/// 并行求两个向量的和，并返回两个和组成的元组。
///
/// Hint: Create two threads for each vector.
/// 提示：为每个向量分别创建一个线程。
#[allow(unused_variables)]
pub fn parallel_sum(a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    // TODO: Create two threads to sum a and b respectively
    // TODO：分别创建两个线程，对 a 和 b 求和
    let handle_a = thread::spawn(move || a.iter().sum::<i32>());
    let handle_b = thread::spawn(move || b.iter().sum::<i32>());

    // Join both threads to get results
    // 等待两个线程并获取结果
    (handle_a.join().unwrap(), handle_b.join().unwrap())
}

// ============================================================================
// Advanced Exercise Functions
// 高级练习函数
// ============================================================================

/// Create a named thread that sleeps for the given milliseconds and then returns the input value.
/// 创建一个命名线程，使其休眠指定的毫秒数后返回输入值。
///
/// The thread should be named `"sleeper"`. Use `thread::Builder` to set the name.
/// 线程名称应为 `"sleeper"`。使用 `thread::Builder` 设置名称。
/// Inside the thread, call `thread::sleep(Duration::from_millis(ms))` before returning `value`.
/// 在线程中返回 `value` 之前，调用 `thread::sleep(Duration::from_millis(ms))`。
///
/// Hint: `thread::sleep` causes the current thread to block; it does not affect other threads.
/// 提示：`thread::sleep` 会阻塞当前线程，不会影响其他线程。
#[allow(unused_variables)]
pub fn named_sleeper(value: i32, ms: u64) -> i32 {
    // TODO: Create a thread builder with name "sleeper"
    // TODO：创建名称为 "sleeper" 的线程构建器
    let handle = thread::Builder::new()
        .name("sleeper".into())
        .spawn(move || {
            thread::sleep(Duration::from_millis(ms));
            value
        })
        .unwrap();
    // TODO: Spawn a thread that sleeps for `ms` milliseconds and returns `value`
    // TODO：启动线程，使其休眠 `ms` 毫秒后返回 `value`
    // TODO: Join the thread and return the value
    // TODO：等待线程并返回其值
    handle.join().unwrap()
}

thread_local! {
    static THREAD_COUNT: RefCell<usize> = RefCell::new(0);
}

/// Use thread‑local storage to count how many times each thread calls `increment`.
/// 使用线程局部存储来统计每个线程调用 `increment` 的次数。
///
/// Define a `thread_local!` static `THREAD_COUNT` of type `RefCell<usize>` initialized to 0.
/// 定义一个初值为 0、类型为 `RefCell<usize>` 的线程局部静态变量 `THREAD_COUNT`。
/// Each call to `increment` should increase the thread‑local count by 1 and return the new value.
/// 每次调用 `increment` 时，都应将线程局部计数加 1 并返回新值。
///
/// Hint: Use `THREAD_COUNT.with(|cell| { ... })` to access the thread‑local variable.
/// 提示：使用 `THREAD_COUNT.with(|cell| { ... })` 访问线程局部变量。
pub fn increment_thread_local() -> usize {
    // TODO: Use THREAD_COUNT.with to increment and return the new count
    // TODO：使用 THREAD_COUNT.with 递增计数并返回新值
    THREAD_COUNT.with(|cell| {
        let mut count = cell.borrow_mut();
        *count += 1;
        *count
    })
}

/// Spawn two threads using a **scoped thread** to compute the sum of two slices without moving ownership.
/// 使用**作用域线程**生成两个线程，计算两个切片的和，而不转移所有权。
///
/// Use `thread::scope` to allow threads to borrow the slices `&[i32]`.
/// 使用 `thread::scope` 让线程借用切片 `&[i32]`。
/// Each thread should compute the sum of its slice, and the function returns `(sum_a, sum_b)`.
/// 每个线程应计算其切片的和，函数返回 `(sum_a, sum_b)`。
///
/// Hint: The slices are references, so you cannot move them into the closure.
/// 提示：这些切片是引用，因此不能将它们移动到闭包中。
/// `thread::scope` guarantees that all spawned threads finish before the scope ends,
/// `thread::scope` 保证所有生成的线程都会在作用域结束前完成，
/// making the borrow safe.
/// 因此借用是安全的。
#[allow(unused_variables)]
pub fn scoped_slice_sum(a: &[i32], b: &[i32]) -> (i32, i32) {
    // TODO: Use thread::scope to spawn two threads
    // TODO：使用 thread::scope 生成两个线程
    // TODO: Each thread sums its slice
    // TODO：每个线程都对自己的切片求和
    thread::scope(|scope| {
        let handle_a = scope.spawn(|| a.iter().sum::<i32>());
        let handle_b = scope.spawn(|| b.iter().sum::<i32>());
        // TODO: Wait for both threads and return the results
        // TODO：等待两个线程并返回结果
        (handle_a.join().unwrap(), handle_b.join().unwrap())
    })
}

/// Handle a possible panic in a spawned thread.
/// 处理生成线程可能发生的 panic。
///
/// Spawn a thread that may panic: if `should_panic` is `true`, the thread calls `panic!("oops")`;
/// 生成一个可能 panic 的线程：若 `should_panic` 为 `true`，线程调用 `panic!("oops")`；
/// otherwise it returns `value`.
/// 否则返回 `value`。
/// The function should return `Ok(value)` if the thread completed successfully,
/// 如果线程成功完成，函数应返回 `Ok(value)`；
/// or `Err(())` if the thread panicked.
/// 如果线程发生 panic，则返回 `Err(())`。
///
/// Hint: `join()` returns `Result<Result<i32, Box<dyn Any + Send>>, _>`.
/// 提示：`join()` 返回 `Result<Result<i32, Box<dyn Any + Send>>, _>`。
/// You'll need to match the outer `Result` (thread panic) and the inner `Result` (if the thread returns a `Result`).
/// 你需要分别匹配外层 `Result`（线程 panic）和内层 `Result`（线程返回 `Result` 时）。
/// In this exercise, the inner type is just `i32`, not a `Result`.
/// 本练习中，内层类型只是 `i32`，而不是 `Result`。
#[allow(unused_variables)]
pub fn handle_panic(value: i32, should_panic: bool) -> Result<i32, ()> {
    // TODO: Spawn a thread that either panics or returns value
    // TODO：生成一个会 panic 或返回 value 的线程
    let handle = thread::spawn(move || {
        if should_panic {
            panic!("oops")
        }
        value
    });
    // TODO: Join and map the result appropriately
    // TODO：等待线程，并按要求映射结果
    match handle.join() {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_basic() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(double_in_thread(nums), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_double_empty() {
        assert_eq!(double_in_thread(vec![]), vec![]);
    }

    #[test]
    fn test_double_negative() {
        assert_eq!(double_in_thread(vec![-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_parallel_sum() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20, 30];
        assert_eq!(parallel_sum(a, b), (6, 60));
    }

    #[test]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![], vec![]), (0, 0));
    }

    // Advanced exercise tests
    // 高级练习测试
    #[test]
    fn test_named_sleeper() {
        // The thread should sleep a short time; we just verify it returns the correct value.
        // 线程应短暂休眠；这里只验证返回值正确。
        let result = named_sleeper(42, 10); // sleep 10 ms
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thread_local() {
        // Each thread has its own counter, so spawning two threads and calling increment
        // 每个线程有独立计数器，因此分别在两个线程中调用 increment
        // in each should give each thread its own sequence.
        // 应使每个线程都得到各自的序列。
        use std::sync::Arc;
        use std::sync::Mutex;

        let counters = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let counters = Arc::clone(&counters);
            handles.push(thread::spawn(move || {
                let v1 = increment_thread_local();
                let v2 = increment_thread_local();
                counters.lock().unwrap().push((v1, v2));
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let results = counters.lock().unwrap();
        // Each thread should have counted (1, 2) independently.
        // 每个线程都应独立完成计数 (1, 2)。
        assert_eq!(results.len(), 2);
        assert!(results.contains(&(1, 2)));
    }

    #[test]
    fn test_scoped_slice_sum() {
        let a = [1, 2, 3];
        let b = [10, 20, 30];
        let (sum_a, sum_b) = scoped_slice_sum(&a, &b);
        assert_eq!(sum_a, 6);
        assert_eq!(sum_b, 60);
        // Ensure slices are still accessible (they are borrowed, not moved).
        // 确认切片仍可访问（它们是借用的，而非被移动）。
        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn test_handle_panic_ok() {
        let result = handle_panic(100, false);
        assert_eq!(result, Ok(100));
    }

    #[test]
    fn test_handle_panic_error() {
        let result = handle_panic(100, true);
        assert_eq!(result, Err(()));
    }
}
