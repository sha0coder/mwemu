.PHONY: all tests pytests

# Detect Apple Silicon and set cross-compile target
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)
ifeq ($(UNAME_S)$(UNAME_M),Darwinarm64)
  CARGO_TARGET := --target x86_64-apple-darwin
else
  CARGO_TARGET :=
endif

all:
	cargo build --release $(CARGO_TARGET)

tests:
	if [ ! -d test ]; then \
		if which wget; then \
			wget -q https://github.com/sha0coder/mwemu/releases/download/maps/test.zip; \
		else \
			curl -L -O https://github.com/sha0coder/mwemu/releases/download/maps/test.zip; \
		fi; \
		unzip -o -P mwemuTestSystem test.zip; \
		rm test.zip; \
	fi
	cargo test --package libmwemu --verbose $(CARGO_TARGET)
	cargo test --release --package libmwemu --verbose $(CARGO_TARGET)

pytests:
	cd crates/pymwemu && ./test_all.sh

test_x86:
	cargo test --release --features rax_x86_tests rax_x86_tests

test_loader:
	cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt --init
