.PHONY: export-build-env invalidate-cache

NMKUP_PACKAGE := nmkup
NMK_PACKAGE := nmk

RELEASE_ENV := RUSTFLAGS="-C link-arg=-s"
RELEASE_BUILD := $(RELEASE_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf

.DEFAULT_GOAL := build

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = arm-linux-gnueabihf-gcc
export CLOUDSDK_ACTIVE_CONFIG_NAME = nui

export-build-env:
	$(eval export GIT_SHORT_SHA = $(shell git rev-parse --short HEAD))

build: invalidate-cache
	cargo build

invalidate-cache:
	touch src/bin/nmk/main.rs
	touch src/bin/nmkup/main.rs

release: invalidate-cache
	$(RELEASE_BUILD)
	du target/release/$(NMKUP_PACKAGE)

clean:
	cargo clean

cross-amd64-linux-musl: export-build-env invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(AMD64_LINUX_MUSL_TARGET)

cross-armv7-linux: export-build-env invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(ARMV7_LINUX_TARGET)

bundle-binary: invalidate-cache cross-amd64-linux-musl cross-armv7-linux
	$(eval UPLOAD_TMP_DIR = $(shell mktemp -d))
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmkup-amd64-linux-musl.xz
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMK_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmk-amd64-linux-musl.xz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE)			xz >	$(UPLOAD_TMP_DIR)/nmkup-armv7-linux.xz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMK_PACKAGE)			xz >	$(UPLOAD_TMP_DIR)/nmk-armv7-linux.xz
	cp target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)			$(UPLOAD_TMP_DIR)/nmkup-amd64-linux-musl
	cp target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE)				$(UPLOAD_TMP_DIR)/nmkup-armv7-linux
	gsutil -m -h "Cache-Control:no-cache,max-age=0,no-transform" cp $(UPLOAD_TMP_DIR)/* gs://nmk.nuimk.com/nmk.rs
	rm -rf $(UPLOAD_TMP_DIR)

