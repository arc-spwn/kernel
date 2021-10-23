#![no_std]
#![no_main]

// if someone knows how to do this without this pathy ass code hmu
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start() -> ! {
    println!("Sussy balls :OOOOOOOOO");
    panic!("too much cringe");

    loop { }
}