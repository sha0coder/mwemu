all:
	cargo build --release
tests:
	wget https://github.com/sha0coder/mwemu/releases/download/maps/test.zip
	unzip -o -P mwemuTestSystem test.zip
	rm test.zip
	cargo test --release --verbose

