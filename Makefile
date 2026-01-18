.PHONY: all tests pytests

all:
	cargo build --release

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
	cargo test --package libmwemu --verbose
	cargo test --release --package libmwemu --verbose

pytests:
	cd crates/pymwemu && ./test_all.sh

test_x86:
	cargo test --release --features rax_x86_tests rax_x86_tests
