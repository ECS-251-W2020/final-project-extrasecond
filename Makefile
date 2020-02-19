TARGET            	= aarch64-unknown-none-softfloat
OUTPUT            	= kernel8.img
QEMU_BINARY       	= qemu-system-aarch64
QEMU_MACHINE_TYPE 	= raspi3
QEMU_RELEASE_ARGS 	= -serial stdio -display none
LINKER_FILE       	= src/link.ld
RUSTC_MISC_ARGS   	= -C target-cpu=cortex-a53

RUSTFLAGS          	= -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC 	= $(RUSTFLAGS) -D warnings

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) $(wildcard **/*.ld)

XRUSTC_CMD = cargo xrustc     \
	--target=$(TARGET)    \
	--features bsp_rpi3 \
	--release

CARGO_OUTPUT = target/$(TARGET)/release/ritos

OBJCOPY_CMD = cargo objcopy \
	--                  \
	--strip-all         \
	-O binary

.PHONY: all doc qemu clippy clean readelf objdump nm

all: clean $(OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(XRUSTC_CMD)

$(OUTPUT): $(CARGO_OUTPUT)
	cp $< .
	$(OBJCOPY_CMD) $< $(OUTPUT)

clean:
	rm -rf target