//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

use crate::config::PAGE_SIZE_BITS;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    // 初始化内核堆
    heap_allocator::init_heap();
    // 收集剩余可用内存，并分配为物理帧集合
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}


/// translated VirtAddr to PhysAddr
// seems va_ptr as VirtAddr , has a bug ? drop ?
pub fn translated_va_to_pa(token: usize, va_ptr: usize) -> PhysAddr {
    let page_table = PageTable::from_token(token);
    let va = VirtAddr::from(va_ptr);
    let ppn = page_table.translate(va.floor()).unwrap().ppn().0;
    PhysAddr((ppn << PAGE_SIZE_BITS) + va.page_offset())
}


/// get current user page table
pub fn current_user_table() -> PageTable {
    PageTable::from_token(crate::task::current_user_token())
}
