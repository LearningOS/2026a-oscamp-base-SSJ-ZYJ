//! # Process and Pipes
//! # 进程与管道
//!
//! In this exercise, you will learn how to create child processes and communicate through pipes.
//! 在本练习中，你将学习如何创建子进程并通过管道进行通信。
//!
//! ## Concepts
//! ## 概念
//! - `std::process::Command` creates child processes (corresponds to `fork()` + `execve()` system calls)
//! - `std::process::Command` 创建子进程（对应 `fork()` + `execve()` 系统调用）
//! - `Stdio::piped()` sets up pipes (corresponds to `pipe()` + `dup2()` system calls)
//! - `Stdio::piped()` 设置管道（对应 `pipe()` + `dup2()` 系统调用）
//! - Communicate with child processes via stdin/stdout
//! - 通过 stdin/stdout 与子进程通信
//! - Obtain child process exit status (corresponds to `waitpid()` system call)
//! - 获取子进程退出状态（对应 `waitpid()` 系统调用）
//!
//! ## OS Concepts Mapping
//! ## 操作系统概念映射
//! This exercise demonstrates user‑space abstractions over underlying OS primitives:
//! 本练习展示了用户空间抽象如何建立在底层操作系统原语之上：
//! - **Process creation**: Rust's `Command::new()` internally invokes `fork()` to create a child process,
//! - **进程创建**：Rust 的 `Command::new()` 内部调用 `fork()` 创建子进程，
//!   then `execve()` (or equivalent) to replace the child's memory image with the target program.
//!   然后调用 `execve()`（或等效操作）将子进程的内存映像替换为目标程序。
//! - **Inter‑process communication (IPC)**: Pipes are kernel‑managed buffers that allow one‑way data
//! - **进程间通信（IPC）**：管道是由内核管理的缓冲区，允许相关进程之间进行单向数据
//!   flow between related processes. The `pipe()` system call creates a pipe, returning two file
//!   流动。`pipe()` 系统调用创建管道并返回两个文件描述符（读端、写端）。`dup2()` 复制文件描述符，从而可以重定向
//!   descriptors (read end, write end). `dup2()` duplicates a file descriptor, enabling redirection
//!   标准输入/输出。
//!   of standard input/output.
//! - **Resource management**: File descriptors (including pipe ends) are automatically closed when
//! - **资源管理**：文件描述符（包括管道端点）会在其 Rust `Stdio` 对象被释放时自动关闭，
//!   their Rust `Stdio` objects are dropped, preventing resource leaks.
//!   从而避免资源泄漏。
//!
//! ## Exercise Structure
//! ## 练习结构
//! 1. **Basic command execution** (`run_command`) – launch a child process and capture its stdout.
//! 1. **基本命令执行**（`run_command`）——启动子进程并捕获其 stdout。
//! 2. **Bidirectional pipe communication** (`pipe_through_cat`) – send data to a child process (`cat`)
//! 2. **双向管道通信**（`pipe_through_cat`）——向子进程（`cat`）发送数据
//!    and read its output.
//!    并读取其输出。
//! 3. **Exit code retrieval** (`get_exit_code`) – obtain the termination status of a child process.
//! 3. **获取退出码**（`get_exit_code`）——获取子进程的终止状态。
//! 4. **Advanced: error‑handling version** (`run_command_with_result`) – learn proper error propagation.
//! 4. **进阶：错误处理版本**（`run_command_with_result`）——学习正确传播错误。
//! 5. **Advanced: complex bidirectional communication** (`pipe_through_grep`) – interact with a filter
//! 5. **进阶：复杂的双向通信**（`pipe_through_grep`）——与读取多行并产生筛选结果的程序交互。
//!    program that reads multiple lines and produces filtered output.
//!    程序。
//!
//! Each function includes a `TODO` comment indicating where you need to write code.
//! 每个函数都包含一个 `TODO` 注释，指出你需要编写代码的位置。
//! Run `cargo test` to check your implementations.
//! 运行 `cargo test` 检查你的实现。

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

