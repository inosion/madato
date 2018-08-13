wasm_compiled = test/www-sample/madatojs.wasm test/www-sample/madatojs.d.ts test/www-sample/madatojs.js
wasm_debug    = target/wasm32-unknown-unknown/debug/madatojs.wasm

$(wasm_debug):
	cargo +nightly build --target wasm32-unknown-unknown

$(wasm_compiled): $(wasm_debug)
	wasm-bindgen $(wasm_debug) --out-dir test/www-sample

all: $(wasm_compiled)

clean:
	rm -f $(wasm_compiled) $(wasm_debug)

.PHONY: all
