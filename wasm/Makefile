clean:
	cargo clean
	rm -rf ./pkg

test:
	cargo test

build:
	cargo build
	wasm-pack build

lint:
	cargo clippy -- -D warnings

run:
	cd www && npm run start
