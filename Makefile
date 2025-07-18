all:
	cargo build --release
tests:
	wget https://github.com/sha0coder/mwemu/releases/download/maps/test.zip
	unzip -o -P mwemuTestSystem test.zip
	rm test.zip
	cargo test --release --package libmwemu --verbose
pytests:
	cargo test --release --package pymwemu --verbose
pytests2:
	cd pymwemu && ./test_all.sh

