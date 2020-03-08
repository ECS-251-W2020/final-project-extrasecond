ifndef DEV_SERIAL
	DEV_SERIAL = /dev/ttyUSB0
endif

TARGET            	= aarch64-unknown-none-softfloat
OUTPUT            	= kernel8.img
QEMU_BINARY       	= qemu-system-aarch64
QEMU_MACHINE_TYPE 	= raspi3
QEMU_RELEASE_ARGS 	= -serial stdio -display none
LINKER_FILE       = src/bsp/rpi/link.ld
RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53 -C relocation-model=pic

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) $(wildcard **/*.ld)

XRUSTC_CMD = cargo xrustc     \
	--target=$(TARGET)    \
	--release

CARGO_OUTPUT = target/$(TARGET)/release/ritos

OBJCOPY_CMD = cargo objcopy \
	--                  \
	--strip-all         \
	-O binary
	
DOCKER_IMAGE         = rustembedded/osdev-utils
DOCKER_CMD           = docker run -it --rm
DOCKER_ARG_DIR_TUT   = -v $(shell pwd):/work -w /work
DOCKER_ARG_DIR_UTILS = -v $(shell pwd)/utils:/utils
DOCKER_ARG_TTY       = --privileged -v /dev:/dev
DOCKER_EXEC_QEMU     = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)
DOCKER_EXEC_MINIPUSH = ruby /utils/minipush.rb

.PHONY: all doc qemu chainboot jtagboot openocd gdb gdb-opt0 clippy clean readelf objdump nm

all: clean $(OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(XRUSTC_CMD)

$(OUTPUT): $(CARGO_OUTPUT)
	cp $< .
	$(OBJCOPY_CMD) $< $(OUTPUT)

qemu: all
	$(DOCKER_EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(OUTPUT)

docker: all
	@$(DOCKER_CMD) $(DOCKER_ARG_DIR_TUT) $(DOCKER_IMAGE) \
		$(DOCKER_EXEC_QEMU) $(QEMU_RELEASE_ARGS)     \
		-kernel $(OUTPUT)

qemuasm: all
	@$(DOCKER_CMD) $(DOCKER_ARG_DIR_TUT) $(DOCKER_IMAGE) \
		$(DOCKER_EXEC_QEMU) $(QEMU_RELEASE_ARGS)     \
		-kernel $(OUTPUT) -d in_asm

chainboot: all
	@$(DOCKER_CMD) $(DOCKER_ARG_DIR_TUT) $(DOCKER_ARG_DIR_UTILS) $(DOCKER_ARG_TTY) \
		$(DOCKER_IMAGE) $(DOCKER_EXEC_MINIPUSH) $(DEV_SERIAL)                  \
		$(OUTPUT)

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" cargo xclippy --target=$(TARGET) --features bsp_$(BSP)

clean:
	rm -rf target

readelf:
	readelf -a ritos

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -no-show-raw-insn -print-imm-hex ritos

