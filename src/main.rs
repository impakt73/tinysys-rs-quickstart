#![no_std]
#![no_main]
#![allow(static_mut_refs)]

extern crate panic_halt;

use alloc::alloc::alloc;
use riscv as _;

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use tinysys_sys::*;

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe { alloc(alloc::alloc::Layout::from_size_align_unchecked(size, 8)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4 * 1024 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    unsafe {
        let kernel_ctx = VPUGetKernelGfxContext();
        let task_ctx = TaskGetContext(0);

        let msg = b"Hello World!\n";
        VPUConsolePrint(kernel_ctx, msg.as_ptr(), msg.len() as i32);

        TaskExitCurrentTask(task_ctx);
    }
}
