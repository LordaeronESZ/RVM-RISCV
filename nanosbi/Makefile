# Arguments
MODE ?= debug

TARGET := riscv64.json
BUILD_ARGS := --target $(TARGET) -Zbuild-std=core,alloc \
	-Zbuild-std-features=compiler-builtins-mem -Zjson-target-spec

ifeq ($(MODE), release)
	BUILD_ARGS += --release
endif

# Build directory
BUILD_DIR := target/riscv64/$(MODE)
ELF := $(BUILD_DIR)/nanosbi
BIN := $(BUILD_DIR)/nanosbi.bin

# Toolchain
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump

all: build

build: $(BIN)

$(ELF):
	cargo build $(BUILD_ARGS)

$(BIN): $(ELF)
	$(OBJCOPY) -O binary $(ELF) $(BIN)

clean:
	cargo clean

disasm: $(ELF)
	$(OBJDUMP) -d $(ELF)

.PHONY: all build clean disasm