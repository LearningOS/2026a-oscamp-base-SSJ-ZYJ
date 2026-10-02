//! # Select and Timeout
//! # Select 与超时
//!
//! In this exercise, you will use `tokio::select!` macro to implement race selection and timeout control.
//! 在本练习中，你将使用 `tokio::select!` 宏实现竞速选择和超时控制。
//!
//! ## Concepts
//! ## 概念
//! - `tokio::select!` waits for multiple async operations simultaneously
//! - `tokio::select!` 同时等待多个异步操作
//! - `tokio::time::timeout` timeout control
//! - `tokio::time::timeout` 超时控制
//! - The first completed branch is executed, others are cancelled
//! - 执行最先完成的分支，其他分支会被取消

use std::future::Future;
use tokio::time::{sleep, Duration};

/// Async operation with timeout.
/// 带超时的异步操作。
/// If `future` completes within `timeout_ms` milliseconds, returns Some(result).
/// 如果 `future` 在 `timeout_ms` 毫秒内完成，则返回 Some(result)。
/// Otherwise returns None.
/// 否则返回 None。
///
/// Hint: Use `tokio::select!` or `tokio::time::timeout`.
/// 提示：使用 `tokio::select!` 或 `tokio::time::timeout`。
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where
    F: Future<Output = T>,
{
    // TODO: Use tokio::select! to race between future and sleep
    // TODO：使用 tokio::select! 在 future 和 sleep 之间竞速
    // Or use tokio::time::timeout
    // 或者使用 tokio::time::timeout
    todo!()
}

/// Race two async tasks, return the result of whichever finishes first.
/// 竞速两个异步任务，返回先完成的那个任务的结果。
///
/// Hint: Use `tokio::select!` macro.
/// 提示：使用 `tokio::select!` 宏。
pub async fn race<F1, F2, T>(f1: F1, f2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    // TODO: Use tokio::select! to wait for f1 and f2
    // TODO：使用 tokio::select! 等待 f1 和 f2
    // Return the result of whichever completes first
    // 返回先完成的任务的结果
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = with_timeout(async { 42 }, 100).await;
        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_timeout_expired() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(200)).await;
                42
            },
            50,
        )
        .await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_race_first_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
        )
        .await;
        assert_eq!(result, "fast");
    }

    #[tokio::test]
    async fn test_race_second_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
        )
        .await;
        assert_eq!(result, "fast");
    }
}
