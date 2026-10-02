//! # Page Table Entry Flags
//! # 页表项标志位
//!
//! In this exercise, you will learn the structure of RISC-V SV39 Page Table Entry (PTE),
//! 在本练习中，你将学习 RISC-V SV39 页表项（PTE）的结构，
//! and construct/parse page table entries through bit operations.
//! 并通过位操作构造和解析页表项。
//!
//! ## Concepts
//! ## 概念
//! - RISC-V SV39 page table entry 64-bit layout
//! - RISC-V SV39 页表项的 64 位布局
//! - Bit operations to construct/extract fields
//! - 使用位操作构造/提取字段
//! - Meaning of PTE flags (V/R/W/X/U/G/A/D)
//! - PTE 标志位（V/R/W/X/U/G/A/D）的含义
//!
//! ## SV39 PTE Layout (64-bit)
//! ## SV39 PTE 布局（64 位）
//! ```text
//! 63    54 53        10 9  8 7 6 5 4 3 2 1 0
//! ┌───────┬────────────┬────┬─┬─┬─┬─┬─┬─┬─┬─┐
//! │ Rsvd  │  PPN[2:0]  │ RSW│D│A│G│U│X│W│R│V│
//! │ 10bit │  44 bits   │ 2b │ │ │ │ │ │ │ │ │
//! └───────┴────────────┴────┴─┴─┴─┴─┴─┴─┴─┴─┘
//! ```
//! - V (Valid): Valid bit indicating whether the page table entry is valid.
//! - V（有效）：指示页表项是否有效的有效位。
//!
//! - R/W/X (Read/Write/Execute): Permission bits for read, write, and execute access respectively.
//! - R/W/X（读/写/执行）：分别表示读、写和执行访问权限的权限位。
//!
//! - U (User): User-accessible bit, allowing access from user mode.
//! - U（用户）：允许从用户模式访问的用户可访问位。
//!
//! - G (Global): Global mapping bit (typically used for kernel space to avoid TLB flushes).
//! - G（全局）：全局映射位（通常用于内核空间以避免刷新 TLB）。
//!
//! - A (Accessed): Accessed bit, set by hardware when the page is accessed.
//! - A（已访问）：访问该页时由硬件设置的已访问位。
//!
//! - D (Dirty): Dirty bit, set by hardware when the page is written to.
//! - D（脏）：写入该页时由硬件设置的脏位。
//!
//! - RSW (Reserved for Supervisor Software): Two bits reserved for operating system software use.
//! - RSW（为监管者软件保留）：为操作系统软件用途保留的两位。
//!
//! - PPN (Physical Page Number): Physical page number occupying 44 bits (bits [53:10]), specifying the base address of the physical page frame.
//! - PPN（物理页号）：占据 44 位（位区间 `[53:10]`）的物理页号，用于指定物理页帧的基址。
//! - PPN[2:0] (Physical Page Number): In the RISC-V SV39 paging mechanism, the Physical Page Number (PPN) is divided into three parts, which are referred to as PPN[2], PPN[1], and PPN[0]. This division is designed to support the indexing of multi-level page tables.
//! - `PPN[2:0]`（物理页号）：在 RISC-V SV39 分页机制中，物理页号（PPN）分为三部分，分别称为 `PPN[2]`、`PPN[1]` 和 `PPN[0]`。这种划分旨在支持多级页表的索引。
//! - Rsvd (Reserved): Reserved bits, typically set to 0.
//! - Rsvd（保留）：保留位，通常设为 0。

/// PTE flag constants
/// PTE 标志常量
pub const PTE_V: u64 = 1 << 0; // Valid
// 有效（Valid）
pub const PTE_R: u64 = 1 << 1; // Readable
// 可读（Readable）
pub const PTE_W: u64 = 1 << 2; // Writable
// 可写（Writable）
pub const PTE_X: u64 = 1 << 3; // Executable
// 可执行（Executable）
pub const PTE_U: u64 = 1 << 4; // User accessible
// 用户可访问（User accessible）
pub const PTE_G: u64 = 1 << 5; // Global
// 全局（Global）
pub const PTE_A: u64 = 1 << 6; // Accessed
// 已访问（Accessed）
pub const PTE_D: u64 = 1 << 7; // Dirty
// 脏（Dirty）

/// PPN field offset and mask in PTE
/// PTE 中 PPN 字段的偏移和掩码
const PPN_SHIFT: u32 = 10;
const PPN_MASK: u64 = (1u64 << 44) - 1; // 44-bit PPN
// 44 位 PPN

/// Construct a page table entry from physical page number (PPN) and flags.
/// 根据物理页号（PPN）和标志位构造页表项。
///
/// PPN occupies bits [53:10], flags occupy bits [7:0].
/// PPN 占据位 `[53:10]`，标志位占据位 `[7:0]`。
///
/// Example: ppn=0x12345, flags=PTE_V|PTE_R|PTE_W
/// 示例：ppn=0x12345，flags=PTE_V|PTE_R|PTE_W
/// Result should be: (0x12345 << 10) | 0b111 = 0x48D14007
/// 结果应为：(0x12345 << 10) | 0b111 = 0x48D14007
///
/// Hint: Shift PPN left by PPN_SHIFT bits, then OR with flags.
/// 提示：将 PPN 左移 PPN_SHIFT 位，再与 flags 按位或。
pub fn make_pte(ppn: u64, flags: u64) -> u64 {
    // TODO: Construct page table entry using ppn and flags
    // TODO：根据 ppn 和 flags 构造页表项
    todo!()
}

