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
	GOARCH=amd64 GOOS=darwin  $(BUILDSTRIP) -o ./build/nmk.go-amd64-darwin  $(PROJECT_DIR)/cmd/nmk
	GOARCH=amd64 GOOS=freebsd $(BUILDSTRIP) -o ./build/nmk.go-amd64-freebsd $(PROJECT_DIR)/cmd/nmk
	GOARCH=amd64 GOOS=linux   $(BUILDSTRIP) -o ./build/nmk.go-amd64-linux   $(PROJECT_DIR)/cmd/nmk
	GOARCH=arm GOOS=linux     $(BUILDSTRIP) -o ./build/nmk.go-arm-linux     $(PROJECT_DIR)/cmd/nmk
	GOARCH=arm64 GOOS=linux   $(BUILDSTRIP) -o ./build/nmk.go-arm64-linux   $(PROJECT_DIR)/cmd/nmk

alpine: clean dependency
	$(BUILDSTRIP) -o ./build/nmk.go-amd64-linux-musl  $(PROJECT_DIR)/cmd/nmk

