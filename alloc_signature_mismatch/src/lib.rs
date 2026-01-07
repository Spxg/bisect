#![no_std]

extern crate alloc;

use mini_alloc::MiniAlloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: MiniAlloc = MiniAlloc::INIT;

#[unsafe(no_mangle)]
extern "C" fn foo(x: i32) -> usize {
    let mut v = alloc::vec::Vec::new();
    v.push(x);
    v.len()
}
