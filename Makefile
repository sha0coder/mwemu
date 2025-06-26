all:
	cargo build --release
tests:
	unzip -P mwemuTestSamples test.zip
	cargo test --release

