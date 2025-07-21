.PHONY: all tests pytests

all:
	cargo build --release
tests:
	wget -q https://github.com/sha0coder/mwemu/releases/download/maps/test.zip
	unzip -o -P mwemuTestSystem test.zip
	rm test.zip
	cargo test --release --package libmwemu --verbose
pytests:
	cd pymwemu && ./test_all.sh

