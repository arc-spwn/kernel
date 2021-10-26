#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod serial;
mod interrupts;
mod gdt;
mod keyboard;

use core::panic::PanicInfo;

use kernel::test_runner;
use kernel::hlt_loop;

// panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    // diffrent keyboard system soon tm

    kernel::init();
    
    #[cfg(not(debug_assertions))]
    vga_buffer::clear_scr();

    println!("
        ..'',,,,'...                        
        .,:cloooooooolc:'.                 .,,,,,.
     .;looooooooooooolllc,..             .,c:::c,.
    .cooooooolc:ccloollllllc,.           .:c:::c,
    .:ooooooc,..  ..,cllllllll:.         .,::::::'
    ,looooo:.        .'clllllllc;'.    ..;::::::;.
    ;oooooc.           .,cllllllclc:;;;:cc:::::;.
    .:ooolo:.             .;clllccccccccccc::::'.
    ':::::.                ..,:clccccccccc:;'. 
                              ..',,,;,,,'..  
                              ");

    println!("Arc loaded.");

    hlt_loop();
}