/// Execute the given shell command and return its stdout output.
/// 执行给定的 shell 命令并返回其 stdout 输出。
///
/// For example: `run_command("echo", &["hello"])` should return `"hello\n"`
/// 例如：`run_command("echo", &["hello"])` 应返回 `"hello\n"`
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new(program)` → `fork()` + `execve()` family
/// - `Command::new(program)` → `fork()` + `execve()` 系列调用
/// - `Stdio::piped()` → `pipe()` + `dup2()` (sets up a pipe for stdout)
/// - `Stdio::piped()` → `pipe()` + `dup2()`（为 stdout 设置管道）
/// - `.output()` → `waitpid()` (waits for child process termination)
/// - `.output()` → `waitpid()`（等待子进程终止）
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` with the given program and arguments.
/// 1. 使用给定的程序和参数创建一个 `Command`。
/// 2. Set `.stdout(Stdio::piped())` to capture the child's stdout.
/// 2. 设置 `.stdout(Stdio::piped())` 以捕获子进程的 stdout。
/// 3. Call `.output()` to execute the child and obtain its `Output`.
/// 3. 调用 `.output()` 执行子进程并获取其 `Output`。
/// 4. Convert the `stdout` field (a `Vec<u8>`) into a `String`.
/// 4. 将 `stdout` 字段（一个 `Vec<u8>`）转换为 `String`。
pub fn run_command(program: &str, args: &[&str]) -> String {
    // TODO: Use Command::new to create process
    // TODO：使用 Command::new 创建进程
    // TODO: Set stdout to Stdio::piped()
    // TODO：将 stdout 设置为 Stdio::piped()
    // TODO: Execute with .output() and get output
    // TODO：调用 .output() 执行并获取输出
    // TODO: Convert stdout to String and return
    // TODO：将 stdout 转为 String 并返回
    todo!()
}

