.PHONY: dependency

BINARIES ?= nmk

# Common variables
PROJECT_DIR :=	$(PWD)/src/nmk
SOURCES :=	$(shell find $(PROJECT_DIR) -type f -name "*.go")
GOBIN :=	$(PWD)/bin
GOENV ?=	GO15VENDOREXPERIMENT=1 GOPATH=$(PWD)
GO ?=		$(GOENV) go
BUILDSTRIP ?=	$(GOENV) go build -ldflags "-w"
GODEP ?=	$(GOENV) dep

all: build

dependency:
	cd $(PROJECT_DIR) && $(GODEP) ensure

$(BINARIES): $(SOURCES)
	$(GO) build -o ./build/$@ $(PROJECT_DIR)/cmd/$@

build:	dependency $(BINARIES)

clean:
	rm -f ./build/*

multiarch: clean dependency
	GOARCH=amd64 GOOS=darwin  $(BUILDSTRIP) -o ./build/nmk-darwin-amd64  $(PROJECT_DIR)/cmd/nmk
	GOARCH=amd64 GOOS=freebsd $(BUILDSTRIP) -o ./build/nmk-freebsd-amd64 $(PROJECT_DIR)/cmd/nmk
	GOARCH=amd64 GOOS=linux   $(BUILDSTRIP) -o ./build/nmk-linux-amd64   $(PROJECT_DIR)/cmd/nmk
	GOARCH=arm GOOS=linux     $(BUILDSTRIP) -o ./build/nmk-linux-arm     $(PROJECT_DIR)/cmd/nmk
	GOARCH=arm64 GOOS=linux   $(BUILDSTRIP) -o ./build/nmk-linux-arm64   $(PROJECT_DIR)/cmd/nmk

alpine: clean dependency
	$(BUILDSTRIP) -o ./build/nmk-alpine-amd64  $(PROJECT_DIR)/cmd/nmk

