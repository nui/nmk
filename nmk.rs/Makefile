BIN_NAME := nmk
RELEASE_BIN := release/$(BIN_NAME)
DIST_DIR ?= dist
EPOCHSECONDS := $(shell date +%s)
BUILD_ENV := EPOCHSECONDS=$(EPOCHSECONDS)

BUILD := $(BUILD_ENV) cargo build

ALPINE_TARGET := x86_64-unknown-linux-musl
ALPINE_BIN := target/$(ALPINE_TARGET)/$(RELEASE_BIN)
ALPINE_OUT := $(DIST_DIR)/nmk.rs-amd64-linux-musl

ARM_TARGET := armv7-unknown-linux-gnueabihf

all: build

arm:
	$(BUILD_ENV) cross build --target $(ARM_TARGET) --release

build:
	$(BUILD)

clean:
	cargo clean

x86_alpine:
	$(BUILD) --release --target $(ALPINE_TARGET)
	cp $(ALPINE_BIN) $(ALPINE_OUT)
	strip $(ALPINE_OUT)
	gzip -k $(ALPINE_OUT)

multiarch: x86_alpine

