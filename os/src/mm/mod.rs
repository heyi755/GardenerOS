mod heap_allocator;
mod address;
mod frame_allocator;
mod page_table;


use page_table::{PageTable, PTEFlags};
use address::{VPNRange, StepByOne};
pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use page_table::{PageTableEntry, translated_byte_buffer};


pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
}