/// Extract physical page number (PPN) from page table entry.
/// 从页表项中提取物理页号（PPN）。
///
/// Hint: Right shift by PPN_SHIFT bits, then AND with PPN_MASK.
/// 提示：右移 PPN_SHIFT 位，然后与 PPN_MASK 按位与。
pub fn extract_ppn(pte: u64) -> u64 {
    // TODO: Extract PPN from pte
    // TODO：从 pte 中提取 PPN
    todo!()
}

/// Extract flags (lower 8 bits) from page table entry.
/// 从页表项中提取标志位（低 8 位）。
pub fn extract_flags(pte: u64) -> u64 {
    // TODO: Extract lower 8-bit flags
    // TODO：提取低 8 位标志
    todo!()
}

/// Check whether page table entry is valid (V bit set).
/// 检查页表项是否有效（V 位已设置）。
pub fn is_valid(pte: u64) -> bool {
    // TODO: Check PTE_V
    // TODO：检查 PTE_V
    todo!()
}

/// Determine whether page table entry is a leaf PTE.
/// 判断页表项是否为叶子 PTE。
///
/// In SV39, if any of R, W, X bits is set, the PTE is a leaf,
/// 在 SV39 中，只要 R、W、X 位中任意一位已设置，该 PTE 就是叶子项，
/// pointing to the final physical page. Otherwise it points to next-level page table.
/// 指向最终物理页；否则它指向下一级页表。
pub fn is_leaf(pte: u64) -> bool {
    // TODO: Check if any of R/W/X bits is set
    // TODO：检查 R/W/X 位中是否有任意一位已设置
    todo!()
}

/// Check whether page table entry permits the requested access based on given permissions.
/// 根据给定权限检查页表项是否允许所请求的访问。
///
/// - `read`: requires read permission
/// - `read`：需要读权限
/// - `write`: requires write permission
/// - `write`：需要写权限
/// - `execute`: requires execute permission
/// - `execute`：需要执行权限
///
/// Returns true iff: PTE is valid, and each requested permission is satisfied.
/// 当且仅当页表项有效且满足每项请求的权限时返回 true。
pub fn check_permission(pte: u64, read: bool, write: bool, execute: bool) -> bool {
    // TODO: First check if valid, then check each requested permission
    // TODO：先检查有效性，再检查每项请求的权限
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_pte_basic() {
        let pte = make_pte(0x12345, PTE_V | PTE_R | PTE_W);
        assert_eq!(extract_ppn(pte), 0x12345);
        assert_eq!(extract_flags(pte), PTE_V | PTE_R | PTE_W);
    }

    #[test]
    fn test_make_pte_zero() {
        let pte = make_pte(0, 0);
        assert_eq!(pte, 0);
        assert_eq!(extract_ppn(pte), 0);
        assert_eq!(extract_flags(pte), 0);
    }

    #[test]
    fn test_make_pte_all_flags() {
        let all = PTE_V | PTE_R | PTE_W | PTE_X | PTE_U | PTE_G | PTE_A | PTE_D;
        let pte = make_pte(0xABC, all);
        assert_eq!(extract_ppn(pte), 0xABC);
        assert_eq!(extract_flags(pte), all);
    }

    #[test]
    fn test_make_pte_large_ppn() {
        let ppn = (1u64 << 44) - 1; // maximum PPN
        // 最大 PPN 值
        let pte = make_pte(ppn, PTE_V);
        assert_eq!(extract_ppn(pte), ppn);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid(make_pte(1, PTE_V)));
        assert!(!is_valid(make_pte(1, PTE_R))); // R set but V not set
        // 已设置 R，但未设置 V
        assert!(!is_valid(0));
    }

    #[test]
    fn test_is_leaf() {
        assert!(is_leaf(make_pte(1, PTE_V | PTE_R)));
        assert!(is_leaf(make_pte(1, PTE_V | PTE_X)));
        assert!(is_leaf(make_pte(1, PTE_V | PTE_R | PTE_W | PTE_X)));
        // Non-leaf: only V set, R/W/X all cleared
        // 非叶子项：仅设置 V，R/W/X 均已清除
        assert!(!is_leaf(make_pte(1, PTE_V)));
        assert!(!is_leaf(make_pte(1, PTE_V | PTE_A | PTE_D)));
    }

    #[test]
    fn test_check_permission_read() {
        let pte = make_pte(1, PTE_V | PTE_R);
        assert!(check_permission(pte, true, false, false));
        assert!(!check_permission(pte, false, true, false));
        assert!(!check_permission(pte, false, false, true));
    }

    #[test]
    fn test_check_permission_rw() {
        let pte = make_pte(1, PTE_V | PTE_R | PTE_W);
        assert!(check_permission(pte, true, true, false));
        assert!(!check_permission(pte, true, true, true));
    }

    #[test]
    fn test_check_permission_all() {
        let pte = make_pte(1, PTE_V | PTE_R | PTE_W | PTE_X);
        assert!(check_permission(pte, true, true, true));
        assert!(check_permission(pte, true, false, false));
        assert!(check_permission(pte, false, false, false)); // no requirement = OK
        // 未要求任何权限 = 允许
    }

    #[test]
    fn test_check_permission_invalid() {
        // V not set, should return false even if R/W/X flags present
        // 未设置 V；即使存在 R/W/X 标志，也应返回 false
        let pte = make_pte(1, PTE_R | PTE_W | PTE_X);
        assert!(!check_permission(pte, true, false, false));
    }
}
