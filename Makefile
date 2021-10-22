.PHONY: all
	all: compile
compile:
	cargo bootimage

setup-env:
	rustup install nightly
	rustup component add rust-src --toolchain nightly
	rustup component add llvm-tools-preview --toolchain nightly
	rustup default nightly
	cargo install bootimage