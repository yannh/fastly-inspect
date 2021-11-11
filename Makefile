#!/usr/bin/make -f

.PHONY: build build-wasm build-binary site

build: build-binary build-wasm

build-wasm:
	wasm-pack build --target web

build-binary:
	cargo build --release

site:
	cp index.html fastly-pages/static/
	cp index.js pkg/fastly_inspect_bg.wasm pkg/fastly_inspect.js  fastly-pages/static/lib/
