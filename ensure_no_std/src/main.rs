#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

use alloc::string::{String, ToString};
use dfmt::dformat;

use core::arch::asm;

fn syscall_exit(code: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
            options(noreturn)
        );
    }
}

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    assert_eq!(
        dformat!("Hello, {world}!".to_string(), world = "World").unwrap(),
        "Hello, World!".to_string(),
    );

    syscall_exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    syscall_exit(1);
}
