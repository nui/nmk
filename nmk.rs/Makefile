ALPINE_TARGET := x86_64-unknown-linux-musl
BIN_NAME := nmkrs
RELEASE_BIN := release/$(BIN_NAME)
DIST_DIR ?= dist
EPOCHSECONDS := $(shell date +%s)
BUILD_ENV := EPOCHSECONDS=$(EPOCHSECONDS)

BUILD := $(BUILD_ENV) cargo build

all: build

build:
	$(BUILD)

multiarch:
	$(BUILD) --release --target $(ALPINE_TARGET)
	cp target/$(ALPINE_TARGET)/$(RELEASE_BIN) $(DIST_DIR)/nmk.rs-amd64-linux-musl
	gzip -k $(DIST_DIR)/nmk.rs-amd64-linux-musl