/// Write data to child process (cat) stdin via pipe and read its stdout output.
/// 通过管道将数据写入子进程（cat）的 stdin，并读取其 stdout 输出。
///
/// This demonstrates bidirectional pipe communication between parent and child processes.
/// 这展示了父子进程之间的双向管道通信。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("cat")` → `fork()` + `execve("cat")`
/// - `Stdio::piped()` (twice) → `pipe()` creates two pipes (stdin & stdout) + `dup2()` redirects them
/// - `Stdio::piped()`（两次）→ `pipe()` 创建两条管道（stdin 和 stdout）并用 `dup2()` 重定向
/// - `ChildStdin::write_all()` → `write()` to the pipe's write end
/// - `ChildStdin::write_all()` → 向管道写端执行 `write()`
/// - `drop(stdin)` → `close()` on the write end, sending EOF to child
/// - `drop(stdin)` → 对写端执行 `close()`，向子进程发送 EOF
/// - `ChildStdout::read_to_string()` → `read()` from the pipe's read end
/// - `ChildStdout::read_to_string()` → 从管道读端执行 `read()`
///
/// # Ownership and Resource Management
/// # 所有权与资源管理
/// Rust's ownership system ensures pipes are closed at the right time:
/// Rust 的所有权系统确保管道会在恰当时机关闭：
/// 1. The `ChildStdin` handle is owned by the parent; writing to it transfers data to the child.
/// 1. `ChildStdin` 句柄由父进程持有；向其写入会将数据传给子进程。
/// 2. After writing, we explicitly `drop(stdin)` (or let it go out of scope) to close the write end.
/// 2. 写入后，我们显式调用 `drop(stdin)`（或让它离开作用域）以关闭写端。
/// 3. Closing the write end signals EOF to `cat`, causing it to exit after processing all input.
/// 3. 关闭写端会向 `cat` 发送 EOF，使其在处理完所有输入后退出。
/// 4. The `ChildStdout` handle is then read to completion; dropping it closes the read end.
/// 4. 随后读取 `ChildStdout` 句柄直到读完；释放它会关闭读端。
///
/// Without dropping `stdin`, the child would wait forever for more input (pipe never closes).
/// 如果不释放 `stdin`，子进程会一直等待更多输入（管道始终不会关闭）。
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"cat"` with `.stdin(Stdio::piped())` and `.stdout(Stdio::piped())`.
/// 1. 使用 `.stdin(Stdio::piped())` 和 `.stdout(Stdio::piped())` 为 `"cat"` 创建 `Command`。
/// 2. `.spawn()` the command to obtain a `Child` with `stdin` and `stdout` handles.
/// 2. 对命令调用 `.spawn()`，以获取带有 `stdin` 和 `stdout` 句柄的 `Child`。
/// 3. Write `input` bytes to the child's stdin (`child.stdin.take().unwrap().write_all(...)`).
/// 3. 将 `input` 字节写入子进程的 stdin（`child.stdin.take().unwrap().write_all(...)`）。
/// 4. Drop the stdin handle (explicit `drop` or let it go out of scope) to close the pipe.
/// 4. 释放 stdin 句柄（显式调用 `drop` 或让它离开作用域）以关闭管道。
/// 5. Read the child's stdout (`child.stdout.take().unwrap().read_to_string(...)`).
/// 5. 读取子进程的 stdout（`child.stdout.take().unwrap().read_to_string(...)`）。
/// 6. Wait for the child to exit with `.wait()` (or rely on drop‑wait).
/// 6. 调用 `.wait()` 等待子进程退出（或依赖释放时等待）。
pub fn pipe_through_cat(input: &str) -> String {
    // TODO: Create "cat" command, set stdin and stdout to piped
    // TODO：创建 "cat" 命令，并将 stdin 和 stdout 设置为管道
    // TODO: Spawn process
    // TODO：启动进程
    // TODO: Write input to child process stdin
    // TODO：将输入写入子进程的 stdin
    // TODO: Drop stdin to close pipe (otherwise cat won't exit)
    // TODO：关闭管道并释放 stdin（否则 cat 不会退出）
    // TODO: Read output from child process stdout
    // TODO：读取子进程的 stdout 输出
    todo!()
}

/// Get child process exit code.
/// 获取子进程退出码。
/// Execute command `sh -c {command}` and return the exit code.
/// 执行命令 `sh -c {command}` 并返回退出码。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("sh")` → `fork()` + `execve("/bin/sh")`
/// - `.args(["-c", command])` passes the shell command line
/// - `.args(["-c", command])` 传入 shell 命令行
/// - `.status()` → `waitpid()` (waits for child and retrieves exit status)
/// - `.status()` → `waitpid()`（等待子进程并获取退出状态）
/// - `ExitStatus::code()` extracts the low‑byte exit code (0‑255)
/// - `ExitStatus::code()` 提取低字节退出码（0–255）
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"sh"` with arguments `["-c", command]`.
/// 1. 创建一个参数为 `["-c", command]` 的 `"sh"` 命令。
/// 2. Call `.status()` to execute the shell and obtain an `ExitStatus`.
/// 2. 调用 `.status()` 执行 shell 并获取 `ExitStatus`。
/// 3. Use `.code()` to get the exit code as `Option<i32>`.
/// 3. 使用 `.code()` 获取 `Option<i32>` 类型的退出码。
/// 4. If the child terminated normally, return the exit code; otherwise return a default.
/// 4. 如果子进程正常终止，则返回退出码；否则返回默认值。
pub fn get_exit_code(command: &str) -> i32 {
    // TODO: Use Command::new("sh").args(["-c", command])
    // TODO：使用 Command::new("sh").args(["-c", command])
    // TODO: Execute and get status
    // TODO：执行命令并获取状态
    // TODO: Return exit code
    // TODO：返回退出码
    todo!()
}

/// Execute the given shell command and return its stdout output as a `Result`.
/// 执行给定的 shell 命令，并以 `Result` 返回其 stdout 输出。
///
/// This version properly propagates errors that may occur during process creation,
/// 此版本会正确传播进程创建、执行或 I/O 过程中可能发生的错误
/// execution, or I/O (e.g., command not found, permission denied, broken pipe).
///（例如命令不存在、权限被拒绝、管道断开）。
///
/// # Underlying System Calls
/// # 底层系统调用
/// Same as `run_command`, but errors are captured from the OS and returned as `Err`.
/// 与 `run_command` 相同，但会捕获操作系统错误并作为 `Err` 返回。
///
/// # Error Handling
/// # 错误处理
/// - `Command::new()` only constructs the builder; errors occur at `.output()`.
/// - `Command::new()` 只构造 builder；错误会在调用 `.output()` 时发生。
/// - `.output()` returns `Result<Output, std::io::Error>`.
/// - `.output()` 返回 `Result<Output, std::io::Error>`。
/// - `String::from_utf8()` may fail if the child's output is not valid UTF‑8.
/// - 如果子进程的输出不是有效 UTF-8，`String::from_utf8()` 可能失败。
///   In that case we return an `io::Error` with kind `InvalidData`.
///   此时我们会返回一个种类为 `InvalidData` 的 `io::Error`。
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` with the given program and arguments.
/// 1. 使用给定的程序和参数创建一个 `Command`。
/// 2. Set `.stdout(Stdio::piped())`.
/// 2. 设置 `.stdout(Stdio::piped())`。
/// 3. Call `.output()` and propagate any `io::Error`.
/// 3. 调用 `.output()` 并传播任何 `io::Error`。
/// 4. Convert `stdout` to `String` with `String::from_utf8`; if that fails, map to an `io::Error`.
/// 4. 使用 `String::from_utf8` 将 `stdout` 转换为 `String`；若失败，则将错误映射为 `io::Error`。
pub fn run_command_with_result(program: &str, args: &[&str]) -> io::Result<String> {
    // TODO: Use Command::new to create process
    // TODO：使用 Command::new 创建进程
    // TODO: Set stdout to Stdio::piped()
    // TODO：将 stdout 设置为 Stdio::piped()
    // TODO: Execute with .output() and handle Result
    // TODO：调用 .output() 并处理 Result
    // TODO: Convert stdout to String with from_utf8, mapping errors to io::Error
    // TODO：使用 from_utf8 将 stdout 转为 String，并将错误映射为 io::Error
    todo!()
}

