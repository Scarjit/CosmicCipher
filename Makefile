clean:
	# Execute the `clean` target in the `library` directory
	$(MAKE) -C library clean
	# Execute the `clean` target in the `wasm` directory
	$(MAKE) -C wasm clean

test:
	# Execute the `test` target in the `library` directory
	$(MAKE) -C library test
	# Execute the `test` target in the `wasm` directory
	$(MAKE) -C wasm test

build:
	# Execute the `build` target in the `library` directory
	$(MAKE) -C library build
	# Execute the `build` target in the `wasm` directory
	$(MAKE) -C wasm build

lint:
	# Execute the `lint` target in the `library` directory
	$(MAKE) -C library lint
	# Execute the `lint` target in the `wasm` directory
	$(MAKE) -C wasm lint

run:
	# Execute the `run` target in the `wasm` directory
	$(MAKE) -C wasm run
