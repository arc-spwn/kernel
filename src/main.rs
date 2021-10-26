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

use core::panic::PanicInfo;

use kernel::test_runner;

// panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    // diffrent keyboard system soon tm

    print!("loading idt and gdt...");
    kernel::init();
    println!(" [ok]");
    
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
    loop { }
}