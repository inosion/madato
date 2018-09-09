npm_pkg_dir   = target/pkg
wasm_dir      = $(npm_pkg_dir)
wasm_file    = ../target/wasm32-unknown-unknown/release/madato_wasm.wasm


$(wasm_dir):
	-mkdir -p $(wasm_dir)

$(npm_pkg_dir):
	-mkdir -p $(npm_pkg_dir)

$(wasm_debug):
	cargo build --release --target wasm32-unknown-unknown

$(wasm_file): $(wasm_debug) $(wasm_dir)
	# wasm-pack init .. can't use web-pack (https://github.com/rustwasm/wasm-pack/issues/252)
	wasm-bindgen $(wasm_file) --out-dir $(wasm_dir)

$(npm_pkg_dir)/package.json: $(npm_pkg_dir)
	cp package.json $(npm_pkg_dir)/package.json

test: $(wasm_compiled) $(npm_pkg_dir)/package.json
	cp -av spec $(npm_pkg_dir)
	cd $(npm_pkg_dir) && \
	npm install --save-dev && \
	npm --experimental-modules test


all: $(wasm_file) test

clean:
	rm -f $(wasm_file)

.PHONY: all
