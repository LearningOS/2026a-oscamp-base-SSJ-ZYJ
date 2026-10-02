//! # Manual Future Implementation
//! # 手动实现 Future
//!
//! In this exercise, you will manually implement the `Future` trait for custom types to understand the core mechanism of asynchronous runtime.
//! 本练习中，你将为自定义类型手动实现 `Future` trait，以理解异步运行时的核心机制。
//!
//! ## Concepts
//! ## 概念
//! - `std::future::Future` trait
//! - `std::future::Future` trait
//! - `Poll::Ready` and `Poll::Pending`
//! - `Poll::Ready` 和 `Poll::Pending`
//! - The role of `Waker`: notifying the runtime to poll again
//! - `Waker` 的作用：通知运行时再次轮询

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Countdown Future: decrements count by 1 each time it's polled,
/// 倒计时 Future：每次被轮询时将 count 减 1，
/// returns `"liftoff!"` when count reaches 0.
/// 当 count 达到 0 时返回 `"liftoff!"`。
pub struct CountDown {
    pub count: u32,
}

impl CountDown {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

// TODO: Implement Future trait for CountDown
// TODO：为 CountDown 实现 Future trait
// - Output type is &'static str
// - 输出类型为 &'static str
// - Each poll: if count == 0, return Poll::Ready("liftoff!")
// - 每次轮询时：若 count == 0，则返回 Poll::Ready("liftoff!")
// - Otherwise count -= 1, call cx.waker().wake_by_ref(), return Poll::Pending
// - 否则将 count 减 1，调用 cx.waker().wake_by_ref()，并返回 Poll::Pending
//
// Hint: Use `self.get_mut()` to get `&mut Self` (since self is Pin<&mut Self>)
// 提示：使用 `self.get_mut()` 获取 `&mut Self`（因为 self 的类型为 Pin<&mut Self>）
impl Future for CountDown {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

/// Yield-only-once Future: first poll returns Pending, second returns Ready(()).
/// 仅让出一次的 Future：第一次轮询返回 Pending，第二次返回 Ready(())。
/// This is the minimal example of an asynchronous state machine.
/// 这是异步状态机的最简示例。
pub struct YieldOnce {
    yielded: bool,
}

impl YieldOnce {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

// TODO: Implement Future trait for YieldOnce
// TODO：为 YieldOnce 实现 Future trait
// - Output type is ()
// - 输出类型为 ()
// - First poll: set yielded = true, wake waker, return Pending
// - 第一次轮询：将 yielded 设为 true，唤醒 waker，并返回 Pending
// - Second poll: return Ready(())
// - 第二次轮询：返回 Ready(())
impl Future for YieldOnce {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_countdown_zero() {
        let result = CountDown::new(0).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_countdown_three() {
        let result = CountDown::new(3).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_yield_once() {
        YieldOnce::new().await;
    }

    #[tokio::test]
    async fn test_countdown_large() {
        let result = CountDown::new(100).await;
        assert_eq!(result, "liftoff!");
    }
}
