.PHONY: all
	all: amogus

amogus:
	@echo ඞ amogus


compile:
	cargo bootimage

setup-env:
	rustup install nightly
	rustup component add rust-src --toolchain nightly
	rustup component add llvm-tools-preview --toolchain nightly
	rustup default nightly
	cargo install bootimage

flash:
ifdef DRIVE 
	@echo "flashing to /dev/$(DRIVE)" 
	sudo dd if=target/x86_64-baremetal/debug/bootimage-kernel.bin of=/dev/$(DRIVE)
else 
	@echo "DRIVE wasnt found, go into your Makefile and set DRIVE to your drives name (no /dev/ prefix)"
endif 

qemu:
	cargo run

run:
	cargo run

r:
	cargo run

test:
	cargo t

t:
	cargo t