/// Interact with `grep` via bidirectional pipes, filtering lines that contain a pattern.
/// 通过双向管道与 `grep` 交互，筛选包含某个模式的行。
///
/// This demonstrates complex parent‑child communication: the parent sends multiple
/// 这展示了复杂的父子进程通信：父进程发送多行
/// lines of input, the child (`grep`) filters them according to a pattern, and the
/// 输入，子进程（`grep`）根据模式筛选这些行，然后
/// parent reads back only the matching lines.
/// 父进程读取并仅获得匹配的行。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("grep")` → `fork()` + `execve("grep")`
/// - Two pipes (stdin & stdout) as in `pipe_through_cat`
/// - 与 `pipe_through_cat` 相同，使用两条管道（stdin 和 stdout）
/// - Line‑by‑line writing and reading to simulate interactive filtering
/// - 逐行写入和读取，以模拟交互式筛选
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"grep"` with argument `pattern`, and both ends piped.
/// 1. 使用参数 `pattern` 创建 `"grep"` 命令，并为两端设置管道。
/// 2. `.spawn()` the command, obtaining `Child` with `stdin` and `stdout` handles.
/// 2. 调用 `.spawn()` 命令，获取带有 `stdin` 和 `stdout` 句柄的 `Child`。
/// 3. Write each line of `input` (separated by `'\n'`) to the child's stdin.
/// 3. 将 `input` 中的每一行（以 `'\n'` 分隔）写入子进程的 stdin。
/// 4. Close the write end (drop stdin) to signal EOF.
/// 4. 关闭写端（释放 stdin）以发送 EOF。
/// 5. Read the child's stdout line by line, collecting matching lines.
/// 5. 逐行读取子进程的 stdout，并收集匹配的行。
/// 6. Wait for the child to exit (optional; `grep` exits after EOF).
/// 6. 等待子进程退出（可选；`grep` 会在收到 EOF 后退出）。
/// 7. Return the concatenated matching lines as a single `String`.
/// 7. 将拼接后的匹配行作为单个 `String` 返回。
///
pub fn pipe_through_grep(pattern: &str, input: &str) -> String {
    // TODO: Create "grep" command with pattern, set stdin and stdout to piped
    // TODO：创建带有 pattern 的 "grep" 命令，并将 stdin 和 stdout 设置为管道
    // TODO: Spawn process
    // TODO：启动进程
    // TODO: Write input lines to child stdin
    // TODO：将输入行写入子进程的 stdin
    // TODO: Drop stdin to close pipe
    // TODO：关闭 stdin 以关闭管道
    // TODO: Read output from child stdout line by line
    // TODO：逐行读取子进程的 stdout
    // TODO: Collect and return matching lines
    // TODO：收集并返回匹配的行
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        let output = run_command("echo", &["hello"]);
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_with_args() {
        let output = run_command("echo", &["-n", "no newline"]);
        assert_eq!(output, "no newline");
    }

    #[test]
    fn test_pipe_cat() {
        let output = pipe_through_cat("hello pipe!");
        assert_eq!(output, "hello pipe!");
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line1\nline2\nline3";
        assert_eq!(pipe_through_cat(input), input);
    }

    #[test]
    fn test_exit_code_success() {
        assert_eq!(get_exit_code("true"), 0);
    }

    #[test]
    fn test_exit_code_failure() {
        assert_eq!(get_exit_code("false"), 1);
    }

    #[test]
    fn test_run_command_with_result_success() {
        let result = run_command_with_result("echo", &["hello"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello");
    }

    #[test]
    fn test_run_command_with_result_nonexistent() {
        let result = run_command_with_result("nonexistent_command_xyz", &[]);
        // Should be an error because command not found
        // 命令不存在时应返回错误
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_through_grep_basic() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("a", input);
        // grep outputs matching lines with newline
        // grep 会为匹配行附加换行符
        assert_eq!(output, "apple\nbanana\n");
    }

    #[test]
    fn test_pipe_through_grep_no_match() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("z", input);
        // No lines match -> empty string
        // 没有匹配行时返回空字符串
        assert_eq!(output, "");
    }

    #[test]
    fn test_pipe_through_grep_multiline() {
        let input = "first line\nsecond line\nthird line\n";
        let output = pipe_through_grep("second", input);
        assert_eq!(output, "second line\n");
    }
}
