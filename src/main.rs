#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

use ps2::{Controller, flags::ControllerConfigFlags};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start() -> ! {

    println!("loading keyboard...");
    
    let mut controller = unsafe { Controller::new() };

    // disable devices because apparently you need to do that
    controller.disable_keyboard().unwrap();
    let _ = controller.read_data();
    
    // configure only keyboard
    let mut config = controller.read_config().unwrap();
    config.set(
        ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT
        | ControllerConfigFlags::DISABLE_MOUSE
        | ControllerConfigFlags::ENABLE_TRANSLATE,
        false,
    );
    controller.write_config(config).unwrap();
    controller.test_controller().unwrap();
    // rewrite config if a reset happened
    controller.write_config(config);

    // test the keyboard
    let keyboard_ok_question_mark = controller.test_keyboard().is_ok();
    config = controller.read_config().unwrap();
    if keyboard_ok_question_mark {
        controller.enable_keyboard().unwrap();
        config.set(ControllerConfigFlags::DISABLE_KEYBOARD, false);
        config.set(ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT, true);
        controller.keyboard().reset_and_self_test().unwrap();
    } else {
        panic!("failed to initialize your keyboard, quitting...");
    }

    let mut keyboard = controller.keyboard();

    println!("keyboard loaded.");

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
    println!("there will be more here soon");

    

    loop { }
}