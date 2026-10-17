//! # Channel Communication
//! # 通道通信
//!
//! In this exercise, you will use `std::sync::mpsc` channels to pass messages between threads.
//! 在本练习中，你将使用 `std::sync::mpsc` 通道在线程间传递消息。
//!
//! ## Concepts
//! ## 概念
//! - `mpsc::channel()` creates a multiple producer, single consumer channel
//! - `mpsc::channel()` 会创建一个多生产者、单消费者通道
//! - `Sender::send()` sends a message
//! - `Sender::send()` 发送消息
//! - `Receiver::recv()` receives a message
//! - `Receiver::recv()` 接收消息
//! - Multiple producers can be created via `Sender::clone()`
//! - 可以通过 `Sender::clone()` 创建多个生产者

use std::sync::mpsc;
use std::thread;

/// Create a producer thread that sends each element from items into the channel.
/// 创建一个生产者线程，将 `items` 中的每个元素发送到通道。
/// The main thread receives all messages and returns them.
/// 主线程接收所有消息并将它们返回。
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel
    // TODO：创建通道
    let (tx, rx) = mpsc::channel();
    // TODO: Spawn thread to send each element in items
    // TODO：生成线程以发送 items 中的每个元素
    thread::spawn(move || {
        for item in items {
            tx.send(item).unwrap();
        }
    });
    // TODO: In main thread, receive all messages and collect into Vec
    // TODO：在主线程中接收所有消息并收集到 Vec 中
    let mut result = Vec::new();
    while let Ok(msg) = rx.recv() {
        result.push(msg);
    }
    // Hint: When all Senders are dropped, recv() returns Err
    // 提示：所有 Sender 都被丢弃后，recv() 会返回 Err
    result
}

/// Create `n_producers` producer threads, each sending a message in format `"msg from {id}"`.
/// 创建 `n_producers` 个生产者线程，每个线程发送格式为 `"msg from {id}"` 的消息。
/// Collect all messages, sort them lexicographically, and return.
/// 收集所有消息，按字典序排序后返回。
///
/// Hint: Use `tx.clone()` to create multiple senders. Note that the original tx must also be dropped.
/// 提示：使用 `tx.clone()` 创建多个发送端。注意也必须丢弃原始的 tx。
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: Create channel
    // TODO：创建通道
    let (tx, rx) = mpsc::channel();
    // TODO: Clone a sender for each producer
    // TODO：为每个生产者克隆一个发送端
    for id in 0..n_producers {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("msg from {}", id)).unwrap();
        });
    }
    // TODO: Remember to drop the original sender, otherwise receiver won't finish
    // TODO：记得丢弃原始发送端，否则接收端不会结束
    drop(tx);
    // TODO: Collect all messages and sort
    // TODO：收集所有消息并排序
    let mut result: Vec<String> = rx.iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(
            result,
            vec![
                "msg from 0".to_string(),
                "msg from 1".to_string(),
                "msg from 2".to_string(),
            ]
        );
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
