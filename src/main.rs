#![no_std]
#![no_main]

mod vga_buffer;
mod recovery;

use core::panic::PanicInfo;

use ps2::{Controller, flags::ControllerConfigFlags};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start() -> ! {;
    // diffrent keyboard system soon tm
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