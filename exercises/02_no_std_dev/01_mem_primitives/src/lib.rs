//! # no_std Memory Primitives
//! # no_std 内存原语
//!
//! In a `#![no_std]` environment, you have no standard library — only `core`.
//! 在 `#![no_std]` 环境中没有标准库——只有 `core`。
//! These memory operation functions are the most fundamental building blocks in an OS kernel.
//! 这些内存操作函数是操作系统内核中最基础的构建模块。
//! Functions like memcpy/memset in libc must be implemented by ourselves in bare-metal environments.
//! 在裸机环境中，必须由我们自行实现 libc 中的 memcpy/memset 等函数。
//!
//! ## Task
//! ## 任务
//!
//! Implement the following five functions:
//! 实现以下五个函数：
//! - Only use the `core` crate, no `std`
//! - 只能使用 `core` crate，不能使用 `std`
//! - Do not call `core::ptr::copy`, `core::ptr::copy_nonoverlapping`, etc. (write your own loops)
//! - 不要调用 `core::ptr::copy`、`core::ptr::copy_nonoverlapping` 等函数（请自行编写循环）
//! - Handle edge cases correctly (n=0, overlapping memory regions, etc.)
//! - 正确处理边界情况（n=0、内存区域重叠等）
//! - Pass all tests
//! - 通过所有测试

// Force no_std in production; allow std in tests (cargo test framework requires it)
// 在生产环境中强制使用 no_std；在测试中允许使用 std（cargo test 测试框架需要它）
#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]

/// Copy `n` bytes from `src` to `dst`.
/// 从 `src` 向 `dst` 复制 `n` 个字节。
///
/// - `dst` and `src` must not overlap (use `my_memmove` for overlapping regions)
/// - `dst` 和 `src` 不得重叠（区域重叠时请使用 `my_memmove`）
/// - Returns `dst`
/// - 返回 `dst`
///
/// # Safety
/// # 安全性
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
/// `dst` 和 `src` 必须分别指向至少 `n` 个字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: Implement memcpy
    // TODO：实现 memcpy
    // Hint: read bytes from src one by one and write to dst
    // 提示：逐个读取 src 中的字节并写入 dst
    for i in 0..n {
        *dst.add(i) = *src.add(i);
    }
    dst
}

/// Set `n` bytes starting at `dst` to the value `c`.
/// 将从 `dst` 开始的 `n` 个字节设置为值 `c`。
///
/// Returns `dst`.
/// 返回 `dst`。
///
/// # Safety
/// # 安全性
/// `dst` must point to at least `n` bytes of valid writable memory.
/// `dst` 必须指向至少 `n` 个字节的有效可写内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memset(dst: *mut u8, c: u8, n: usize) -> *mut u8 {
    // TODO: Implement memset
    // TODO：实现 memset
    for i in 0..n {
        *dst.add(i) = c;
    }
    dst
}

/// Copy `n` bytes from `src` to `dst`, correctly handling overlapping memory.
/// 从 `src` 向 `dst` 复制 `n` 个字节，并正确处理内存重叠。
///
/// Returns `dst`.
/// 返回 `dst`。
///
/// # Safety
/// # 安全性
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
/// `dst` 和 `src` 必须分别指向至少 `n` 个字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: Implement memmove
    // TODO：实现 memmove
    // Hint: when dst > src and regions overlap, copy backwards (from end to start)
    // 提示：当 dst > src 且内存区域重叠时，应从后向前复制
    if n == 0 {
        return dst;
    }

    let dst_addr = dst as usize;
    let src_addr = src as usize;

    if dst_addr > src_addr && dst_addr < src_addr + n {
        for i in (0..n).rev() {
            *dst.add(i) = *src.add(i);
        }
    } else {
        for i in 0..n {
            *dst.add(i) = *src.add(i);
        }
    }

    dst
}

/// Return the length of a null-terminated byte string, excluding the trailing null.
/// 返回以 null 结尾的字节字符串长度（不包括末尾的 null）。
///
/// # Safety
/// # 安全性
/// `s` must point to a valid null-terminated byte string.
/// `s` 必须指向一个有效的、以 null 结尾的字节字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strlen(s: *const u8) -> usize {
    // TODO: Implement strlen
    // TODO：实现 strlen
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

/// Compare two null-terminated byte strings.
/// 比较两个以 null 结尾的字节字符串。
///
/// Returns:
/// 返回值：
/// - `0`  : strings are equal
/// - `0`：字符串相等
/// - `< 0`: `s1` is lexicographically less than `s2`
/// - `< 0`：`s1` 按字典序小于 `s2`
/// - `> 0`: `s1` is lexicographically greater than `s2`
/// - `> 0`：`s1` 按字典序大于 `s2`
///
/// # Safety
/// # 安全性
/// `s1` and `s2` must each point to a valid null-terminated byte string.
/// `s1` 和 `s2` 必须分别指向有效的、以 null 结尾的字节字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    // TODO: Implement strcmp
    // TODO：实现 strcmp
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);

        if c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
}

// ============================================================
// ============================================================
// Tests (std is available under #[cfg(test)])
// 测试（在 #[cfg(test)] 下可使用 std）
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memcpy_basic() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 5) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memcpy_zero_len() {
        let src = [0xFFu8; 4];
        let mut dst = [0u8; 4];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 0) };
        assert_eq!(dst, [0u8; 4]);
    }

    #[test]
    fn test_memset_basic() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xAB, 8) };
        assert!(buf.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_memset_partial() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xFF, 4) };
        assert_eq!(&buf[..4], &[0xFF; 4]);
        assert_eq!(&buf[4..], &[0x00; 4]);
    }

    #[test]
    fn test_memmove_no_overlap() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        unsafe { my_memmove(dst.as_mut_ptr(), src.as_ptr(), 4) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memmove_overlap_forward() {
        // Copy buf[0..4] to buf[1..5], shifting right by 1
        // 将 buf[0..4] 复制到 buf[1..5]，向右移动 1 个位置
        let mut buf = [1u8, 2, 3, 4, 5];
        unsafe { my_memmove(buf.as_mut_ptr().add(1), buf.as_ptr(), 4) };
        assert_eq!(buf, [1, 1, 2, 3, 4]);
    }

    #[test]
    fn test_strlen_basic() {
        let s = b"hello\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 5);
    }

    #[test]
    fn test_strlen_empty() {
        let s = b"\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_equal() {
        let a = b"hello\0";
        let b = b"hello\0";
        assert_eq!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_less() {
        let a = b"abc\0";
        let b = b"abd\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } < 0);
    }

    #[test]
    fn test_strcmp_greater() {
        let a = b"abd\0";
        let b = b"abc\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } > 0);
    }
}
