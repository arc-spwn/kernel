#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// qemu debugging
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// panic handler that prints to serial out
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // this is very important
    match tests.len() {
        1 => serial_println!("Running {} test", tests.len()),
        _ => serial_println!("Running {} tests", tests.len()),
    }
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);

    // just to be sure
    loop { }
}

#[test_case]
fn trivial_assertion() {
    serial_print!("testing if tests work... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}



// panic handler that prints to vga buffer
#[cfg(not(test))]
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

    // dont clear in a test enviroment
    #[cfg(not(test))]
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