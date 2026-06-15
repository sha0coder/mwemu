.PHONY: all tests pytests sloppy maps

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

# Pre-fetch the genuine x64 Windows system DLLs from Microsoft's symbol server
# into maps/windows/x86_64/ (instead of needing a local copy or a GitHub
# release). This is optional warm-up: libmwemu also auto-fetches any missing DLL
# on demand, so forgetting `make maps` just makes the first run slower, not
# broken. Idempotent: skips if the maps are already present.
# (32-bit / maps/windows/x86 is not covered yet — winver is AMD64-only.)
maps: all
	@mkdir -p maps/windows/x86_64
	@if [ -f maps/windows/x86_64/kernelbase.dll ]; then \
		echo "[maps] x64 system DLLs already present"; \
	elif [ -f test/exe64win_msgbox.bin ]; then \
		echo "[maps] pre-fetching x64 system DLLs from the symbol server..."; \
		./target/release/mwemu -f test/exe64win_msgbox.bin -6 --maps maps/windows/x86_64/ -e 300000 --banzai >/dev/null 2>&1 || true; \
		echo "[maps] warmed $$(ls maps/windows/x86_64/*.dll 2>/dev/null | wc -l) DLLs into maps/windows/x86_64/ (more are auto-fetched on demand)"; \
	else \
		echo "[maps] need a sample PE to warm the cache; run 'make tests' once (downloads test/) then 'make maps'"; \
	fi

sloppy:
	-python3 scripts/sloppy.py

tests: sloppy
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

test_syscall:
	cargo run --release -- -f test/exe64win_msgbox.bin -6 --syscall-mode --winver win11
test_linux:
	cargo run --release -- -f /bin/ls -A '"-l"' -6
test_windows:
	cargo run --release -- -f test/exe64win_enigma.bin -6 --winver win11 -v
test_inception:
	cargo run --release -- -f target/release/mwemu -6 -v

#test_bench:
#	time cargo run --release -- -f test/exe64win_enigma.bin -6 --winver win11 -c 10000000 --cmd q
