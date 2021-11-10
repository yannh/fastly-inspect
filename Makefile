#!/usr/bin/make -f

.PHONY: build build-wasm build-binary output

build: build-binary build-wasm

build-wasm:
	wasm-pack build --target web

build-binary:
	cargo build --release

site:
	mkdir -p output/lib
	cp index.html output/
	cp pkg/fastly_inspect_bg.wasm pkg/fastly_inspect.js  output/lib
