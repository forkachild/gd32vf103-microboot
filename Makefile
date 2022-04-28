OUT_DIR:=target/riscv32imac-unknown-none-elf/release

build-release:
	cargo build --release

gen-artifacts: build-release
	riscv32-unknown-elf-objdump $(OUT_DIR)/app -D > $(OUT_DIR)/firmware.dump
	riscv32-unknown-elf-objcopy $(OUT_DIR)/app -O binary $(OUT_DIR)/firmware.bin

flash: gen-artifacts
	dfu-util -a 0 -d 28e9:0189 -s 0x8000000:leave -D $(OUT_DIR)/firmware.bin