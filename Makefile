TARGET            	= aarch64-unknown-none-softfloat
OUTPUT            	= kernel8.img
QEMU_BINARY       	= qemu-system-aarch64
QEMU_MACHINE_TYPE 	= raspi3
QEMU_RELEASE_ARGS 	= -serial stdio -display none
LINKER_FILE       	= src/bsp/rpi/link.ld
RUSTC_MISC_ARGS   	= -C target-cpu=cortex-a53

RUSTFLAGS          	= -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC 	= $(RUSTFLAGS) -D warnings

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
DOCKER_EXEC_QEMU     = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)

.PHONY: all doc qemu clippy clean readelf objdump nm

all: clean $(OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(XRUSTC_CMD)

$(OUTPUT): $(CARGO_OUTPUT)
	cp $< .
	$(OBJCOPY_CMD) $< $(OUTPUT)

docker: all
	@$(DOCKER_CMD) $(DOCKER_ARG_DIR_TUT) $(DOCKER_IMAGE) \
		$(DOCKER_EXEC_QEMU) $(QEMU_RELEASE_ARGS)     \
		-kernel $(OUTPUT)

qemu: all
	$(DOCKER_EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(OUTPUT)

clean:
	rm -rf target