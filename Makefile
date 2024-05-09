clean:
	# Execute the `clean` target in the `library` directory
	$(MAKE) -C library clean
	# Execute the `clean` target in the `wasm` directory
	$(MAKE) -C wasm clean
	# Execute the `clean` target in the `rest` directory
	$(MAKE) -C rest clean

test:
	# Execute the `test` target in the `library` directory
	$(MAKE) -C library test
	# Execute the `test` target in the `wasm` directory
	$(MAKE) -C wasm test
	# Execute the `test` target in the `rest` directory
	$(MAKE) -C rest test

build:
	# Execute the `build` target in the `library` directory
	$(MAKE) -C library build
	# Execute the `build` target in the `wasm` directory
	$(MAKE) -C wasm build
	# Execute the `build` target in the `rest` directory
	$(MAKE) -C rest build

lint:
	# Execute the `lint` target in the `library` directory
	$(MAKE) -C library lint
	# Execute the `lint` target in the `wasm` directory
	$(MAKE) -C wasm lint
	# Execute the `lint` target in the `rest` directory
	$(MAKE) -C rest lint

run_wasm:
	# Execute the `run` target in the `wasm` directory
	$(MAKE) -C wasm run

run_rest:
	# Execute the `run` target in the `rest` directory
	$(MAKE) -C rest run
