#![no_std]
#![no_main]

mod sbi;
mod console;

use core::arch::asm;
use core::panic::PanicInfo;
use core::arch::global_asm;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;
global_asm!(include_str!("entry.asm"));



fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!("ecall",
             in("x10") args[0],
             in("x11") args[1],
             in("x12") args[2],
             in("x17") id,
             lateout("x10") ret
        );
    }
    ret
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}


pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() {
    //loop{};
    println!("Hello, world!");
    sys_exit(9);
}

#[no_mangle]
pub fn rust_main() -> ! {
    loop{};
}
